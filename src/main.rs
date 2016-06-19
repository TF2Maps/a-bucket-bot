extern crate boiler;
extern crate boiler_generated;
extern crate byteorder;
extern crate crc;
#[macro_use] extern crate log;
extern crate log4rs;
extern crate toml;

mod boiler_dispatch;
mod boiler_encryption;

use std::io::{Cursor, Read};
use std::cell::RefCell;
use boiler::{SteamConnection, EMsg, EPersonaState, Message, MessageHeader, MsgHdrProtoBuf, ExtendedClientMsgHdr};
use boiler_generated::ProtoMessage;
use boiler_generated::steammessages_base::CMsgProtoBufHeader;
use boiler_generated::steammessages_clientserver::{CMsgClientLogonResponse, CMsgClientChangeStatus, CMsgClientLoggedOff};
use byteorder::{ReadBytesExt, WriteBytesExt, LittleEndian};
use boiler_dispatch::MessageDispatcher;

fn on_logoff(message: Message) {
    let mut data = CMsgClientLoggedOff::new();
    data.merge_from_bytes(&message.body).unwrap();
    panic!("Logged off with EResult: {:?}", data.get_eresult());
}

fn on_chat_msg(message: Message) {
    trace!("Deserializing chat message...");

    let mut body_c = Cursor::new(message.body);

    // First get the message's body
    let msg_id_chatter = body_c.read_u64::<LittleEndian>().unwrap();
    let msg_id_chatroom = body_c.read_u64::<LittleEndian>().unwrap();
    let _msg_type = body_c.read_u32::<LittleEndian>().unwrap();

    // Then the remaining is payload
    let mut msg_data = String::new();
    body_c.read_to_string(&mut msg_data).unwrap();

    debug!("[{}] {}: {}", msg_id_chatroom, msg_id_chatter, msg_data);
}

fn on_client_logon_response(message: Message, connection: &RefCell<SteamConnection>) {
    let session_id;
    let steam_id;

    // Keep track of the session id for all future messages
    if let &MessageHeader::MsgHdrProtoBuf(ref header) = &message.header {
        session_id = header.proto.get_client_sessionid();
        steam_id = header.proto.get_steamid();
    } else {
        panic!("Unexpected header for this message");
    }

    // Parse in the response
    let mut response = CMsgClientLogonResponse::new();
    response.merge_from_bytes(&message.body).unwrap();

    if response.get_eresult() != 1 {
        panic!("Failed to LogOn, EResult: {}", response.get_eresult());
    }

    // We now know our login succeeded
    info!("Successfully logged in with SteamID {} and SessionID {}", steam_id, session_id);

    // Start up the heartbeat so we don't get disconnected
    let interval = response.get_out_of_game_heartbeat_seconds();
    connection.borrow_mut().start_heartbeat(interval, session_id);

    // Set ourselves to online
    let mut body = CMsgClientChangeStatus::new();
    body.set_persona_state(EPersonaState::Online as u32);
    body.set_player_name("A Bucket Bot is Watching".into());
    let mut hdr_proto = CMsgProtoBufHeader::new();
    hdr_proto.set_jobid_source(1); // TODO: Auto-assign
    hdr_proto.set_client_sessionid(session_id);
    hdr_proto.set_steamid(steam_id);
    let header = MsgHdrProtoBuf {
        msg: EMsg::ClientChangeStatus,
        proto: hdr_proto,
    };
    let message = Message {
        header: MessageHeader::MsgHdrProtoBuf(header),
        body: body.write_to_bytes().unwrap()
    };
    connection.borrow_mut().send(message);

    // Massage the SteamID for TF2Maps until we've got what we need
    let mut tf2maps_id = SteamId { value: 103582791429594873 };
    tf2maps_id.set_account_instance(CHAT_INSTANCE_FLAG_CLAN);
    tf2maps_id.set_account_type(8); // Account type: chat

    // Connect to the group chat
    // TODO: Add limited account warning
    let mut body = Vec::new();
    body.write_u64::<LittleEndian>(tf2maps_id.value).unwrap(); // SteamID
    body.write_u8(0).unwrap(); // IsVoiceSpeaker
    let header = ExtendedClientMsgHdr {
        msg: EMsg::ClientJoinChat,
        header_size: 36,
        header_version: 2,
        target_job_id: 0xffffffffffffffff,
        source_job_id: 0xffffffffffffffff,
        header_canary: 239,
        steam_id: steam_id,
        session_id: session_id,
    };
    let message = Message {
        header: MessageHeader::ExtendedClientMsgHdr(header),
        body: body
    };
    connection.borrow_mut().send(message);
}

fn main() {
    // Set up logging
    log4rs::init_file("./config/Log4rs.toml", Default::default()).unwrap();

    // Start the client
    let server_addr = "162.254.196.43:27018".parse().unwrap();
    let connection = RefCell::new(SteamConnection::connect(server_addr));

    // Set up the dispatch
    let mut dispatch = MessageDispatcher::new();
    boiler_encryption::register_for(&mut dispatch, &connection);
    dispatch.register(EMsg::ClientLoggedOff, on_logoff);
    dispatch.register(EMsg::ClientChatMsg, on_chat_msg);
    dispatch.register(EMsg::ClientLogOnResponse, |msg| on_client_logon_response(msg, &connection));

    dispatch.register_fallback(Box::new(|msg|
        debug!("Unhandled message type {:?}", msg.header.emsg())
    ));

    // Loop over messages that get sent to us
    loop {
        // Get a message
        let message = connection.borrow_mut().recv();

        // Handle what message we got
        dispatch.handle(message);
    }
}

const ACCOUNT_INSTANCE_MASK: u32 = 0x000FFFFF;
const CHAT_INSTANCE_FLAG_CLAN: u32 = ( ACCOUNT_INSTANCE_MASK + 1 ) >> 1;

struct SteamId {
    value: u64
}

impl SteamId {
    /*fn get(&self, offset: usize, mask: u64) -> u64 {
        (self.value >> offset) & mask
    }*/

    fn set(&mut self, offset: usize, mask: u64, new_value: u64) {
        self.value = (self.value & !(mask << offset)) | ((new_value & mask) << offset);
    }

    fn set_account_instance(&mut self, value: u32) {
        self.set(32, 0xFFFFF, value as u64);
    }

    fn set_account_type(&mut self, value: u32) {
        self.set(52, 0xF, value as u64);
    }
}
