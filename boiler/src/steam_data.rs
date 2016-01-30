//! TODO: Auto-generate

use std::io::Cursor;
use byteorder::{ReadBytesExt, LittleEndian};
use num::FromPrimitive;

const PROTO_MASK: u32 = 0x80000000;

enum_from_primitive! {
    #[derive(Debug, PartialEq)]
    pub enum EMsg {
        ChannelEncryptRequest = 1303,
    	ChannelEncryptResponse = 1304,
    	ChannelEncryptResult = 1305,
    }
}

impl EMsg {
    pub fn parse(data: &mut Cursor<&Vec<u8>>) -> Self {
        let raw = Self::parse_to_raw(data);
        Self::from_raw(raw)
    }

    pub fn parse_to_raw(data: &mut Cursor<&Vec<u8>>) -> u32 {
        data.read_u32::<LittleEndian>().unwrap()
    }

    pub fn from_raw(raw: u32) -> Self {
        EMsg::from_u32(raw & !PROTO_MASK).unwrap()
    }
}

#[derive(Debug)]
pub struct MsgHdr {
    pub msg: EMsg,
    pub target_job_id: u64,
    pub source_job_id: u64
}

impl MsgHdr {
    pub fn parse(data: &mut Cursor<&Vec<u8>>) -> Self {
        debug!("Decoding MsgHdr type header");

        MsgHdr {
            msg: EMsg::parse(data),
            target_job_id: data.read_u64::<LittleEndian>().unwrap(),
            source_job_id: data.read_u64::<LittleEndian>().unwrap()
        }
    }
}

#[derive(Debug)]
pub struct MsgHdrProtoBuf;

impl MsgHdrProtoBuf {
    pub fn parse(_data: &mut Cursor<&Vec<u8>>) -> Self {
        debug!("Decoding MsgHdrProtoBuf type header");

        // TODO: Implement, the following is from node-steam-client
        //header = Schema.MsgHdrProtoBuf.decode(data);
        //header.proto = Steam._processProto(header.proto);
        //if (!this._sessionID && header.headerLength > 0) {
        //	this._sessionID = header.proto.client_sessionid;
        //	this.steamID = header.proto.steamid;
        //}
        //sourceJobID = header.proto.jobid_source;
        //targetJobID = header.proto.jobid_target;

        unimplemented!();
    }
}

#[derive(Debug)]
pub struct ExtendedClientMsgHdr;

impl ExtendedClientMsgHdr {
    pub fn parse(_data: &mut Cursor<&Vec<u8>>) -> Self {
        debug!("Decoding ExtendedClientMsgHdr type header");

        // TODO: Implement, the following is from node-steam-client
        //header = Schema.ExtendedClientMsgHdr.decode(data);
        //sourceJobID = header.sourceJobID;
        //targetJobID = header.targetJobID;

        unimplemented!();
    }
}

#[derive(Debug)]
pub enum MessageHeader {
    MsgHdr(MsgHdr),
    MsgHdrProtoBuf(MsgHdrProtoBuf),
    ExtendedClientMsgHdr(ExtendedClientMsgHdr)
}

impl MessageHeader {
    pub fn parse(data: &mut Cursor<&Vec<u8>>) -> Self {
        assert!(data.position() == 0, "`data` position must be 0 when parsing a header");

        // Get the event type we received
        let emsg_raw = EMsg::parse_to_raw(data);
        let emsg = EMsg::from_raw(emsg_raw);
        debug!("Parsing message for EMsg: {:?}", emsg);

        // Reset the cursor again, the header needs it to be at the start
        data.set_position(0);

        // Handle the header of the message, which can be different things
        if emsg == EMsg::ChannelEncryptRequest ||
           emsg == EMsg::ChannelEncryptResult {
            MessageHeader::MsgHdr(MsgHdr::parse(data))
        }
        else if (emsg_raw & PROTO_MASK) != 0 {
            MessageHeader::MsgHdrProtoBuf(MsgHdrProtoBuf::parse(data))
        }
        else {
            MessageHeader::ExtendedClientMsgHdr(ExtendedClientMsgHdr::parse(data))
        }
    }
}
