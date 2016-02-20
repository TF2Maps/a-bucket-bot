extern crate boiler;
extern crate boiler_generated;
extern crate byteorder;
extern crate crc;
#[macro_use] extern crate log;
extern crate log4rs;
extern crate toml;

use std::io::{Cursor, Read, Write};
use std::fs::File;
use boiler::{SteamConnection, EMsg, EPersonaState, Message, MessageHeader, MsgHdr, MsgHdrProtoBuf};
use boiler_generated::ProtoMessage;
use boiler_generated::steammessages_base::CMsgProtoBufHeader;
use boiler_generated::steammessages_clientserver::{CMsgClientLogon, CMsgClientLogonResponse, CMsgClientChangeStatus, CMsgClientLoggedOff};
use byteorder::{ReadBytesExt, WriteBytesExt, LittleEndian};
use crc::{crc32, Hasher32};
use toml::Parser;

fn main() {
    // Set up logging
    log4rs::init_file("./config/Log4rs.toml", Default::default()).unwrap();

    // Start the client
    let mut client = SteamConnection::connect();
    let mut encryption_key = None;
    let mut session_id;

    // Loop over messages that get sent to us
    loop {
        // Get a message
        let message = client.recv();

        // Handle what message we got
        match message.header.emsg()  {
            EMsg::ChannelEncryptRequest => {
                // We got asked for encryption, handle that
                debug!("Building encryption request response...");

                // TODO: Verify protocol version and universe

                // Generate the keys
                let key = boiler::crypto::generate_key();
                let encrypted_key = boiler::crypto::encrypt_key(&key);
                let crc = crc32::checksum_ieee(&encrypted_key);

                // Store the key for later
                encryption_key = Some(key);

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
                client.send(message);
            },
            EMsg::ChannelEncryptResult => {
                debug!("Completing encryption handshake...");

                // See if it succeeded first
                let result = Cursor::new(message.body).read_u32::<LittleEndian>().unwrap();
                if result != 1 {
                    panic!("Encryption failed, EResult: {}", result);
                }
                trace!("Encryption succeeded!");

                // Now notify the connection that all messages from now on will use encryption
                client.set_encryption_key(encryption_key.take().unwrap());

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
                client.send(message);
            },
            EMsg::ClientLogOnResponse => {
                // Keep track of the session id for all future messages
                if let &MessageHeader::MsgHdrProtoBuf(ref header) = &message.header {
                    session_id = header.proto.get_client_sessionid();
                } else {
                    panic!("Unexpected header for this message");
                }

                // Parse in the response
                let mut response = CMsgClientLogonResponse::new();
                response.merge_from_bytes(&message.body).unwrap();

                if response.get_eresult() != 1 {
                    panic!("Failed to LogOn");
                }

                // We now know our login succeeded
                debug!("LogOn success received");

                // Start up the heartbeat so we don't get disconnected
                let interval = response.get_out_of_game_heartbeat_seconds();
                client.start_heartbeat(interval, session_id);

                // Set ourselves to online
                let mut body = CMsgClientChangeStatus::new();
                body.set_persona_state(EPersonaState::Online as u32);
                body.set_player_name("A Bucket2".into());
                let mut hdr_proto = CMsgProtoBufHeader::new();
                hdr_proto.set_jobid_source(1); // TODO: Auto-assign
                hdr_proto.set_client_sessionid(session_id);
                hdr_proto.set_steamid(76561197960265728);
                let header = MsgHdrProtoBuf {
                    msg: EMsg::ClientChangeStatus,
                    proto: hdr_proto,
                };
                let message = Message {
                    header: MessageHeader::MsgHdrProtoBuf(header),
                    body: body.write_to_bytes().unwrap()
                };
                client.send(message);
            },
            EMsg::ClientLoggedOff => {
                let mut data = CMsgClientLoggedOff::new();
                data.merge_from_bytes(&message.body).unwrap();
                debug!("Logged off with EResult: {:?}", data.get_eresult());
            }
            msg => { debug!("Received unknown message type {:?}", msg); }
        }
    }

    //client.wait_close();
}
