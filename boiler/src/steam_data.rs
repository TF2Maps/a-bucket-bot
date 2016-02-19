//! TODO: Auto-generate

use std::io::{Cursor, Read, Write};
use byteorder::{WriteBytesExt, ReadBytesExt, LittleEndian};
use num::FromPrimitive;
use boiler_generated::ProtoMessage;
use boiler_generated::steammessages_base::CMsgProtoBufHeader;

const PROTO_MASK: u32 = 0x80000000;

enum_from_primitive! {
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub enum EMsg {
        Invalid = 0,
        Multi = 1,
        GenericReply = 100,
        DestJobFailed = 113,
        Alert = 115,
        ScidRequest = 120,
        ScidResponse = 121,
        JobHeartbeat = 123,
        HubConnect = 124,
        Subscribe = 126,
        RouteMessage = 127,
        RemoteSysID = 128,
        AmCreateAccountResponse = 129,
        WGRequest = 130,
        WGResponse = 131,
        KeepAlive = 132,
        WebApiJobRequest = 133,
        WebApiJobResponse = 134,
        ClientSessionStart = 135,
        ClientSessionEnd = 136,
        ClientSessionUpdateAuthTicket = 137,
        ClientLogOnResponse = 751,
		ClientVACChallenge = 753,
		ClientSetHeartbeatRate = 755,
        ChannelEncryptRequest = 1303,
    	ChannelEncryptResponse = 1304,
    	ChannelEncryptResult = 1305,
        ClientLogon = 5514,
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
        let protoless_raw = raw & !PROTO_MASK;
        EMsg::from_u32(protoless_raw)
            .expect(&format!("Can't parse EMsg {}, unknown!", protoless_raw))
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
        trace!("Decoding MsgHdr type header");

        MsgHdr {
            msg: EMsg::parse(data),
            target_job_id: data.read_u64::<LittleEndian>().unwrap(),
            source_job_id: data.read_u64::<LittleEndian>().unwrap()
        }
    }

    pub fn write_to(&self, data: &mut Cursor<Vec<u8>>) {
        data.write_u32::<LittleEndian>(self.msg as u32).unwrap();
        data.write_u64::<LittleEndian>(self.target_job_id).unwrap();
        data.write_u64::<LittleEndian>(self.source_job_id).unwrap();
    }
}

#[derive(Debug)]
pub struct MsgHdrProtoBuf {
    pub msg: EMsg,
    pub proto: CMsgProtoBufHeader,
}

impl MsgHdrProtoBuf {
    pub fn parse(data: &mut Cursor<&Vec<u8>>) -> Self {
        trace!("Decoding MsgHdrProtoBuf type header");

        // Read in data
        let msg = EMsg::parse(data);
        let len = data.read_u32::<LittleEndian>().unwrap() as usize;
        let mut bytes = vec![0u8; len];
        data.read(&mut bytes).unwrap();

        // Convert protobuf data to the header type
        let mut proto = CMsgProtoBufHeader::new();
        proto.merge_from_bytes(&bytes).unwrap();

        MsgHdrProtoBuf {
            msg: msg,
            proto: proto
        }
    }


    pub fn write_to(&self, data: &mut Cursor<Vec<u8>>) {
        // Turn the protobuf data into bytes
        let bytes = self.proto.write_to_bytes().unwrap();

        // Flag the msg as protobuf
        let msg = self.msg as u32 | PROTO_MASK;

        // Actually write in the header data
        data.write_u32::<LittleEndian>(msg).unwrap();
        data.write_u32::<LittleEndian>(bytes.len() as u32).unwrap();
        data.write(&bytes).unwrap();
    }
}

#[derive(Debug)]
pub struct ExtendedClientMsgHdr;

impl ExtendedClientMsgHdr {
    pub fn parse(_data: &mut Cursor<&Vec<u8>>) -> Self {
        trace!("Decoding ExtendedClientMsgHdr type header");

        // TODO: Implement, the following is from node-steam-client
        //header = Schema.ExtendedClientMsgHdr.decode(data);
        //sourceJobID = header.sourceJobID;
        //targetJobID = header.targetJobID;

        unimplemented!();
    }
}

/// A header of a message to be sent to or received from a server. Can be one of three header types.
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

    pub fn write_to(&self, data: &mut Cursor<Vec<u8>>) {
        match *self {
            MessageHeader::MsgHdr(ref h) => h.write_to(data),
            MessageHeader::MsgHdrProtoBuf(ref h) => h.write_to(data),
            MessageHeader::ExtendedClientMsgHdr(_) => unimplemented!(),
        }
    }

    /// Gets the EMsg of the inner header type.
    pub fn emsg(&self) -> EMsg {
        match *self {
            MessageHeader::MsgHdr(ref h) => h.msg,
            MessageHeader::MsgHdrProtoBuf(ref h) => h.msg,
            MessageHeader::ExtendedClientMsgHdr(_) => unimplemented!(),
        }
    }
}

/// A message to be sent to or received from a steam server.
#[derive(Debug)]
pub struct Message {
    pub header: MessageHeader,
    pub body: Vec<u8>
}

impl Message {
    pub fn parse(data: &mut Cursor<&Vec<u8>>) -> Self {
        // Parse in the header
        let header = MessageHeader::parse(data);

        // Get the remaining data
        let mut body = Vec::new();
        data.read_to_end(&mut body).unwrap();

        // Create the actual message
        Message {
            header: header,
            body: body
        }
    }

    pub fn write_to(&self, data: &mut Cursor<Vec<u8>>) {
        self.header.write_to(data);
        data.write(&self.body).unwrap();
    }

    pub fn into_bytes(&self) -> Vec<u8> {
        let mut data_c = Cursor::new(Vec::new());
        self.write_to(&mut data_c);
        data_c.into_inner()
    }
}
