//! Experimental boiler-dispatch module for managing the encryption handshake

use std::io::{Cursor, Read, Write};
use std::rc::Rc;
use std::cell::RefCell;
use std::fs::File;
use byteorder::{ReadBytesExt, WriteBytesExt, LittleEndian};
use boiler::{self, SteamConnection, Message, EMsg, MsgHdr, MessageHeader, MsgHdrProtoBuf};
use boiler_generated::ProtoMessage;
use boiler_generated::steammessages_base::CMsgProtoBufHeader;
use boiler_generated::steammessages_clientserver::CMsgClientLogon;
use boiler_dispatch::MessageDispatcher;
use crc::crc32;
use toml::Parser;

pub fn register_for<'a>(dispatcher: &mut MessageDispatcher<'a>, connection: &'a RefCell<SteamConnection>) {
    let state = Rc::new(RefCell::new(BoilerEncryption {
        key: None
    }));

    let state_c = state.clone();
    dispatcher.register(EMsg::ChannelEncryptRequest, move |_| {
        state_c.borrow_mut().on_channel_encrypt_request(connection);
    });

    let state_c = state.clone();
    dispatcher.register(EMsg::ChannelEncryptResult, move |msg| {
        state_c.borrow_mut().on_channel_encrypt_result(msg, connection);
    });
}

struct BoilerEncryption {
    key: Option<Vec<u8>>
}

impl BoilerEncryption {
    fn on_channel_encrypt_request(&mut self, connection: &RefCell<SteamConnection>) {
        // We got asked for encryption, handle that
        debug!("Building encryption request response...");

        // TODO: Verify protocol version and universe

        // Generate the keys
        let key = boiler::crypto::generate_key();
        let encrypted_key = boiler::crypto::encrypt_key(&key);
        let crc = crc32::checksum_ieee(&encrypted_key);

        // Store the key for later
        self.key = Some(key);

        // Push them into a buffer to send
        let mut body = Cursor::new(Vec::new());
        body.write_u32::<LittleEndian>(1).unwrap(); // Protocol version
        body.write_u32::<LittleEndian>(encrypted_key.len() as u32).unwrap(); // Encrypted key size in bytes

        // This is the actual body
        body.write(&encrypted_key).unwrap(); // The actual encrypted key
        body.write_u32::<LittleEndian>(crc).unwrap(); // Key checksum
        body.write_u32::<LittleEndian>(0).unwrap(); // Trailer (TODO: no idea what this is)

        // Build and send the message
        let header = MsgHdr {
            msg: EMsg::ChannelEncryptResponse,
            target_job_id: 0xffffffffffffffff,
            source_job_id: 0xffffffffffffffff
        };
        let message = Message {
            header: MessageHeader::MsgHdr(header),
            body: body.into_inner()
        };
        connection.borrow_mut().send(message);
    }

    fn on_channel_encrypt_result(&mut self, message: Message, connection: &RefCell<SteamConnection>) {
        debug!("Completing encryption handshake...");

        // See if it succeeded first
        let result = Cursor::new(message.body).read_u32::<LittleEndian>().unwrap();
        if result != 1 {
            panic!("Encryption failed, EResult: {}", result);
        }
        trace!("Encryption succeeded!");

        // Now notify the connection that all messages from now on will use encryption
        let key = self.key.take().unwrap();
        connection.borrow_mut().set_encryption_key(key);

        // TODO: Perhaps make the following use an event channel instead
        // We are now connected, send a login request
        debug!("Logging in...");

        // Read in the config file
        let mut file = File::open("./config/Steam.toml")
            .expect("Unable to open /config/Steam.toml, create one and configure an account.");
        let mut toml = String::new();
        file.read_to_string(&mut toml).unwrap();
        let conf = Parser::new(&toml).parse().unwrap();
        let name = conf["name"].as_str().unwrap();
        let password = conf["password"].as_str().unwrap();

        // Build the body
        let mut body = CMsgClientLogon::new();
        body.set_account_name(name.into());
        body.set_password(password.into());
        body.set_protocol_version(65579);

        // Build and send the message
        let mut hdr_proto = CMsgProtoBufHeader::new();
        hdr_proto.set_client_sessionid(0);
        hdr_proto.set_steamid(76561197960265728);
        let header = MsgHdrProtoBuf {
            msg: EMsg::ClientLogon,
            proto: hdr_proto,
        };
        let message = Message {
            header: MessageHeader::MsgHdrProtoBuf(header),
            body: body.write_to_bytes().unwrap()
        };
        connection.borrow_mut().send(message);
    }
}
