// This file is generated. Do not edit
// @generated

#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_imports)]

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

#[derive(Clone,Default)]
pub struct EncryptedAppTicket {
    // message fields
    ticket_version_no: ::std::option::Option<u32>,
    crc_encryptedticket: ::std::option::Option<u32>,
    cb_encrypteduserdata: ::std::option::Option<u32>,
    cb_encrypted_appownershipticket: ::std::option::Option<u32>,
    encrypted_ticket: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for EncryptedAppTicket {}

impl EncryptedAppTicket {
    pub fn new() -> EncryptedAppTicket {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static EncryptedAppTicket {
        static mut instance: ::protobuf::lazy::Lazy<EncryptedAppTicket> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const EncryptedAppTicket,
        };
        unsafe {
            instance.get(|| {
                EncryptedAppTicket {
                    ticket_version_no: ::std::option::Option::None,
                    crc_encryptedticket: ::std::option::Option::None,
                    cb_encrypteduserdata: ::std::option::Option::None,
                    cb_encrypted_appownershipticket: ::std::option::Option::None,
                    encrypted_ticket: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional uint32 ticket_version_no = 1;

    pub fn clear_ticket_version_no(&mut self) {
        self.ticket_version_no = ::std::option::Option::None;
    }

    pub fn has_ticket_version_no(&self) -> bool {
        self.ticket_version_no.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ticket_version_no(&mut self, v: u32) {
        self.ticket_version_no = ::std::option::Option::Some(v);
    }

    pub fn get_ticket_version_no<'a>(&self) -> u32 {
        self.ticket_version_no.unwrap_or(0)
    }

    // optional uint32 crc_encryptedticket = 2;

    pub fn clear_crc_encryptedticket(&mut self) {
        self.crc_encryptedticket = ::std::option::Option::None;
    }

    pub fn has_crc_encryptedticket(&self) -> bool {
        self.crc_encryptedticket.is_some()
    }

    // Param is passed by value, moved
    pub fn set_crc_encryptedticket(&mut self, v: u32) {
        self.crc_encryptedticket = ::std::option::Option::Some(v);
    }

    pub fn get_crc_encryptedticket<'a>(&self) -> u32 {
        self.crc_encryptedticket.unwrap_or(0)
    }

    // optional uint32 cb_encrypteduserdata = 3;

    pub fn clear_cb_encrypteduserdata(&mut self) {
        self.cb_encrypteduserdata = ::std::option::Option::None;
    }

    pub fn has_cb_encrypteduserdata(&self) -> bool {
        self.cb_encrypteduserdata.is_some()
    }

    // Param is passed by value, moved
    pub fn set_cb_encrypteduserdata(&mut self, v: u32) {
        self.cb_encrypteduserdata = ::std::option::Option::Some(v);
    }

    pub fn get_cb_encrypteduserdata<'a>(&self) -> u32 {
        self.cb_encrypteduserdata.unwrap_or(0)
    }

    // optional uint32 cb_encrypted_appownershipticket = 4;

    pub fn clear_cb_encrypted_appownershipticket(&mut self) {
        self.cb_encrypted_appownershipticket = ::std::option::Option::None;
    }

    pub fn has_cb_encrypted_appownershipticket(&self) -> bool {
        self.cb_encrypted_appownershipticket.is_some()
    }

    // Param is passed by value, moved
    pub fn set_cb_encrypted_appownershipticket(&mut self, v: u32) {
        self.cb_encrypted_appownershipticket = ::std::option::Option::Some(v);
    }

    pub fn get_cb_encrypted_appownershipticket<'a>(&self) -> u32 {
        self.cb_encrypted_appownershipticket.unwrap_or(0)
    }

    // optional bytes encrypted_ticket = 5;

    pub fn clear_encrypted_ticket(&mut self) {
        self.encrypted_ticket.clear();
    }

    pub fn has_encrypted_ticket(&self) -> bool {
        self.encrypted_ticket.is_some()
    }

    // Param is passed by value, moved
    pub fn set_encrypted_ticket(&mut self, v: ::std::vec::Vec<u8>) {
        self.encrypted_ticket = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_encrypted_ticket<'a>(&'a mut self) -> &'a mut ::std::vec::Vec<u8> {
        if self.encrypted_ticket.is_none() {
            self.encrypted_ticket.set_default();
        };
        self.encrypted_ticket.as_mut().unwrap()
    }

    // Take field
    pub fn take_encrypted_ticket(&mut self) -> ::std::vec::Vec<u8> {
        self.encrypted_ticket.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_encrypted_ticket<'a>(&'a self) -> &'a [u8] {
        match self.encrypted_ticket.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }
}

impl ::protobuf::Message for EncryptedAppTicket {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint32());
                    self.ticket_version_no = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint32());
                    self.crc_encryptedticket = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint32());
                    self.cb_encrypteduserdata = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint32());
                    self.cb_encrypted_appownershipticket = ::std::option::Option::Some(tmp);
                },
                5 => {
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.encrypted_ticket));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.ticket_version_no.iter() {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.crc_encryptedticket.iter() {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.cb_encrypteduserdata.iter() {
            my_size += ::protobuf::rt::value_size(3, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.cb_encrypted_appownershipticket.iter() {
            my_size += ::protobuf::rt::value_size(4, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.encrypted_ticket.iter() {
            my_size += ::protobuf::rt::bytes_size(5, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.ticket_version_no {
            try!(os.write_uint32(1, v));
        };
        if let Some(v) = self.crc_encryptedticket {
            try!(os.write_uint32(2, v));
        };
        if let Some(v) = self.cb_encrypteduserdata {
            try!(os.write_uint32(3, v));
        };
        if let Some(v) = self.cb_encrypted_appownershipticket {
            try!(os.write_uint32(4, v));
        };
        if let Some(v) = self.encrypted_ticket.as_ref() {
            try!(os.write_bytes(5, &v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<EncryptedAppTicket>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for EncryptedAppTicket {
    fn new() -> EncryptedAppTicket {
        EncryptedAppTicket::new()
    }

    fn descriptor_static(_: ::std::option::Option<EncryptedAppTicket>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "ticket_version_no",
                    EncryptedAppTicket::has_ticket_version_no,
                    EncryptedAppTicket::get_ticket_version_no,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "crc_encryptedticket",
                    EncryptedAppTicket::has_crc_encryptedticket,
                    EncryptedAppTicket::get_crc_encryptedticket,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "cb_encrypteduserdata",
                    EncryptedAppTicket::has_cb_encrypteduserdata,
                    EncryptedAppTicket::get_cb_encrypteduserdata,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "cb_encrypted_appownershipticket",
                    EncryptedAppTicket::has_cb_encrypted_appownershipticket,
                    EncryptedAppTicket::get_cb_encrypted_appownershipticket,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "encrypted_ticket",
                    EncryptedAppTicket::has_encrypted_ticket,
                    EncryptedAppTicket::get_encrypted_ticket,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<EncryptedAppTicket>(
                    "EncryptedAppTicket",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for EncryptedAppTicket {
    fn clear(&mut self) {
        self.clear_ticket_version_no();
        self.clear_crc_encryptedticket();
        self.clear_cb_encrypteduserdata();
        self.clear_cb_encrypted_appownershipticket();
        self.clear_encrypted_ticket();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for EncryptedAppTicket {
    fn eq(&self, other: &EncryptedAppTicket) -> bool {
        self.ticket_version_no == other.ticket_version_no &&
        self.crc_encryptedticket == other.crc_encryptedticket &&
        self.cb_encrypteduserdata == other.cb_encrypteduserdata &&
        self.cb_encrypted_appownershipticket == other.cb_encrypted_appownershipticket &&
        self.encrypted_ticket == other.encrypted_ticket &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for EncryptedAppTicket {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x1a, 0x65, 0x6e, 0x63, 0x72, 0x79, 0x70, 0x74, 0x65, 0x64, 0x5f, 0x61, 0x70, 0x70, 0x5f,
    0x74, 0x69, 0x63, 0x6b, 0x65, 0x74, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0xad, 0x01, 0x0a,
    0x12, 0x45, 0x6e, 0x63, 0x72, 0x79, 0x70, 0x74, 0x65, 0x64, 0x41, 0x70, 0x70, 0x54, 0x69, 0x63,
    0x6b, 0x65, 0x74, 0x12, 0x19, 0x0a, 0x11, 0x74, 0x69, 0x63, 0x6b, 0x65, 0x74, 0x5f, 0x76, 0x65,
    0x72, 0x73, 0x69, 0x6f, 0x6e, 0x5f, 0x6e, 0x6f, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0d, 0x12, 0x1b,
    0x0a, 0x13, 0x63, 0x72, 0x63, 0x5f, 0x65, 0x6e, 0x63, 0x72, 0x79, 0x70, 0x74, 0x65, 0x64, 0x74,
    0x69, 0x63, 0x6b, 0x65, 0x74, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0d, 0x12, 0x1c, 0x0a, 0x14, 0x63,
    0x62, 0x5f, 0x65, 0x6e, 0x63, 0x72, 0x79, 0x70, 0x74, 0x65, 0x64, 0x75, 0x73, 0x65, 0x72, 0x64,
    0x61, 0x74, 0x61, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0d, 0x12, 0x27, 0x0a, 0x1f, 0x63, 0x62, 0x5f,
    0x65, 0x6e, 0x63, 0x72, 0x79, 0x70, 0x74, 0x65, 0x64, 0x5f, 0x61, 0x70, 0x70, 0x6f, 0x77, 0x6e,
    0x65, 0x72, 0x73, 0x68, 0x69, 0x70, 0x74, 0x69, 0x63, 0x6b, 0x65, 0x74, 0x18, 0x04, 0x20, 0x01,
    0x28, 0x0d, 0x12, 0x18, 0x0a, 0x10, 0x65, 0x6e, 0x63, 0x72, 0x79, 0x70, 0x74, 0x65, 0x64, 0x5f,
    0x74, 0x69, 0x63, 0x6b, 0x65, 0x74, 0x18, 0x05, 0x20, 0x01, 0x28, 0x0c, 0x42, 0x05, 0x48, 0x01,
    0x80, 0x01, 0x00, 0x4a, 0x9d, 0x04, 0x0a, 0x06, 0x12, 0x04, 0x00, 0x00, 0x09, 0x01, 0x0a, 0x08,
    0x0a, 0x01, 0x08, 0x12, 0x03, 0x00, 0x00, 0x1c, 0x0a, 0x0b, 0x0a, 0x04, 0x08, 0xe7, 0x07, 0x00,
    0x12, 0x03, 0x00, 0x00, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x12, 0x03,
    0x00, 0x07, 0x13, 0x0a, 0x0d, 0x0a, 0x06, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x12, 0x03, 0x00,
    0x07, 0x13, 0x0a, 0x0e, 0x0a, 0x07, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x00,
    0x07, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x08, 0xe7, 0x07, 0x00, 0x03, 0x12, 0x03, 0x00, 0x16, 0x1b,
    0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x01, 0x00, 0x23, 0x0a, 0x0b, 0x0a, 0x04, 0x08, 0xe7,
    0x07, 0x01, 0x12, 0x03, 0x01, 0x00, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x08, 0xe7, 0x07, 0x01, 0x02,
    0x12, 0x03, 0x01, 0x07, 0x1a, 0x0a, 0x0d, 0x0a, 0x06, 0x08, 0xe7, 0x07, 0x01, 0x02, 0x00, 0x12,
    0x03, 0x01, 0x07, 0x1a, 0x0a, 0x0e, 0x0a, 0x07, 0x08, 0xe7, 0x07, 0x01, 0x02, 0x00, 0x01, 0x12,
    0x03, 0x01, 0x07, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x08, 0xe7, 0x07, 0x01, 0x03, 0x12, 0x03, 0x01,
    0x1d, 0x22, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x03, 0x00, 0x09, 0x01, 0x0a, 0x0a,
    0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x03, 0x08, 0x1a, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00,
    0x02, 0x00, 0x12, 0x03, 0x04, 0x08, 0x2e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x04,
    0x12, 0x03, 0x04, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x05, 0x12, 0x03,
    0x04, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x04, 0x18,
    0x29, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x04, 0x2c, 0x2d, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x05, 0x08, 0x30, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x01, 0x04, 0x12, 0x03, 0x05, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x01, 0x05, 0x12, 0x03, 0x05, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01,
    0x01, 0x12, 0x03, 0x05, 0x18, 0x2b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x03, 0x12,
    0x03, 0x05, 0x2e, 0x2f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x02, 0x12, 0x03, 0x06, 0x08,
    0x31, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x04, 0x12, 0x03, 0x06, 0x08, 0x10, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x05, 0x12, 0x03, 0x06, 0x11, 0x17, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x06, 0x18, 0x2c, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x02, 0x03, 0x12, 0x03, 0x06, 0x2f, 0x30, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02,
    0x03, 0x12, 0x03, 0x07, 0x08, 0x3c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x04, 0x12,
    0x03, 0x07, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x05, 0x12, 0x03, 0x07,
    0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x01, 0x12, 0x03, 0x07, 0x18, 0x37,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x03, 0x12, 0x03, 0x07, 0x3a, 0x3b, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x00, 0x02, 0x04, 0x12, 0x03, 0x08, 0x08, 0x2c, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x04, 0x04, 0x12, 0x03, 0x08, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x04, 0x05, 0x12, 0x03, 0x08, 0x11, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x01,
    0x12, 0x03, 0x08, 0x17, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x03, 0x12, 0x03,
    0x08, 0x2a, 0x2b,
];

static mut file_descriptor_proto_lazy: ::protobuf::lazy::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::lazy::Lazy {
    lock: ::protobuf::lazy::ONCE_INIT,
    ptr: 0 as *const ::protobuf::descriptor::FileDescriptorProto,
};

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    unsafe {
        file_descriptor_proto_lazy.get(|| {
            parse_descriptor_proto()
        })
    }
}
