extern crate boiler;
extern crate byteorder;
extern crate crc;
#[macro_use] extern crate log;
extern crate log4rs;

use std::io::{Cursor, Write};
use boiler::{SteamConnection, EMsg, Message, MessageHeader, MsgHdr};
use byteorder::{ReadBytesExt, WriteBytesExt, LittleEndian};
use crc::{crc32, Hasher32};

fn main() {
    // Set up logging
    log4rs::init_file("./config/Log4rs.toml", Default::default()).unwrap();

    // Start the client
    let mut client = SteamConnection::connect();
    let mut encryption_key = None;

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
                let body: Vec<u8> = Vec::new();
                let mut body_c = Cursor::new(body);
                body_c.write_u32::<LittleEndian>(1).unwrap(); // Protocol version
                body_c.write_u32::<LittleEndian>(encrypted_key.len() as u32).unwrap(); // Encrypted key size in bytes

                // This is the actual body
                body_c.write(&encrypted_key).unwrap(); // The actual encrypted key
                body_c.write_u32::<LittleEndian>(crc).unwrap(); // Key checksum
                body_c.write_u32::<LittleEndian>(0).unwrap(); // Trailer (TODO: no idea what this is)

                // Build and send the message
                let header = MsgHdr {
                    msg: EMsg::ChannelEncryptResponse,
                    target_job_id: 0xffffffffffffffff,
                    source_job_id: 0xffffffffffffffff
                };
                let message = Message {
                    header: MessageHeader::MsgHdr(header),
                    body: body_c.into_inner()
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

                // TODO: We are now connected, send a login request
            }
            msg => { debug!("Received unknown message type {:?}", msg); }
        }
    }

    //client.wait_close();
}
