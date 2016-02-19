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
pub struct CMsgProtoBufHeader {
    // message fields
    steamid: ::std::option::Option<u64>,
    client_sessionid: ::std::option::Option<i32>,
    routing_appid: ::std::option::Option<u32>,
    jobid_source: ::std::option::Option<u64>,
    jobid_target: ::std::option::Option<u64>,
    target_job_name: ::protobuf::SingularField<::std::string::String>,
    seq_num: ::std::option::Option<i32>,
    eresult: ::std::option::Option<i32>,
    error_message: ::protobuf::SingularField<::std::string::String>,
    ip: ::std::option::Option<u32>,
    auth_account_flags: ::std::option::Option<u32>,
    token_source: ::std::option::Option<u32>,
    admin_spoofing_user: ::std::option::Option<bool>,
    transport_error: ::std::option::Option<i32>,
    messageid: ::std::option::Option<u64>,
    publisher_group_id: ::std::option::Option<u32>,
    sysid: ::std::option::Option<u32>,
    trace_tag: ::std::option::Option<u64>,
    webapi_key_id: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgProtoBufHeader {}

impl CMsgProtoBufHeader {
    pub fn new() -> CMsgProtoBufHeader {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgProtoBufHeader {
        static mut instance: ::protobuf::lazy::Lazy<CMsgProtoBufHeader> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgProtoBufHeader,
        };
        unsafe {
            instance.get(|| {
                CMsgProtoBufHeader {
                    steamid: ::std::option::Option::None,
                    client_sessionid: ::std::option::Option::None,
                    routing_appid: ::std::option::Option::None,
                    jobid_source: ::std::option::Option::None,
                    jobid_target: ::std::option::Option::None,
                    target_job_name: ::protobuf::SingularField::none(),
                    seq_num: ::std::option::Option::None,
                    eresult: ::std::option::Option::None,
                    error_message: ::protobuf::SingularField::none(),
                    ip: ::std::option::Option::None,
                    auth_account_flags: ::std::option::Option::None,
                    token_source: ::std::option::Option::None,
                    admin_spoofing_user: ::std::option::Option::None,
                    transport_error: ::std::option::Option::None,
                    messageid: ::std::option::Option::None,
                    publisher_group_id: ::std::option::Option::None,
                    sysid: ::std::option::Option::None,
                    trace_tag: ::std::option::Option::None,
                    webapi_key_id: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional fixed64 steamid = 1;

    pub fn clear_steamid(&mut self) {
        self.steamid = ::std::option::Option::None;
    }

    pub fn has_steamid(&self) -> bool {
        self.steamid.is_some()
    }

    // Param is passed by value, moved
    pub fn set_steamid(&mut self, v: u64) {
        self.steamid = ::std::option::Option::Some(v);
    }

    pub fn get_steamid<'a>(&self) -> u64 {
        self.steamid.unwrap_or(0)
    }

    // optional int32 client_sessionid = 2;

    pub fn clear_client_sessionid(&mut self) {
        self.client_sessionid = ::std::option::Option::None;
    }

    pub fn has_client_sessionid(&self) -> bool {
        self.client_sessionid.is_some()
    }

    // Param is passed by value, moved
    pub fn set_client_sessionid(&mut self, v: i32) {
        self.client_sessionid = ::std::option::Option::Some(v);
    }

    pub fn get_client_sessionid<'a>(&self) -> i32 {
        self.client_sessionid.unwrap_or(0)
    }

    // optional uint32 routing_appid = 3;

    pub fn clear_routing_appid(&mut self) {
        self.routing_appid = ::std::option::Option::None;
    }

    pub fn has_routing_appid(&self) -> bool {
        self.routing_appid.is_some()
    }

    // Param is passed by value, moved
    pub fn set_routing_appid(&mut self, v: u32) {
        self.routing_appid = ::std::option::Option::Some(v);
    }

    pub fn get_routing_appid<'a>(&self) -> u32 {
        self.routing_appid.unwrap_or(0)
    }

    // optional fixed64 jobid_source = 10;

    pub fn clear_jobid_source(&mut self) {
        self.jobid_source = ::std::option::Option::None;
    }

    pub fn has_jobid_source(&self) -> bool {
        self.jobid_source.is_some()
    }

    // Param is passed by value, moved
    pub fn set_jobid_source(&mut self, v: u64) {
        self.jobid_source = ::std::option::Option::Some(v);
    }

    pub fn get_jobid_source<'a>(&self) -> u64 {
        self.jobid_source.unwrap_or(18446744073709551615u64)
    }

    // optional fixed64 jobid_target = 11;

    pub fn clear_jobid_target(&mut self) {
        self.jobid_target = ::std::option::Option::None;
    }

    pub fn has_jobid_target(&self) -> bool {
        self.jobid_target.is_some()
    }

    // Param is passed by value, moved
    pub fn set_jobid_target(&mut self, v: u64) {
        self.jobid_target = ::std::option::Option::Some(v);
    }

    pub fn get_jobid_target<'a>(&self) -> u64 {
        self.jobid_target.unwrap_or(18446744073709551615u64)
    }

    // optional string target_job_name = 12;

    pub fn clear_target_job_name(&mut self) {
        self.target_job_name.clear();
    }

    pub fn has_target_job_name(&self) -> bool {
        self.target_job_name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_target_job_name(&mut self, v: ::std::string::String) {
        self.target_job_name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_target_job_name<'a>(&'a mut self) -> &'a mut ::std::string::String {
        if self.target_job_name.is_none() {
            self.target_job_name.set_default();
        };
        self.target_job_name.as_mut().unwrap()
    }

    // Take field
    pub fn take_target_job_name(&mut self) -> ::std::string::String {
        self.target_job_name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_target_job_name<'a>(&'a self) -> &'a str {
        match self.target_job_name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // optional int32 seq_num = 24;

    pub fn clear_seq_num(&mut self) {
        self.seq_num = ::std::option::Option::None;
    }

    pub fn has_seq_num(&self) -> bool {
        self.seq_num.is_some()
    }

    // Param is passed by value, moved
    pub fn set_seq_num(&mut self, v: i32) {
        self.seq_num = ::std::option::Option::Some(v);
    }

    pub fn get_seq_num<'a>(&self) -> i32 {
        self.seq_num.unwrap_or(0)
    }

    // optional int32 eresult = 13;

    pub fn clear_eresult(&mut self) {
        self.eresult = ::std::option::Option::None;
    }

    pub fn has_eresult(&self) -> bool {
        self.eresult.is_some()
    }

    // Param is passed by value, moved
    pub fn set_eresult(&mut self, v: i32) {
        self.eresult = ::std::option::Option::Some(v);
    }

    pub fn get_eresult<'a>(&self) -> i32 {
        self.eresult.unwrap_or(2i32)
    }

    // optional string error_message = 14;

    pub fn clear_error_message(&mut self) {
        self.error_message.clear();
    }

    pub fn has_error_message(&self) -> bool {
        self.error_message.is_some()
    }

    // Param is passed by value, moved
    pub fn set_error_message(&mut self, v: ::std::string::String) {
        self.error_message = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_error_message<'a>(&'a mut self) -> &'a mut ::std::string::String {
        if self.error_message.is_none() {
            self.error_message.set_default();
        };
        self.error_message.as_mut().unwrap()
    }

    // Take field
    pub fn take_error_message(&mut self) -> ::std::string::String {
        self.error_message.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_error_message<'a>(&'a self) -> &'a str {
        match self.error_message.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // optional uint32 ip = 15;

    pub fn clear_ip(&mut self) {
        self.ip = ::std::option::Option::None;
    }

    pub fn has_ip(&self) -> bool {
        self.ip.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ip(&mut self, v: u32) {
        self.ip = ::std::option::Option::Some(v);
    }

    pub fn get_ip<'a>(&self) -> u32 {
        self.ip.unwrap_or(0)
    }

    // optional uint32 auth_account_flags = 16;

    pub fn clear_auth_account_flags(&mut self) {
        self.auth_account_flags = ::std::option::Option::None;
    }

    pub fn has_auth_account_flags(&self) -> bool {
        self.auth_account_flags.is_some()
    }

    // Param is passed by value, moved
    pub fn set_auth_account_flags(&mut self, v: u32) {
        self.auth_account_flags = ::std::option::Option::Some(v);
    }

    pub fn get_auth_account_flags<'a>(&self) -> u32 {
        self.auth_account_flags.unwrap_or(0)
    }

    // optional uint32 token_source = 22;

    pub fn clear_token_source(&mut self) {
        self.token_source = ::std::option::Option::None;
    }

    pub fn has_token_source(&self) -> bool {
        self.token_source.is_some()
    }

    // Param is passed by value, moved
    pub fn set_token_source(&mut self, v: u32) {
        self.token_source = ::std::option::Option::Some(v);
    }

    pub fn get_token_source<'a>(&self) -> u32 {
        self.token_source.unwrap_or(0)
    }

    // optional bool admin_spoofing_user = 23;

    pub fn clear_admin_spoofing_user(&mut self) {
        self.admin_spoofing_user = ::std::option::Option::None;
    }

    pub fn has_admin_spoofing_user(&self) -> bool {
        self.admin_spoofing_user.is_some()
    }

    // Param is passed by value, moved
    pub fn set_admin_spoofing_user(&mut self, v: bool) {
        self.admin_spoofing_user = ::std::option::Option::Some(v);
    }

    pub fn get_admin_spoofing_user<'a>(&self) -> bool {
        self.admin_spoofing_user.unwrap_or(false)
    }

    // optional int32 transport_error = 17;

    pub fn clear_transport_error(&mut self) {
        self.transport_error = ::std::option::Option::None;
    }

    pub fn has_transport_error(&self) -> bool {
        self.transport_error.is_some()
    }

    // Param is passed by value, moved
    pub fn set_transport_error(&mut self, v: i32) {
        self.transport_error = ::std::option::Option::Some(v);
    }

    pub fn get_transport_error<'a>(&self) -> i32 {
        self.transport_error.unwrap_or(1i32)
    }

    // optional uint64 messageid = 18;

    pub fn clear_messageid(&mut self) {
        self.messageid = ::std::option::Option::None;
    }

    pub fn has_messageid(&self) -> bool {
        self.messageid.is_some()
    }

    // Param is passed by value, moved
    pub fn set_messageid(&mut self, v: u64) {
        self.messageid = ::std::option::Option::Some(v);
    }

    pub fn get_messageid<'a>(&self) -> u64 {
        self.messageid.unwrap_or(18446744073709551615u64)
    }

    // optional uint32 publisher_group_id = 19;

    pub fn clear_publisher_group_id(&mut self) {
        self.publisher_group_id = ::std::option::Option::None;
    }

    pub fn has_publisher_group_id(&self) -> bool {
        self.publisher_group_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_publisher_group_id(&mut self, v: u32) {
        self.publisher_group_id = ::std::option::Option::Some(v);
    }

    pub fn get_publisher_group_id<'a>(&self) -> u32 {
        self.publisher_group_id.unwrap_or(0)
    }

    // optional uint32 sysid = 20;

    pub fn clear_sysid(&mut self) {
        self.sysid = ::std::option::Option::None;
    }

    pub fn has_sysid(&self) -> bool {
        self.sysid.is_some()
    }

    // Param is passed by value, moved
    pub fn set_sysid(&mut self, v: u32) {
        self.sysid = ::std::option::Option::Some(v);
    }

    pub fn get_sysid<'a>(&self) -> u32 {
        self.sysid.unwrap_or(0)
    }

    // optional uint64 trace_tag = 21;

    pub fn clear_trace_tag(&mut self) {
        self.trace_tag = ::std::option::Option::None;
    }

    pub fn has_trace_tag(&self) -> bool {
        self.trace_tag.is_some()
    }

    // Param is passed by value, moved
    pub fn set_trace_tag(&mut self, v: u64) {
        self.trace_tag = ::std::option::Option::Some(v);
    }

    pub fn get_trace_tag<'a>(&self) -> u64 {
        self.trace_tag.unwrap_or(0)
    }

    // optional uint32 webapi_key_id = 25;

    pub fn clear_webapi_key_id(&mut self) {
        self.webapi_key_id = ::std::option::Option::None;
    }

    pub fn has_webapi_key_id(&self) -> bool {
        self.webapi_key_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_webapi_key_id(&mut self, v: u32) {
        self.webapi_key_id = ::std::option::Option::Some(v);
    }

    pub fn get_webapi_key_id<'a>(&self) -> u32 {
        self.webapi_key_id.unwrap_or(0)
    }
}

impl ::protobuf::Message for CMsgProtoBufHeader {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_fixed64());
                    self.steamid = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.client_sessionid = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint32());
                    self.routing_appid = ::std::option::Option::Some(tmp);
                },
                10 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_fixed64());
                    self.jobid_source = ::std::option::Option::Some(tmp);
                },
                11 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_fixed64());
                    self.jobid_target = ::std::option::Option::Some(tmp);
                },
                12 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.target_job_name));
                },
                24 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.seq_num = ::std::option::Option::Some(tmp);
                },
                13 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.eresult = ::std::option::Option::Some(tmp);
                },
                14 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.error_message));
                },
                15 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint32());
                    self.ip = ::std::option::Option::Some(tmp);
                },
                16 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint32());
                    self.auth_account_flags = ::std::option::Option::Some(tmp);
                },
                22 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint32());
                    self.token_source = ::std::option::Option::Some(tmp);
                },
                23 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_bool());
                    self.admin_spoofing_user = ::std::option::Option::Some(tmp);
                },
                17 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.transport_error = ::std::option::Option::Some(tmp);
                },
                18 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint64());
                    self.messageid = ::std::option::Option::Some(tmp);
                },
                19 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint32());
                    self.publisher_group_id = ::std::option::Option::Some(tmp);
                },
                20 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint32());
                    self.sysid = ::std::option::Option::Some(tmp);
                },
                21 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint64());
                    self.trace_tag = ::std::option::Option::Some(tmp);
                },
                25 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint32());
                    self.webapi_key_id = ::std::option::Option::Some(tmp);
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
        if self.steamid.is_some() {
            my_size += 9;
        };
        for value in self.client_sessionid.iter() {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.routing_appid.iter() {
            my_size += ::protobuf::rt::value_size(3, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        if self.jobid_source.is_some() {
            my_size += 9;
        };
        if self.jobid_target.is_some() {
            my_size += 9;
        };
        for value in self.target_job_name.iter() {
            my_size += ::protobuf::rt::string_size(12, &value);
        };
        for value in self.seq_num.iter() {
            my_size += ::protobuf::rt::value_size(24, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.eresult.iter() {
            my_size += ::protobuf::rt::value_size(13, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.error_message.iter() {
            my_size += ::protobuf::rt::string_size(14, &value);
        };
        for value in self.ip.iter() {
            my_size += ::protobuf::rt::value_size(15, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.auth_account_flags.iter() {
            my_size += ::protobuf::rt::value_size(16, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.token_source.iter() {
            my_size += ::protobuf::rt::value_size(22, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        if self.admin_spoofing_user.is_some() {
            my_size += 3;
        };
        for value in self.transport_error.iter() {
            my_size += ::protobuf::rt::value_size(17, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.messageid.iter() {
            my_size += ::protobuf::rt::value_size(18, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.publisher_group_id.iter() {
            my_size += ::protobuf::rt::value_size(19, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.sysid.iter() {
            my_size += ::protobuf::rt::value_size(20, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.trace_tag.iter() {
            my_size += ::protobuf::rt::value_size(21, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.webapi_key_id.iter() {
            my_size += ::protobuf::rt::value_size(25, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.steamid {
            try!(os.write_fixed64(1, v));
        };
        if let Some(v) = self.client_sessionid {
            try!(os.write_int32(2, v));
        };
        if let Some(v) = self.routing_appid {
            try!(os.write_uint32(3, v));
        };
        if let Some(v) = self.jobid_source {
            try!(os.write_fixed64(10, v));
        };
        if let Some(v) = self.jobid_target {
            try!(os.write_fixed64(11, v));
        };
        if let Some(v) = self.target_job_name.as_ref() {
            try!(os.write_string(12, &v));
        };
        if let Some(v) = self.seq_num {
            try!(os.write_int32(24, v));
        };
        if let Some(v) = self.eresult {
            try!(os.write_int32(13, v));
        };
        if let Some(v) = self.error_message.as_ref() {
            try!(os.write_string(14, &v));
        };
        if let Some(v) = self.ip {
            try!(os.write_uint32(15, v));
        };
        if let Some(v) = self.auth_account_flags {
            try!(os.write_uint32(16, v));
        };
        if let Some(v) = self.token_source {
            try!(os.write_uint32(22, v));
        };
        if let Some(v) = self.admin_spoofing_user {
            try!(os.write_bool(23, v));
        };
        if let Some(v) = self.transport_error {
            try!(os.write_int32(17, v));
        };
        if let Some(v) = self.messageid {
            try!(os.write_uint64(18, v));
        };
        if let Some(v) = self.publisher_group_id {
            try!(os.write_uint32(19, v));
        };
        if let Some(v) = self.sysid {
            try!(os.write_uint32(20, v));
        };
        if let Some(v) = self.trace_tag {
            try!(os.write_uint64(21, v));
        };
        if let Some(v) = self.webapi_key_id {
            try!(os.write_uint32(25, v));
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
        ::std::any::TypeId::of::<CMsgProtoBufHeader>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CMsgProtoBufHeader {
    fn new() -> CMsgProtoBufHeader {
        CMsgProtoBufHeader::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgProtoBufHeader>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "steamid",
                    CMsgProtoBufHeader::has_steamid,
                    CMsgProtoBufHeader::get_steamid,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "client_sessionid",
                    CMsgProtoBufHeader::has_client_sessionid,
                    CMsgProtoBufHeader::get_client_sessionid,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "routing_appid",
                    CMsgProtoBufHeader::has_routing_appid,
                    CMsgProtoBufHeader::get_routing_appid,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "jobid_source",
                    CMsgProtoBufHeader::has_jobid_source,
                    CMsgProtoBufHeader::get_jobid_source,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "jobid_target",
                    CMsgProtoBufHeader::has_jobid_target,
                    CMsgProtoBufHeader::get_jobid_target,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "target_job_name",
                    CMsgProtoBufHeader::has_target_job_name,
                    CMsgProtoBufHeader::get_target_job_name,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "seq_num",
                    CMsgProtoBufHeader::has_seq_num,
                    CMsgProtoBufHeader::get_seq_num,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "eresult",
                    CMsgProtoBufHeader::has_eresult,
                    CMsgProtoBufHeader::get_eresult,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "error_message",
                    CMsgProtoBufHeader::has_error_message,
                    CMsgProtoBufHeader::get_error_message,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "ip",
                    CMsgProtoBufHeader::has_ip,
                    CMsgProtoBufHeader::get_ip,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "auth_account_flags",
                    CMsgProtoBufHeader::has_auth_account_flags,
                    CMsgProtoBufHeader::get_auth_account_flags,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "token_source",
                    CMsgProtoBufHeader::has_token_source,
                    CMsgProtoBufHeader::get_token_source,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "admin_spoofing_user",
                    CMsgProtoBufHeader::has_admin_spoofing_user,
                    CMsgProtoBufHeader::get_admin_spoofing_user,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "transport_error",
                    CMsgProtoBufHeader::has_transport_error,
                    CMsgProtoBufHeader::get_transport_error,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "messageid",
                    CMsgProtoBufHeader::has_messageid,
                    CMsgProtoBufHeader::get_messageid,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "publisher_group_id",
                    CMsgProtoBufHeader::has_publisher_group_id,
                    CMsgProtoBufHeader::get_publisher_group_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "sysid",
                    CMsgProtoBufHeader::has_sysid,
                    CMsgProtoBufHeader::get_sysid,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "trace_tag",
                    CMsgProtoBufHeader::has_trace_tag,
                    CMsgProtoBufHeader::get_trace_tag,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "webapi_key_id",
                    CMsgProtoBufHeader::has_webapi_key_id,
                    CMsgProtoBufHeader::get_webapi_key_id,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgProtoBufHeader>(
                    "CMsgProtoBufHeader",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgProtoBufHeader {
    fn clear(&mut self) {
        self.clear_steamid();
        self.clear_client_sessionid();
        self.clear_routing_appid();
        self.clear_jobid_source();
        self.clear_jobid_target();
        self.clear_target_job_name();
        self.clear_seq_num();
        self.clear_eresult();
        self.clear_error_message();
        self.clear_ip();
        self.clear_auth_account_flags();
        self.clear_token_source();
        self.clear_admin_spoofing_user();
        self.clear_transport_error();
        self.clear_messageid();
        self.clear_publisher_group_id();
        self.clear_sysid();
        self.clear_trace_tag();
        self.clear_webapi_key_id();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CMsgProtoBufHeader {
    fn eq(&self, other: &CMsgProtoBufHeader) -> bool {
        self.steamid == other.steamid &&
        self.client_sessionid == other.client_sessionid &&
        self.routing_appid == other.routing_appid &&
        self.jobid_source == other.jobid_source &&
        self.jobid_target == other.jobid_target &&
        self.target_job_name == other.target_job_name &&
        self.seq_num == other.seq_num &&
        self.eresult == other.eresult &&
        self.error_message == other.error_message &&
        self.ip == other.ip &&
        self.auth_account_flags == other.auth_account_flags &&
        self.token_source == other.token_source &&
        self.admin_spoofing_user == other.admin_spoofing_user &&
        self.transport_error == other.transport_error &&
        self.messageid == other.messageid &&
        self.publisher_group_id == other.publisher_group_id &&
        self.sysid == other.sysid &&
        self.trace_tag == other.trace_tag &&
        self.webapi_key_id == other.webapi_key_id &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for CMsgProtoBufHeader {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CMsgMulti {
    // message fields
    size_unzipped: ::std::option::Option<u32>,
    message_body: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgMulti {}

impl CMsgMulti {
    pub fn new() -> CMsgMulti {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgMulti {
        static mut instance: ::protobuf::lazy::Lazy<CMsgMulti> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgMulti,
        };
        unsafe {
            instance.get(|| {
                CMsgMulti {
                    size_unzipped: ::std::option::Option::None,
                    message_body: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional uint32 size_unzipped = 1;

    pub fn clear_size_unzipped(&mut self) {
        self.size_unzipped = ::std::option::Option::None;
    }

    pub fn has_size_unzipped(&self) -> bool {
        self.size_unzipped.is_some()
    }

    // Param is passed by value, moved
    pub fn set_size_unzipped(&mut self, v: u32) {
        self.size_unzipped = ::std::option::Option::Some(v);
    }

    pub fn get_size_unzipped<'a>(&self) -> u32 {
        self.size_unzipped.unwrap_or(0)
    }

    // optional bytes message_body = 2;

    pub fn clear_message_body(&mut self) {
        self.message_body.clear();
    }

    pub fn has_message_body(&self) -> bool {
        self.message_body.is_some()
    }

    // Param is passed by value, moved
    pub fn set_message_body(&mut self, v: ::std::vec::Vec<u8>) {
        self.message_body = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_message_body<'a>(&'a mut self) -> &'a mut ::std::vec::Vec<u8> {
        if self.message_body.is_none() {
            self.message_body.set_default();
        };
        self.message_body.as_mut().unwrap()
    }

    // Take field
    pub fn take_message_body(&mut self) -> ::std::vec::Vec<u8> {
        self.message_body.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_message_body<'a>(&'a self) -> &'a [u8] {
        match self.message_body.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }
}

impl ::protobuf::Message for CMsgMulti {
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
                    self.size_unzipped = ::std::option::Option::Some(tmp);
                },
                2 => {
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.message_body));
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
        for value in self.size_unzipped.iter() {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.message_body.iter() {
            my_size += ::protobuf::rt::bytes_size(2, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.size_unzipped {
            try!(os.write_uint32(1, v));
        };
        if let Some(v) = self.message_body.as_ref() {
            try!(os.write_bytes(2, &v));
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
        ::std::any::TypeId::of::<CMsgMulti>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CMsgMulti {
    fn new() -> CMsgMulti {
        CMsgMulti::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgMulti>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "size_unzipped",
                    CMsgMulti::has_size_unzipped,
                    CMsgMulti::get_size_unzipped,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "message_body",
                    CMsgMulti::has_message_body,
                    CMsgMulti::get_message_body,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgMulti>(
                    "CMsgMulti",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgMulti {
    fn clear(&mut self) {
        self.clear_size_unzipped();
        self.clear_message_body();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CMsgMulti {
    fn eq(&self, other: &CMsgMulti) -> bool {
        self.size_unzipped == other.size_unzipped &&
        self.message_body == other.message_body &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for CMsgMulti {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CMsgProtobufWrapped {
    // message fields
    message_body: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgProtobufWrapped {}

impl CMsgProtobufWrapped {
    pub fn new() -> CMsgProtobufWrapped {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgProtobufWrapped {
        static mut instance: ::protobuf::lazy::Lazy<CMsgProtobufWrapped> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgProtobufWrapped,
        };
        unsafe {
            instance.get(|| {
                CMsgProtobufWrapped {
                    message_body: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional bytes message_body = 1;

    pub fn clear_message_body(&mut self) {
        self.message_body.clear();
    }

    pub fn has_message_body(&self) -> bool {
        self.message_body.is_some()
    }

    // Param is passed by value, moved
    pub fn set_message_body(&mut self, v: ::std::vec::Vec<u8>) {
        self.message_body = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_message_body<'a>(&'a mut self) -> &'a mut ::std::vec::Vec<u8> {
        if self.message_body.is_none() {
            self.message_body.set_default();
        };
        self.message_body.as_mut().unwrap()
    }

    // Take field
    pub fn take_message_body(&mut self) -> ::std::vec::Vec<u8> {
        self.message_body.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_message_body<'a>(&'a self) -> &'a [u8] {
        match self.message_body.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }
}

impl ::protobuf::Message for CMsgProtobufWrapped {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.message_body));
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
        for value in self.message_body.iter() {
            my_size += ::protobuf::rt::bytes_size(1, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.message_body.as_ref() {
            try!(os.write_bytes(1, &v));
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
        ::std::any::TypeId::of::<CMsgProtobufWrapped>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CMsgProtobufWrapped {
    fn new() -> CMsgProtobufWrapped {
        CMsgProtobufWrapped::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgProtobufWrapped>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "message_body",
                    CMsgProtobufWrapped::has_message_body,
                    CMsgProtobufWrapped::get_message_body,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgProtobufWrapped>(
                    "CMsgProtobufWrapped",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgProtobufWrapped {
    fn clear(&mut self) {
        self.clear_message_body();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CMsgProtobufWrapped {
    fn eq(&self, other: &CMsgProtobufWrapped) -> bool {
        self.message_body == other.message_body &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for CMsgProtobufWrapped {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CMsgAuthTicket {
    // message fields
    estate: ::std::option::Option<u32>,
    eresult: ::std::option::Option<u32>,
    steamid: ::std::option::Option<u64>,
    gameid: ::std::option::Option<u64>,
    h_steam_pipe: ::std::option::Option<u32>,
    ticket_crc: ::std::option::Option<u32>,
    ticket: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgAuthTicket {}

impl CMsgAuthTicket {
    pub fn new() -> CMsgAuthTicket {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgAuthTicket {
        static mut instance: ::protobuf::lazy::Lazy<CMsgAuthTicket> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgAuthTicket,
        };
        unsafe {
            instance.get(|| {
                CMsgAuthTicket {
                    estate: ::std::option::Option::None,
                    eresult: ::std::option::Option::None,
                    steamid: ::std::option::Option::None,
                    gameid: ::std::option::Option::None,
                    h_steam_pipe: ::std::option::Option::None,
                    ticket_crc: ::std::option::Option::None,
                    ticket: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional uint32 estate = 1;

    pub fn clear_estate(&mut self) {
        self.estate = ::std::option::Option::None;
    }

    pub fn has_estate(&self) -> bool {
        self.estate.is_some()
    }

    // Param is passed by value, moved
    pub fn set_estate(&mut self, v: u32) {
        self.estate = ::std::option::Option::Some(v);
    }

    pub fn get_estate<'a>(&self) -> u32 {
        self.estate.unwrap_or(0)
    }

    // optional uint32 eresult = 2;

    pub fn clear_eresult(&mut self) {
        self.eresult = ::std::option::Option::None;
    }

    pub fn has_eresult(&self) -> bool {
        self.eresult.is_some()
    }

    // Param is passed by value, moved
    pub fn set_eresult(&mut self, v: u32) {
        self.eresult = ::std::option::Option::Some(v);
    }

    pub fn get_eresult<'a>(&self) -> u32 {
        self.eresult.unwrap_or(2u32)
    }

    // optional fixed64 steamid = 3;

    pub fn clear_steamid(&mut self) {
        self.steamid = ::std::option::Option::None;
    }

    pub fn has_steamid(&self) -> bool {
        self.steamid.is_some()
    }

    // Param is passed by value, moved
    pub fn set_steamid(&mut self, v: u64) {
        self.steamid = ::std::option::Option::Some(v);
    }

    pub fn get_steamid<'a>(&self) -> u64 {
        self.steamid.unwrap_or(0)
    }

    // optional fixed64 gameid = 4;

    pub fn clear_gameid(&mut self) {
        self.gameid = ::std::option::Option::None;
    }

    pub fn has_gameid(&self) -> bool {
        self.gameid.is_some()
    }

    // Param is passed by value, moved
    pub fn set_gameid(&mut self, v: u64) {
        self.gameid = ::std::option::Option::Some(v);
    }

    pub fn get_gameid<'a>(&self) -> u64 {
        self.gameid.unwrap_or(0)
    }

    // optional uint32 h_steam_pipe = 5;

    pub fn clear_h_steam_pipe(&mut self) {
        self.h_steam_pipe = ::std::option::Option::None;
    }

    pub fn has_h_steam_pipe(&self) -> bool {
        self.h_steam_pipe.is_some()
    }

    // Param is passed by value, moved
    pub fn set_h_steam_pipe(&mut self, v: u32) {
        self.h_steam_pipe = ::std::option::Option::Some(v);
    }

    pub fn get_h_steam_pipe<'a>(&self) -> u32 {
        self.h_steam_pipe.unwrap_or(0)
    }

    // optional uint32 ticket_crc = 6;

    pub fn clear_ticket_crc(&mut self) {
        self.ticket_crc = ::std::option::Option::None;
    }

    pub fn has_ticket_crc(&self) -> bool {
        self.ticket_crc.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ticket_crc(&mut self, v: u32) {
        self.ticket_crc = ::std::option::Option::Some(v);
    }

    pub fn get_ticket_crc<'a>(&self) -> u32 {
        self.ticket_crc.unwrap_or(0)
    }

    // optional bytes ticket = 7;

    pub fn clear_ticket(&mut self) {
        self.ticket.clear();
    }

    pub fn has_ticket(&self) -> bool {
        self.ticket.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ticket(&mut self, v: ::std::vec::Vec<u8>) {
        self.ticket = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_ticket<'a>(&'a mut self) -> &'a mut ::std::vec::Vec<u8> {
        if self.ticket.is_none() {
            self.ticket.set_default();
        };
        self.ticket.as_mut().unwrap()
    }

    // Take field
    pub fn take_ticket(&mut self) -> ::std::vec::Vec<u8> {
        self.ticket.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_ticket<'a>(&'a self) -> &'a [u8] {
        match self.ticket.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }
}

impl ::protobuf::Message for CMsgAuthTicket {
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
                    self.estate = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint32());
                    self.eresult = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_fixed64());
                    self.steamid = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_fixed64());
                    self.gameid = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint32());
                    self.h_steam_pipe = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint32());
                    self.ticket_crc = ::std::option::Option::Some(tmp);
                },
                7 => {
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.ticket));
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
        for value in self.estate.iter() {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.eresult.iter() {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        if self.steamid.is_some() {
            my_size += 9;
        };
        if self.gameid.is_some() {
            my_size += 9;
        };
        for value in self.h_steam_pipe.iter() {
            my_size += ::protobuf::rt::value_size(5, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.ticket_crc.iter() {
            my_size += ::protobuf::rt::value_size(6, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.ticket.iter() {
            my_size += ::protobuf::rt::bytes_size(7, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.estate {
            try!(os.write_uint32(1, v));
        };
        if let Some(v) = self.eresult {
            try!(os.write_uint32(2, v));
        };
        if let Some(v) = self.steamid {
            try!(os.write_fixed64(3, v));
        };
        if let Some(v) = self.gameid {
            try!(os.write_fixed64(4, v));
        };
        if let Some(v) = self.h_steam_pipe {
            try!(os.write_uint32(5, v));
        };
        if let Some(v) = self.ticket_crc {
            try!(os.write_uint32(6, v));
        };
        if let Some(v) = self.ticket.as_ref() {
            try!(os.write_bytes(7, &v));
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
        ::std::any::TypeId::of::<CMsgAuthTicket>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CMsgAuthTicket {
    fn new() -> CMsgAuthTicket {
        CMsgAuthTicket::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgAuthTicket>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "estate",
                    CMsgAuthTicket::has_estate,
                    CMsgAuthTicket::get_estate,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "eresult",
                    CMsgAuthTicket::has_eresult,
                    CMsgAuthTicket::get_eresult,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "steamid",
                    CMsgAuthTicket::has_steamid,
                    CMsgAuthTicket::get_steamid,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "gameid",
                    CMsgAuthTicket::has_gameid,
                    CMsgAuthTicket::get_gameid,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "h_steam_pipe",
                    CMsgAuthTicket::has_h_steam_pipe,
                    CMsgAuthTicket::get_h_steam_pipe,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "ticket_crc",
                    CMsgAuthTicket::has_ticket_crc,
                    CMsgAuthTicket::get_ticket_crc,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "ticket",
                    CMsgAuthTicket::has_ticket,
                    CMsgAuthTicket::get_ticket,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgAuthTicket>(
                    "CMsgAuthTicket",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgAuthTicket {
    fn clear(&mut self) {
        self.clear_estate();
        self.clear_eresult();
        self.clear_steamid();
        self.clear_gameid();
        self.clear_h_steam_pipe();
        self.clear_ticket_crc();
        self.clear_ticket();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CMsgAuthTicket {
    fn eq(&self, other: &CMsgAuthTicket) -> bool {
        self.estate == other.estate &&
        self.eresult == other.eresult &&
        self.steamid == other.steamid &&
        self.gameid == other.gameid &&
        self.h_steam_pipe == other.h_steam_pipe &&
        self.ticket_crc == other.ticket_crc &&
        self.ticket == other.ticket &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for CMsgAuthTicket {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CCDDBAppDetailCommon {
    // message fields
    appid: ::std::option::Option<u32>,
    name: ::protobuf::SingularField<::std::string::String>,
    icon: ::protobuf::SingularField<::std::string::String>,
    logo: ::protobuf::SingularField<::std::string::String>,
    logo_small: ::protobuf::SingularField<::std::string::String>,
    tool: ::std::option::Option<bool>,
    demo: ::std::option::Option<bool>,
    media: ::std::option::Option<bool>,
    community_visible_stats: ::std::option::Option<bool>,
    friendly_name: ::protobuf::SingularField<::std::string::String>,
    propagation: ::protobuf::SingularField<::std::string::String>,
    has_adult_content: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CCDDBAppDetailCommon {}

impl CCDDBAppDetailCommon {
    pub fn new() -> CCDDBAppDetailCommon {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CCDDBAppDetailCommon {
        static mut instance: ::protobuf::lazy::Lazy<CCDDBAppDetailCommon> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CCDDBAppDetailCommon,
        };
        unsafe {
            instance.get(|| {
                CCDDBAppDetailCommon {
                    appid: ::std::option::Option::None,
                    name: ::protobuf::SingularField::none(),
                    icon: ::protobuf::SingularField::none(),
                    logo: ::protobuf::SingularField::none(),
                    logo_small: ::protobuf::SingularField::none(),
                    tool: ::std::option::Option::None,
                    demo: ::std::option::Option::None,
                    media: ::std::option::Option::None,
                    community_visible_stats: ::std::option::Option::None,
                    friendly_name: ::protobuf::SingularField::none(),
                    propagation: ::protobuf::SingularField::none(),
                    has_adult_content: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional uint32 appid = 1;

    pub fn clear_appid(&mut self) {
        self.appid = ::std::option::Option::None;
    }

    pub fn has_appid(&self) -> bool {
        self.appid.is_some()
    }

    // Param is passed by value, moved
    pub fn set_appid(&mut self, v: u32) {
        self.appid = ::std::option::Option::Some(v);
    }

    pub fn get_appid<'a>(&self) -> u32 {
        self.appid.unwrap_or(0)
    }

    // optional string name = 2;

    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    pub fn has_name(&self) -> bool {
        self.name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name<'a>(&'a mut self) -> &'a mut ::std::string::String {
        if self.name.is_none() {
            self.name.set_default();
        };
        self.name.as_mut().unwrap()
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::string::String {
        self.name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_name<'a>(&'a self) -> &'a str {
        match self.name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // optional string icon = 3;

    pub fn clear_icon(&mut self) {
        self.icon.clear();
    }

    pub fn has_icon(&self) -> bool {
        self.icon.is_some()
    }

    // Param is passed by value, moved
    pub fn set_icon(&mut self, v: ::std::string::String) {
        self.icon = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_icon<'a>(&'a mut self) -> &'a mut ::std::string::String {
        if self.icon.is_none() {
            self.icon.set_default();
        };
        self.icon.as_mut().unwrap()
    }

    // Take field
    pub fn take_icon(&mut self) -> ::std::string::String {
        self.icon.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_icon<'a>(&'a self) -> &'a str {
        match self.icon.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // optional string logo = 4;

    pub fn clear_logo(&mut self) {
        self.logo.clear();
    }

    pub fn has_logo(&self) -> bool {
        self.logo.is_some()
    }

    // Param is passed by value, moved
    pub fn set_logo(&mut self, v: ::std::string::String) {
        self.logo = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_logo<'a>(&'a mut self) -> &'a mut ::std::string::String {
        if self.logo.is_none() {
            self.logo.set_default();
        };
        self.logo.as_mut().unwrap()
    }

    // Take field
    pub fn take_logo(&mut self) -> ::std::string::String {
        self.logo.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_logo<'a>(&'a self) -> &'a str {
        match self.logo.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // optional string logo_small = 5;

    pub fn clear_logo_small(&mut self) {
        self.logo_small.clear();
    }

    pub fn has_logo_small(&self) -> bool {
        self.logo_small.is_some()
    }

    // Param is passed by value, moved
    pub fn set_logo_small(&mut self, v: ::std::string::String) {
        self.logo_small = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_logo_small<'a>(&'a mut self) -> &'a mut ::std::string::String {
        if self.logo_small.is_none() {
            self.logo_small.set_default();
        };
        self.logo_small.as_mut().unwrap()
    }

    // Take field
    pub fn take_logo_small(&mut self) -> ::std::string::String {
        self.logo_small.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_logo_small<'a>(&'a self) -> &'a str {
        match self.logo_small.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // optional bool tool = 6;

    pub fn clear_tool(&mut self) {
        self.tool = ::std::option::Option::None;
    }

    pub fn has_tool(&self) -> bool {
        self.tool.is_some()
    }

    // Param is passed by value, moved
    pub fn set_tool(&mut self, v: bool) {
        self.tool = ::std::option::Option::Some(v);
    }

    pub fn get_tool<'a>(&self) -> bool {
        self.tool.unwrap_or(false)
    }

    // optional bool demo = 7;

    pub fn clear_demo(&mut self) {
        self.demo = ::std::option::Option::None;
    }

    pub fn has_demo(&self) -> bool {
        self.demo.is_some()
    }

    // Param is passed by value, moved
    pub fn set_demo(&mut self, v: bool) {
        self.demo = ::std::option::Option::Some(v);
    }

    pub fn get_demo<'a>(&self) -> bool {
        self.demo.unwrap_or(false)
    }

    // optional bool media = 8;

    pub fn clear_media(&mut self) {
        self.media = ::std::option::Option::None;
    }

    pub fn has_media(&self) -> bool {
        self.media.is_some()
    }

    // Param is passed by value, moved
    pub fn set_media(&mut self, v: bool) {
        self.media = ::std::option::Option::Some(v);
    }

    pub fn get_media<'a>(&self) -> bool {
        self.media.unwrap_or(false)
    }

    // optional bool community_visible_stats = 9;

    pub fn clear_community_visible_stats(&mut self) {
        self.community_visible_stats = ::std::option::Option::None;
    }

    pub fn has_community_visible_stats(&self) -> bool {
        self.community_visible_stats.is_some()
    }

    // Param is passed by value, moved
    pub fn set_community_visible_stats(&mut self, v: bool) {
        self.community_visible_stats = ::std::option::Option::Some(v);
    }

    pub fn get_community_visible_stats<'a>(&self) -> bool {
        self.community_visible_stats.unwrap_or(false)
    }

    // optional string friendly_name = 10;

    pub fn clear_friendly_name(&mut self) {
        self.friendly_name.clear();
    }

    pub fn has_friendly_name(&self) -> bool {
        self.friendly_name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_friendly_name(&mut self, v: ::std::string::String) {
        self.friendly_name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_friendly_name<'a>(&'a mut self) -> &'a mut ::std::string::String {
        if self.friendly_name.is_none() {
            self.friendly_name.set_default();
        };
        self.friendly_name.as_mut().unwrap()
    }

    // Take field
    pub fn take_friendly_name(&mut self) -> ::std::string::String {
        self.friendly_name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_friendly_name<'a>(&'a self) -> &'a str {
        match self.friendly_name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // optional string propagation = 11;

    pub fn clear_propagation(&mut self) {
        self.propagation.clear();
    }

    pub fn has_propagation(&self) -> bool {
        self.propagation.is_some()
    }

    // Param is passed by value, moved
    pub fn set_propagation(&mut self, v: ::std::string::String) {
        self.propagation = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_propagation<'a>(&'a mut self) -> &'a mut ::std::string::String {
        if self.propagation.is_none() {
            self.propagation.set_default();
        };
        self.propagation.as_mut().unwrap()
    }

    // Take field
    pub fn take_propagation(&mut self) -> ::std::string::String {
        self.propagation.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_propagation<'a>(&'a self) -> &'a str {
        match self.propagation.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // optional bool has_adult_content = 12;

    pub fn clear_has_adult_content(&mut self) {
        self.has_adult_content = ::std::option::Option::None;
    }

    pub fn has_has_adult_content(&self) -> bool {
        self.has_adult_content.is_some()
    }

    // Param is passed by value, moved
    pub fn set_has_adult_content(&mut self, v: bool) {
        self.has_adult_content = ::std::option::Option::Some(v);
    }

    pub fn get_has_adult_content<'a>(&self) -> bool {
        self.has_adult_content.unwrap_or(false)
    }
}

impl ::protobuf::Message for CCDDBAppDetailCommon {
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
                    self.appid = ::std::option::Option::Some(tmp);
                },
                2 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.name));
                },
                3 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.icon));
                },
                4 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.logo));
                },
                5 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.logo_small));
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_bool());
                    self.tool = ::std::option::Option::Some(tmp);
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_bool());
                    self.demo = ::std::option::Option::Some(tmp);
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_bool());
                    self.media = ::std::option::Option::Some(tmp);
                },
                9 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_bool());
                    self.community_visible_stats = ::std::option::Option::Some(tmp);
                },
                10 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.friendly_name));
                },
                11 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.propagation));
                },
                12 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_bool());
                    self.has_adult_content = ::std::option::Option::Some(tmp);
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
        for value in self.appid.iter() {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.name.iter() {
            my_size += ::protobuf::rt::string_size(2, &value);
        };
        for value in self.icon.iter() {
            my_size += ::protobuf::rt::string_size(3, &value);
        };
        for value in self.logo.iter() {
            my_size += ::protobuf::rt::string_size(4, &value);
        };
        for value in self.logo_small.iter() {
            my_size += ::protobuf::rt::string_size(5, &value);
        };
        if self.tool.is_some() {
            my_size += 2;
        };
        if self.demo.is_some() {
            my_size += 2;
        };
        if self.media.is_some() {
            my_size += 2;
        };
        if self.community_visible_stats.is_some() {
            my_size += 2;
        };
        for value in self.friendly_name.iter() {
            my_size += ::protobuf::rt::string_size(10, &value);
        };
        for value in self.propagation.iter() {
            my_size += ::protobuf::rt::string_size(11, &value);
        };
        if self.has_adult_content.is_some() {
            my_size += 2;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.appid {
            try!(os.write_uint32(1, v));
        };
        if let Some(v) = self.name.as_ref() {
            try!(os.write_string(2, &v));
        };
        if let Some(v) = self.icon.as_ref() {
            try!(os.write_string(3, &v));
        };
        if let Some(v) = self.logo.as_ref() {
            try!(os.write_string(4, &v));
        };
        if let Some(v) = self.logo_small.as_ref() {
            try!(os.write_string(5, &v));
        };
        if let Some(v) = self.tool {
            try!(os.write_bool(6, v));
        };
        if let Some(v) = self.demo {
            try!(os.write_bool(7, v));
        };
        if let Some(v) = self.media {
            try!(os.write_bool(8, v));
        };
        if let Some(v) = self.community_visible_stats {
            try!(os.write_bool(9, v));
        };
        if let Some(v) = self.friendly_name.as_ref() {
            try!(os.write_string(10, &v));
        };
        if let Some(v) = self.propagation.as_ref() {
            try!(os.write_string(11, &v));
        };
        if let Some(v) = self.has_adult_content {
            try!(os.write_bool(12, v));
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
        ::std::any::TypeId::of::<CCDDBAppDetailCommon>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CCDDBAppDetailCommon {
    fn new() -> CCDDBAppDetailCommon {
        CCDDBAppDetailCommon::new()
    }

    fn descriptor_static(_: ::std::option::Option<CCDDBAppDetailCommon>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "appid",
                    CCDDBAppDetailCommon::has_appid,
                    CCDDBAppDetailCommon::get_appid,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "name",
                    CCDDBAppDetailCommon::has_name,
                    CCDDBAppDetailCommon::get_name,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "icon",
                    CCDDBAppDetailCommon::has_icon,
                    CCDDBAppDetailCommon::get_icon,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "logo",
                    CCDDBAppDetailCommon::has_logo,
                    CCDDBAppDetailCommon::get_logo,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "logo_small",
                    CCDDBAppDetailCommon::has_logo_small,
                    CCDDBAppDetailCommon::get_logo_small,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "tool",
                    CCDDBAppDetailCommon::has_tool,
                    CCDDBAppDetailCommon::get_tool,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "demo",
                    CCDDBAppDetailCommon::has_demo,
                    CCDDBAppDetailCommon::get_demo,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "media",
                    CCDDBAppDetailCommon::has_media,
                    CCDDBAppDetailCommon::get_media,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "community_visible_stats",
                    CCDDBAppDetailCommon::has_community_visible_stats,
                    CCDDBAppDetailCommon::get_community_visible_stats,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "friendly_name",
                    CCDDBAppDetailCommon::has_friendly_name,
                    CCDDBAppDetailCommon::get_friendly_name,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "propagation",
                    CCDDBAppDetailCommon::has_propagation,
                    CCDDBAppDetailCommon::get_propagation,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "has_adult_content",
                    CCDDBAppDetailCommon::has_has_adult_content,
                    CCDDBAppDetailCommon::get_has_adult_content,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CCDDBAppDetailCommon>(
                    "CCDDBAppDetailCommon",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CCDDBAppDetailCommon {
    fn clear(&mut self) {
        self.clear_appid();
        self.clear_name();
        self.clear_icon();
        self.clear_logo();
        self.clear_logo_small();
        self.clear_tool();
        self.clear_demo();
        self.clear_media();
        self.clear_community_visible_stats();
        self.clear_friendly_name();
        self.clear_propagation();
        self.clear_has_adult_content();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CCDDBAppDetailCommon {
    fn eq(&self, other: &CCDDBAppDetailCommon) -> bool {
        self.appid == other.appid &&
        self.name == other.name &&
        self.icon == other.icon &&
        self.logo == other.logo &&
        self.logo_small == other.logo_small &&
        self.tool == other.tool &&
        self.demo == other.demo &&
        self.media == other.media &&
        self.community_visible_stats == other.community_visible_stats &&
        self.friendly_name == other.friendly_name &&
        self.propagation == other.propagation &&
        self.has_adult_content == other.has_adult_content &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for CCDDBAppDetailCommon {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CMsgAppRights {
    // message fields
    edit_info: ::std::option::Option<bool>,
    publish: ::std::option::Option<bool>,
    view_error_data: ::std::option::Option<bool>,
    download: ::std::option::Option<bool>,
    upload_cdkeys: ::std::option::Option<bool>,
    generate_cdkeys: ::std::option::Option<bool>,
    view_financials: ::std::option::Option<bool>,
    manage_ceg: ::std::option::Option<bool>,
    manage_signing: ::std::option::Option<bool>,
    manage_cdkeys: ::std::option::Option<bool>,
    edit_marketing: ::std::option::Option<bool>,
    economy_support: ::std::option::Option<bool>,
    economy_support_supervisor: ::std::option::Option<bool>,
    manage_pricing: ::std::option::Option<bool>,
    broadcast_live: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgAppRights {}

impl CMsgAppRights {
    pub fn new() -> CMsgAppRights {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgAppRights {
        static mut instance: ::protobuf::lazy::Lazy<CMsgAppRights> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgAppRights,
        };
        unsafe {
            instance.get(|| {
                CMsgAppRights {
                    edit_info: ::std::option::Option::None,
                    publish: ::std::option::Option::None,
                    view_error_data: ::std::option::Option::None,
                    download: ::std::option::Option::None,
                    upload_cdkeys: ::std::option::Option::None,
                    generate_cdkeys: ::std::option::Option::None,
                    view_financials: ::std::option::Option::None,
                    manage_ceg: ::std::option::Option::None,
                    manage_signing: ::std::option::Option::None,
                    manage_cdkeys: ::std::option::Option::None,
                    edit_marketing: ::std::option::Option::None,
                    economy_support: ::std::option::Option::None,
                    economy_support_supervisor: ::std::option::Option::None,
                    manage_pricing: ::std::option::Option::None,
                    broadcast_live: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional bool edit_info = 1;

    pub fn clear_edit_info(&mut self) {
        self.edit_info = ::std::option::Option::None;
    }

    pub fn has_edit_info(&self) -> bool {
        self.edit_info.is_some()
    }

    // Param is passed by value, moved
    pub fn set_edit_info(&mut self, v: bool) {
        self.edit_info = ::std::option::Option::Some(v);
    }

    pub fn get_edit_info<'a>(&self) -> bool {
        self.edit_info.unwrap_or(false)
    }

    // optional bool publish = 2;

    pub fn clear_publish(&mut self) {
        self.publish = ::std::option::Option::None;
    }

    pub fn has_publish(&self) -> bool {
        self.publish.is_some()
    }

    // Param is passed by value, moved
    pub fn set_publish(&mut self, v: bool) {
        self.publish = ::std::option::Option::Some(v);
    }

    pub fn get_publish<'a>(&self) -> bool {
        self.publish.unwrap_or(false)
    }

    // optional bool view_error_data = 3;

    pub fn clear_view_error_data(&mut self) {
        self.view_error_data = ::std::option::Option::None;
    }

    pub fn has_view_error_data(&self) -> bool {
        self.view_error_data.is_some()
    }

    // Param is passed by value, moved
    pub fn set_view_error_data(&mut self, v: bool) {
        self.view_error_data = ::std::option::Option::Some(v);
    }

    pub fn get_view_error_data<'a>(&self) -> bool {
        self.view_error_data.unwrap_or(false)
    }

    // optional bool download = 4;

    pub fn clear_download(&mut self) {
        self.download = ::std::option::Option::None;
    }

    pub fn has_download(&self) -> bool {
        self.download.is_some()
    }

    // Param is passed by value, moved
    pub fn set_download(&mut self, v: bool) {
        self.download = ::std::option::Option::Some(v);
    }

    pub fn get_download<'a>(&self) -> bool {
        self.download.unwrap_or(false)
    }

    // optional bool upload_cdkeys = 5;

    pub fn clear_upload_cdkeys(&mut self) {
        self.upload_cdkeys = ::std::option::Option::None;
    }

    pub fn has_upload_cdkeys(&self) -> bool {
        self.upload_cdkeys.is_some()
    }

    // Param is passed by value, moved
    pub fn set_upload_cdkeys(&mut self, v: bool) {
        self.upload_cdkeys = ::std::option::Option::Some(v);
    }

    pub fn get_upload_cdkeys<'a>(&self) -> bool {
        self.upload_cdkeys.unwrap_or(false)
    }

    // optional bool generate_cdkeys = 6;

    pub fn clear_generate_cdkeys(&mut self) {
        self.generate_cdkeys = ::std::option::Option::None;
    }

    pub fn has_generate_cdkeys(&self) -> bool {
        self.generate_cdkeys.is_some()
    }

    // Param is passed by value, moved
    pub fn set_generate_cdkeys(&mut self, v: bool) {
        self.generate_cdkeys = ::std::option::Option::Some(v);
    }

    pub fn get_generate_cdkeys<'a>(&self) -> bool {
        self.generate_cdkeys.unwrap_or(false)
    }

    // optional bool view_financials = 7;

    pub fn clear_view_financials(&mut self) {
        self.view_financials = ::std::option::Option::None;
    }

    pub fn has_view_financials(&self) -> bool {
        self.view_financials.is_some()
    }

    // Param is passed by value, moved
    pub fn set_view_financials(&mut self, v: bool) {
        self.view_financials = ::std::option::Option::Some(v);
    }

    pub fn get_view_financials<'a>(&self) -> bool {
        self.view_financials.unwrap_or(false)
    }

    // optional bool manage_ceg = 8;

    pub fn clear_manage_ceg(&mut self) {
        self.manage_ceg = ::std::option::Option::None;
    }

    pub fn has_manage_ceg(&self) -> bool {
        self.manage_ceg.is_some()
    }

    // Param is passed by value, moved
    pub fn set_manage_ceg(&mut self, v: bool) {
        self.manage_ceg = ::std::option::Option::Some(v);
    }

    pub fn get_manage_ceg<'a>(&self) -> bool {
        self.manage_ceg.unwrap_or(false)
    }

    // optional bool manage_signing = 9;

    pub fn clear_manage_signing(&mut self) {
        self.manage_signing = ::std::option::Option::None;
    }

    pub fn has_manage_signing(&self) -> bool {
        self.manage_signing.is_some()
    }

    // Param is passed by value, moved
    pub fn set_manage_signing(&mut self, v: bool) {
        self.manage_signing = ::std::option::Option::Some(v);
    }

    pub fn get_manage_signing<'a>(&self) -> bool {
        self.manage_signing.unwrap_or(false)
    }

    // optional bool manage_cdkeys = 10;

    pub fn clear_manage_cdkeys(&mut self) {
        self.manage_cdkeys = ::std::option::Option::None;
    }

    pub fn has_manage_cdkeys(&self) -> bool {
        self.manage_cdkeys.is_some()
    }

    // Param is passed by value, moved
    pub fn set_manage_cdkeys(&mut self, v: bool) {
        self.manage_cdkeys = ::std::option::Option::Some(v);
    }

    pub fn get_manage_cdkeys<'a>(&self) -> bool {
        self.manage_cdkeys.unwrap_or(false)
    }

    // optional bool edit_marketing = 11;

    pub fn clear_edit_marketing(&mut self) {
        self.edit_marketing = ::std::option::Option::None;
    }

    pub fn has_edit_marketing(&self) -> bool {
        self.edit_marketing.is_some()
    }

    // Param is passed by value, moved
    pub fn set_edit_marketing(&mut self, v: bool) {
        self.edit_marketing = ::std::option::Option::Some(v);
    }

    pub fn get_edit_marketing<'a>(&self) -> bool {
        self.edit_marketing.unwrap_or(false)
    }

    // optional bool economy_support = 12;

    pub fn clear_economy_support(&mut self) {
        self.economy_support = ::std::option::Option::None;
    }

    pub fn has_economy_support(&self) -> bool {
        self.economy_support.is_some()
    }

    // Param is passed by value, moved
    pub fn set_economy_support(&mut self, v: bool) {
        self.economy_support = ::std::option::Option::Some(v);
    }

    pub fn get_economy_support<'a>(&self) -> bool {
        self.economy_support.unwrap_or(false)
    }

    // optional bool economy_support_supervisor = 13;

    pub fn clear_economy_support_supervisor(&mut self) {
        self.economy_support_supervisor = ::std::option::Option::None;
    }

    pub fn has_economy_support_supervisor(&self) -> bool {
        self.economy_support_supervisor.is_some()
    }

    // Param is passed by value, moved
    pub fn set_economy_support_supervisor(&mut self, v: bool) {
        self.economy_support_supervisor = ::std::option::Option::Some(v);
    }

    pub fn get_economy_support_supervisor<'a>(&self) -> bool {
        self.economy_support_supervisor.unwrap_or(false)
    }

    // optional bool manage_pricing = 14;

    pub fn clear_manage_pricing(&mut self) {
        self.manage_pricing = ::std::option::Option::None;
    }

    pub fn has_manage_pricing(&self) -> bool {
        self.manage_pricing.is_some()
    }

    // Param is passed by value, moved
    pub fn set_manage_pricing(&mut self, v: bool) {
        self.manage_pricing = ::std::option::Option::Some(v);
    }

    pub fn get_manage_pricing<'a>(&self) -> bool {
        self.manage_pricing.unwrap_or(false)
    }

    // optional bool broadcast_live = 15;

    pub fn clear_broadcast_live(&mut self) {
        self.broadcast_live = ::std::option::Option::None;
    }

    pub fn has_broadcast_live(&self) -> bool {
        self.broadcast_live.is_some()
    }

    // Param is passed by value, moved
    pub fn set_broadcast_live(&mut self, v: bool) {
        self.broadcast_live = ::std::option::Option::Some(v);
    }

    pub fn get_broadcast_live<'a>(&self) -> bool {
        self.broadcast_live.unwrap_or(false)
    }
}

impl ::protobuf::Message for CMsgAppRights {
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
                    let tmp = try!(is.read_bool());
                    self.edit_info = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_bool());
                    self.publish = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_bool());
                    self.view_error_data = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_bool());
                    self.download = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_bool());
                    self.upload_cdkeys = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_bool());
                    self.generate_cdkeys = ::std::option::Option::Some(tmp);
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_bool());
                    self.view_financials = ::std::option::Option::Some(tmp);
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_bool());
                    self.manage_ceg = ::std::option::Option::Some(tmp);
                },
                9 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_bool());
                    self.manage_signing = ::std::option::Option::Some(tmp);
                },
                10 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_bool());
                    self.manage_cdkeys = ::std::option::Option::Some(tmp);
                },
                11 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_bool());
                    self.edit_marketing = ::std::option::Option::Some(tmp);
                },
                12 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_bool());
                    self.economy_support = ::std::option::Option::Some(tmp);
                },
                13 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_bool());
                    self.economy_support_supervisor = ::std::option::Option::Some(tmp);
                },
                14 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_bool());
                    self.manage_pricing = ::std::option::Option::Some(tmp);
                },
                15 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_bool());
                    self.broadcast_live = ::std::option::Option::Some(tmp);
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
        if self.edit_info.is_some() {
            my_size += 2;
        };
        if self.publish.is_some() {
            my_size += 2;
        };
        if self.view_error_data.is_some() {
            my_size += 2;
        };
        if self.download.is_some() {
            my_size += 2;
        };
        if self.upload_cdkeys.is_some() {
            my_size += 2;
        };
        if self.generate_cdkeys.is_some() {
            my_size += 2;
        };
        if self.view_financials.is_some() {
            my_size += 2;
        };
        if self.manage_ceg.is_some() {
            my_size += 2;
        };
        if self.manage_signing.is_some() {
            my_size += 2;
        };
        if self.manage_cdkeys.is_some() {
            my_size += 2;
        };
        if self.edit_marketing.is_some() {
            my_size += 2;
        };
        if self.economy_support.is_some() {
            my_size += 2;
        };
        if self.economy_support_supervisor.is_some() {
            my_size += 2;
        };
        if self.manage_pricing.is_some() {
            my_size += 2;
        };
        if self.broadcast_live.is_some() {
            my_size += 2;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.edit_info {
            try!(os.write_bool(1, v));
        };
        if let Some(v) = self.publish {
            try!(os.write_bool(2, v));
        };
        if let Some(v) = self.view_error_data {
            try!(os.write_bool(3, v));
        };
        if let Some(v) = self.download {
            try!(os.write_bool(4, v));
        };
        if let Some(v) = self.upload_cdkeys {
            try!(os.write_bool(5, v));
        };
        if let Some(v) = self.generate_cdkeys {
            try!(os.write_bool(6, v));
        };
        if let Some(v) = self.view_financials {
            try!(os.write_bool(7, v));
        };
        if let Some(v) = self.manage_ceg {
            try!(os.write_bool(8, v));
        };
        if let Some(v) = self.manage_signing {
            try!(os.write_bool(9, v));
        };
        if let Some(v) = self.manage_cdkeys {
            try!(os.write_bool(10, v));
        };
        if let Some(v) = self.edit_marketing {
            try!(os.write_bool(11, v));
        };
        if let Some(v) = self.economy_support {
            try!(os.write_bool(12, v));
        };
        if let Some(v) = self.economy_support_supervisor {
            try!(os.write_bool(13, v));
        };
        if let Some(v) = self.manage_pricing {
            try!(os.write_bool(14, v));
        };
        if let Some(v) = self.broadcast_live {
            try!(os.write_bool(15, v));
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
        ::std::any::TypeId::of::<CMsgAppRights>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CMsgAppRights {
    fn new() -> CMsgAppRights {
        CMsgAppRights::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgAppRights>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "edit_info",
                    CMsgAppRights::has_edit_info,
                    CMsgAppRights::get_edit_info,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "publish",
                    CMsgAppRights::has_publish,
                    CMsgAppRights::get_publish,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "view_error_data",
                    CMsgAppRights::has_view_error_data,
                    CMsgAppRights::get_view_error_data,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "download",
                    CMsgAppRights::has_download,
                    CMsgAppRights::get_download,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "upload_cdkeys",
                    CMsgAppRights::has_upload_cdkeys,
                    CMsgAppRights::get_upload_cdkeys,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "generate_cdkeys",
                    CMsgAppRights::has_generate_cdkeys,
                    CMsgAppRights::get_generate_cdkeys,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "view_financials",
                    CMsgAppRights::has_view_financials,
                    CMsgAppRights::get_view_financials,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "manage_ceg",
                    CMsgAppRights::has_manage_ceg,
                    CMsgAppRights::get_manage_ceg,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "manage_signing",
                    CMsgAppRights::has_manage_signing,
                    CMsgAppRights::get_manage_signing,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "manage_cdkeys",
                    CMsgAppRights::has_manage_cdkeys,
                    CMsgAppRights::get_manage_cdkeys,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "edit_marketing",
                    CMsgAppRights::has_edit_marketing,
                    CMsgAppRights::get_edit_marketing,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "economy_support",
                    CMsgAppRights::has_economy_support,
                    CMsgAppRights::get_economy_support,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "economy_support_supervisor",
                    CMsgAppRights::has_economy_support_supervisor,
                    CMsgAppRights::get_economy_support_supervisor,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "manage_pricing",
                    CMsgAppRights::has_manage_pricing,
                    CMsgAppRights::get_manage_pricing,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "broadcast_live",
                    CMsgAppRights::has_broadcast_live,
                    CMsgAppRights::get_broadcast_live,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgAppRights>(
                    "CMsgAppRights",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgAppRights {
    fn clear(&mut self) {
        self.clear_edit_info();
        self.clear_publish();
        self.clear_view_error_data();
        self.clear_download();
        self.clear_upload_cdkeys();
        self.clear_generate_cdkeys();
        self.clear_view_financials();
        self.clear_manage_ceg();
        self.clear_manage_signing();
        self.clear_manage_cdkeys();
        self.clear_edit_marketing();
        self.clear_economy_support();
        self.clear_economy_support_supervisor();
        self.clear_manage_pricing();
        self.clear_broadcast_live();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CMsgAppRights {
    fn eq(&self, other: &CMsgAppRights) -> bool {
        self.edit_info == other.edit_info &&
        self.publish == other.publish &&
        self.view_error_data == other.view_error_data &&
        self.download == other.download &&
        self.upload_cdkeys == other.upload_cdkeys &&
        self.generate_cdkeys == other.generate_cdkeys &&
        self.view_financials == other.view_financials &&
        self.manage_ceg == other.manage_ceg &&
        self.manage_signing == other.manage_signing &&
        self.manage_cdkeys == other.manage_cdkeys &&
        self.edit_marketing == other.edit_marketing &&
        self.economy_support == other.economy_support &&
        self.economy_support_supervisor == other.economy_support_supervisor &&
        self.manage_pricing == other.manage_pricing &&
        self.broadcast_live == other.broadcast_live &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for CMsgAppRights {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x18, 0x73, 0x74, 0x65, 0x61, 0x6d, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x73, 0x5f,
    0x62, 0x61, 0x73, 0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x20, 0x67, 0x6f, 0x6f, 0x67,
    0x6c, 0x65, 0x2f, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x62, 0x75, 0x66, 0x2f, 0x64, 0x65, 0x73, 0x63,
    0x72, 0x69, 0x70, 0x74, 0x6f, 0x72, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0xf8, 0x03, 0x0a,
    0x12, 0x43, 0x4d, 0x73, 0x67, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x42, 0x75, 0x66, 0x48, 0x65, 0x61,
    0x64, 0x65, 0x72, 0x12, 0x0f, 0x0a, 0x07, 0x73, 0x74, 0x65, 0x61, 0x6d, 0x69, 0x64, 0x18, 0x01,
    0x20, 0x01, 0x28, 0x06, 0x12, 0x18, 0x0a, 0x10, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x5f, 0x73,
    0x65, 0x73, 0x73, 0x69, 0x6f, 0x6e, 0x69, 0x64, 0x18, 0x02, 0x20, 0x01, 0x28, 0x05, 0x12, 0x15,
    0x0a, 0x0d, 0x72, 0x6f, 0x75, 0x74, 0x69, 0x6e, 0x67, 0x5f, 0x61, 0x70, 0x70, 0x69, 0x64, 0x18,
    0x03, 0x20, 0x01, 0x28, 0x0d, 0x12, 0x2a, 0x0a, 0x0c, 0x6a, 0x6f, 0x62, 0x69, 0x64, 0x5f, 0x73,
    0x6f, 0x75, 0x72, 0x63, 0x65, 0x18, 0x0a, 0x20, 0x01, 0x28, 0x06, 0x3a, 0x14, 0x31, 0x38, 0x34,
    0x34, 0x36, 0x37, 0x34, 0x34, 0x30, 0x37, 0x33, 0x37, 0x30, 0x39, 0x35, 0x35, 0x31, 0x36, 0x31,
    0x35, 0x12, 0x2a, 0x0a, 0x0c, 0x6a, 0x6f, 0x62, 0x69, 0x64, 0x5f, 0x74, 0x61, 0x72, 0x67, 0x65,
    0x74, 0x18, 0x0b, 0x20, 0x01, 0x28, 0x06, 0x3a, 0x14, 0x31, 0x38, 0x34, 0x34, 0x36, 0x37, 0x34,
    0x34, 0x30, 0x37, 0x33, 0x37, 0x30, 0x39, 0x35, 0x35, 0x31, 0x36, 0x31, 0x35, 0x12, 0x17, 0x0a,
    0x0f, 0x74, 0x61, 0x72, 0x67, 0x65, 0x74, 0x5f, 0x6a, 0x6f, 0x62, 0x5f, 0x6e, 0x61, 0x6d, 0x65,
    0x18, 0x0c, 0x20, 0x01, 0x28, 0x09, 0x12, 0x0f, 0x0a, 0x07, 0x73, 0x65, 0x71, 0x5f, 0x6e, 0x75,
    0x6d, 0x18, 0x18, 0x20, 0x01, 0x28, 0x05, 0x12, 0x12, 0x0a, 0x07, 0x65, 0x72, 0x65, 0x73, 0x75,
    0x6c, 0x74, 0x18, 0x0d, 0x20, 0x01, 0x28, 0x05, 0x3a, 0x01, 0x32, 0x12, 0x15, 0x0a, 0x0d, 0x65,
    0x72, 0x72, 0x6f, 0x72, 0x5f, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x18, 0x0e, 0x20, 0x01,
    0x28, 0x09, 0x12, 0x0a, 0x0a, 0x02, 0x69, 0x70, 0x18, 0x0f, 0x20, 0x01, 0x28, 0x0d, 0x12, 0x1a,
    0x0a, 0x12, 0x61, 0x75, 0x74, 0x68, 0x5f, 0x61, 0x63, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x5f, 0x66,
    0x6c, 0x61, 0x67, 0x73, 0x18, 0x10, 0x20, 0x01, 0x28, 0x0d, 0x12, 0x14, 0x0a, 0x0c, 0x74, 0x6f,
    0x6b, 0x65, 0x6e, 0x5f, 0x73, 0x6f, 0x75, 0x72, 0x63, 0x65, 0x18, 0x16, 0x20, 0x01, 0x28, 0x0d,
    0x12, 0x1b, 0x0a, 0x13, 0x61, 0x64, 0x6d, 0x69, 0x6e, 0x5f, 0x73, 0x70, 0x6f, 0x6f, 0x66, 0x69,
    0x6e, 0x67, 0x5f, 0x75, 0x73, 0x65, 0x72, 0x18, 0x17, 0x20, 0x01, 0x28, 0x08, 0x12, 0x1a, 0x0a,
    0x0f, 0x74, 0x72, 0x61, 0x6e, 0x73, 0x70, 0x6f, 0x72, 0x74, 0x5f, 0x65, 0x72, 0x72, 0x6f, 0x72,
    0x18, 0x11, 0x20, 0x01, 0x28, 0x05, 0x3a, 0x01, 0x31, 0x12, 0x27, 0x0a, 0x09, 0x6d, 0x65, 0x73,
    0x73, 0x61, 0x67, 0x65, 0x69, 0x64, 0x18, 0x12, 0x20, 0x01, 0x28, 0x04, 0x3a, 0x14, 0x31, 0x38,
    0x34, 0x34, 0x36, 0x37, 0x34, 0x34, 0x30, 0x37, 0x33, 0x37, 0x30, 0x39, 0x35, 0x35, 0x31, 0x36,
    0x31, 0x35, 0x12, 0x1a, 0x0a, 0x12, 0x70, 0x75, 0x62, 0x6c, 0x69, 0x73, 0x68, 0x65, 0x72, 0x5f,
    0x67, 0x72, 0x6f, 0x75, 0x70, 0x5f, 0x69, 0x64, 0x18, 0x13, 0x20, 0x01, 0x28, 0x0d, 0x12, 0x0d,
    0x0a, 0x05, 0x73, 0x79, 0x73, 0x69, 0x64, 0x18, 0x14, 0x20, 0x01, 0x28, 0x0d, 0x12, 0x11, 0x0a,
    0x09, 0x74, 0x72, 0x61, 0x63, 0x65, 0x5f, 0x74, 0x61, 0x67, 0x18, 0x15, 0x20, 0x01, 0x28, 0x04,
    0x12, 0x15, 0x0a, 0x0d, 0x77, 0x65, 0x62, 0x61, 0x70, 0x69, 0x5f, 0x6b, 0x65, 0x79, 0x5f, 0x69,
    0x64, 0x18, 0x19, 0x20, 0x01, 0x28, 0x0d, 0x22, 0x38, 0x0a, 0x09, 0x43, 0x4d, 0x73, 0x67, 0x4d,
    0x75, 0x6c, 0x74, 0x69, 0x12, 0x15, 0x0a, 0x0d, 0x73, 0x69, 0x7a, 0x65, 0x5f, 0x75, 0x6e, 0x7a,
    0x69, 0x70, 0x70, 0x65, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0d, 0x12, 0x14, 0x0a, 0x0c, 0x6d,
    0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x5f, 0x62, 0x6f, 0x64, 0x79, 0x18, 0x02, 0x20, 0x01, 0x28,
    0x0c, 0x22, 0x2b, 0x0a, 0x13, 0x43, 0x4d, 0x73, 0x67, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x62, 0x75,
    0x66, 0x57, 0x72, 0x61, 0x70, 0x70, 0x65, 0x64, 0x12, 0x14, 0x0a, 0x0c, 0x6d, 0x65, 0x73, 0x73,
    0x61, 0x67, 0x65, 0x5f, 0x62, 0x6f, 0x64, 0x79, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0c, 0x22, 0x8f,
    0x01, 0x0a, 0x0e, 0x43, 0x4d, 0x73, 0x67, 0x41, 0x75, 0x74, 0x68, 0x54, 0x69, 0x63, 0x6b, 0x65,
    0x74, 0x12, 0x0e, 0x0a, 0x06, 0x65, 0x73, 0x74, 0x61, 0x74, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28,
    0x0d, 0x12, 0x12, 0x0a, 0x07, 0x65, 0x72, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x18, 0x02, 0x20, 0x01,
    0x28, 0x0d, 0x3a, 0x01, 0x32, 0x12, 0x0f, 0x0a, 0x07, 0x73, 0x74, 0x65, 0x61, 0x6d, 0x69, 0x64,
    0x18, 0x03, 0x20, 0x01, 0x28, 0x06, 0x12, 0x0e, 0x0a, 0x06, 0x67, 0x61, 0x6d, 0x65, 0x69, 0x64,
    0x18, 0x04, 0x20, 0x01, 0x28, 0x06, 0x12, 0x14, 0x0a, 0x0c, 0x68, 0x5f, 0x73, 0x74, 0x65, 0x61,
    0x6d, 0x5f, 0x70, 0x69, 0x70, 0x65, 0x18, 0x05, 0x20, 0x01, 0x28, 0x0d, 0x12, 0x12, 0x0a, 0x0a,
    0x74, 0x69, 0x63, 0x6b, 0x65, 0x74, 0x5f, 0x63, 0x72, 0x63, 0x18, 0x06, 0x20, 0x01, 0x28, 0x0d,
    0x12, 0x0e, 0x0a, 0x06, 0x74, 0x69, 0x63, 0x6b, 0x65, 0x74, 0x18, 0x07, 0x20, 0x01, 0x28, 0x0c,
    0x22, 0xf6, 0x01, 0x0a, 0x14, 0x43, 0x43, 0x44, 0x44, 0x42, 0x41, 0x70, 0x70, 0x44, 0x65, 0x74,
    0x61, 0x69, 0x6c, 0x43, 0x6f, 0x6d, 0x6d, 0x6f, 0x6e, 0x12, 0x0d, 0x0a, 0x05, 0x61, 0x70, 0x70,
    0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0d, 0x12, 0x0c, 0x0a, 0x04, 0x6e, 0x61, 0x6d, 0x65,
    0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x12, 0x0c, 0x0a, 0x04, 0x69, 0x63, 0x6f, 0x6e, 0x18, 0x03,
    0x20, 0x01, 0x28, 0x09, 0x12, 0x0c, 0x0a, 0x04, 0x6c, 0x6f, 0x67, 0x6f, 0x18, 0x04, 0x20, 0x01,
    0x28, 0x09, 0x12, 0x12, 0x0a, 0x0a, 0x6c, 0x6f, 0x67, 0x6f, 0x5f, 0x73, 0x6d, 0x61, 0x6c, 0x6c,
    0x18, 0x05, 0x20, 0x01, 0x28, 0x09, 0x12, 0x0c, 0x0a, 0x04, 0x74, 0x6f, 0x6f, 0x6c, 0x18, 0x06,
    0x20, 0x01, 0x28, 0x08, 0x12, 0x0c, 0x0a, 0x04, 0x64, 0x65, 0x6d, 0x6f, 0x18, 0x07, 0x20, 0x01,
    0x28, 0x08, 0x12, 0x0d, 0x0a, 0x05, 0x6d, 0x65, 0x64, 0x69, 0x61, 0x18, 0x08, 0x20, 0x01, 0x28,
    0x08, 0x12, 0x1f, 0x0a, 0x17, 0x63, 0x6f, 0x6d, 0x6d, 0x75, 0x6e, 0x69, 0x74, 0x79, 0x5f, 0x76,
    0x69, 0x73, 0x69, 0x62, 0x6c, 0x65, 0x5f, 0x73, 0x74, 0x61, 0x74, 0x73, 0x18, 0x09, 0x20, 0x01,
    0x28, 0x08, 0x12, 0x15, 0x0a, 0x0d, 0x66, 0x72, 0x69, 0x65, 0x6e, 0x64, 0x6c, 0x79, 0x5f, 0x6e,
    0x61, 0x6d, 0x65, 0x18, 0x0a, 0x20, 0x01, 0x28, 0x09, 0x12, 0x13, 0x0a, 0x0b, 0x70, 0x72, 0x6f,
    0x70, 0x61, 0x67, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x18, 0x0b, 0x20, 0x01, 0x28, 0x09, 0x12, 0x19,
    0x0a, 0x11, 0x68, 0x61, 0x73, 0x5f, 0x61, 0x64, 0x75, 0x6c, 0x74, 0x5f, 0x63, 0x6f, 0x6e, 0x74,
    0x65, 0x6e, 0x74, 0x18, 0x0c, 0x20, 0x01, 0x28, 0x08, 0x22, 0xef, 0x02, 0x0a, 0x0d, 0x43, 0x4d,
    0x73, 0x67, 0x41, 0x70, 0x70, 0x52, 0x69, 0x67, 0x68, 0x74, 0x73, 0x12, 0x11, 0x0a, 0x09, 0x65,
    0x64, 0x69, 0x74, 0x5f, 0x69, 0x6e, 0x66, 0x6f, 0x18, 0x01, 0x20, 0x01, 0x28, 0x08, 0x12, 0x0f,
    0x0a, 0x07, 0x70, 0x75, 0x62, 0x6c, 0x69, 0x73, 0x68, 0x18, 0x02, 0x20, 0x01, 0x28, 0x08, 0x12,
    0x17, 0x0a, 0x0f, 0x76, 0x69, 0x65, 0x77, 0x5f, 0x65, 0x72, 0x72, 0x6f, 0x72, 0x5f, 0x64, 0x61,
    0x74, 0x61, 0x18, 0x03, 0x20, 0x01, 0x28, 0x08, 0x12, 0x10, 0x0a, 0x08, 0x64, 0x6f, 0x77, 0x6e,
    0x6c, 0x6f, 0x61, 0x64, 0x18, 0x04, 0x20, 0x01, 0x28, 0x08, 0x12, 0x15, 0x0a, 0x0d, 0x75, 0x70,
    0x6c, 0x6f, 0x61, 0x64, 0x5f, 0x63, 0x64, 0x6b, 0x65, 0x79, 0x73, 0x18, 0x05, 0x20, 0x01, 0x28,
    0x08, 0x12, 0x17, 0x0a, 0x0f, 0x67, 0x65, 0x6e, 0x65, 0x72, 0x61, 0x74, 0x65, 0x5f, 0x63, 0x64,
    0x6b, 0x65, 0x79, 0x73, 0x18, 0x06, 0x20, 0x01, 0x28, 0x08, 0x12, 0x17, 0x0a, 0x0f, 0x76, 0x69,
    0x65, 0x77, 0x5f, 0x66, 0x69, 0x6e, 0x61, 0x6e, 0x63, 0x69, 0x61, 0x6c, 0x73, 0x18, 0x07, 0x20,
    0x01, 0x28, 0x08, 0x12, 0x12, 0x0a, 0x0a, 0x6d, 0x61, 0x6e, 0x61, 0x67, 0x65, 0x5f, 0x63, 0x65,
    0x67, 0x18, 0x08, 0x20, 0x01, 0x28, 0x08, 0x12, 0x16, 0x0a, 0x0e, 0x6d, 0x61, 0x6e, 0x61, 0x67,
    0x65, 0x5f, 0x73, 0x69, 0x67, 0x6e, 0x69, 0x6e, 0x67, 0x18, 0x09, 0x20, 0x01, 0x28, 0x08, 0x12,
    0x15, 0x0a, 0x0d, 0x6d, 0x61, 0x6e, 0x61, 0x67, 0x65, 0x5f, 0x63, 0x64, 0x6b, 0x65, 0x79, 0x73,
    0x18, 0x0a, 0x20, 0x01, 0x28, 0x08, 0x12, 0x16, 0x0a, 0x0e, 0x65, 0x64, 0x69, 0x74, 0x5f, 0x6d,
    0x61, 0x72, 0x6b, 0x65, 0x74, 0x69, 0x6e, 0x67, 0x18, 0x0b, 0x20, 0x01, 0x28, 0x08, 0x12, 0x17,
    0x0a, 0x0f, 0x65, 0x63, 0x6f, 0x6e, 0x6f, 0x6d, 0x79, 0x5f, 0x73, 0x75, 0x70, 0x70, 0x6f, 0x72,
    0x74, 0x18, 0x0c, 0x20, 0x01, 0x28, 0x08, 0x12, 0x22, 0x0a, 0x1a, 0x65, 0x63, 0x6f, 0x6e, 0x6f,
    0x6d, 0x79, 0x5f, 0x73, 0x75, 0x70, 0x70, 0x6f, 0x72, 0x74, 0x5f, 0x73, 0x75, 0x70, 0x65, 0x72,
    0x76, 0x69, 0x73, 0x6f, 0x72, 0x18, 0x0d, 0x20, 0x01, 0x28, 0x08, 0x12, 0x16, 0x0a, 0x0e, 0x6d,
    0x61, 0x6e, 0x61, 0x67, 0x65, 0x5f, 0x70, 0x72, 0x69, 0x63, 0x69, 0x6e, 0x67, 0x18, 0x0e, 0x20,
    0x01, 0x28, 0x08, 0x12, 0x16, 0x0a, 0x0e, 0x62, 0x72, 0x6f, 0x61, 0x64, 0x63, 0x61, 0x73, 0x74,
    0x5f, 0x6c, 0x69, 0x76, 0x65, 0x18, 0x0f, 0x20, 0x01, 0x28, 0x08, 0x3a, 0x41, 0x0a, 0x12, 0x6d,
    0x73, 0x67, 0x70, 0x6f, 0x6f, 0x6c, 0x5f, 0x73, 0x6f, 0x66, 0x74, 0x5f, 0x6c, 0x69, 0x6d, 0x69,
    0x74, 0x18, 0xd0, 0x86, 0x03, 0x20, 0x01, 0x28, 0x05, 0x12, 0x1f, 0x2e, 0x67, 0x6f, 0x6f, 0x67,
    0x6c, 0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x62, 0x75, 0x66, 0x2e, 0x4d, 0x65, 0x73, 0x73,
    0x61, 0x67, 0x65, 0x4f, 0x70, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x3a, 0x02, 0x33, 0x32, 0x3a, 0x42,
    0x0a, 0x12, 0x6d, 0x73, 0x67, 0x70, 0x6f, 0x6f, 0x6c, 0x5f, 0x68, 0x61, 0x72, 0x64, 0x5f, 0x6c,
    0x69, 0x6d, 0x69, 0x74, 0x18, 0xd1, 0x86, 0x03, 0x20, 0x01, 0x28, 0x05, 0x12, 0x1f, 0x2e, 0x67,
    0x6f, 0x6f, 0x67, 0x6c, 0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x62, 0x75, 0x66, 0x2e, 0x4d,
    0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x4f, 0x70, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x3a, 0x03, 0x33,
    0x38, 0x34, 0x42, 0x05, 0x48, 0x01, 0x80, 0x01, 0x00, 0x4a, 0xd0, 0x23, 0x0a, 0x06, 0x12, 0x04,
    0x00, 0x00, 0x52, 0x01, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x00, 0x12, 0x03, 0x00, 0x07, 0x29, 0x0a,
    0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x02, 0x00, 0x1c, 0x0a, 0x0b, 0x0a, 0x04, 0x08, 0xe7, 0x07,
    0x00, 0x12, 0x03, 0x02, 0x00, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x12,
    0x03, 0x02, 0x07, 0x13, 0x0a, 0x0d, 0x0a, 0x06, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x12, 0x03,
    0x02, 0x07, 0x13, 0x0a, 0x0e, 0x0a, 0x07, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03,
    0x02, 0x07, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x08, 0xe7, 0x07, 0x00, 0x03, 0x12, 0x03, 0x02, 0x16,
    0x1b, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x03, 0x00, 0x23, 0x0a, 0x0b, 0x0a, 0x04, 0x08,
    0xe7, 0x07, 0x01, 0x12, 0x03, 0x03, 0x00, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x08, 0xe7, 0x07, 0x01,
    0x02, 0x12, 0x03, 0x03, 0x07, 0x1a, 0x0a, 0x0d, 0x0a, 0x06, 0x08, 0xe7, 0x07, 0x01, 0x02, 0x00,
    0x12, 0x03, 0x03, 0x07, 0x1a, 0x0a, 0x0e, 0x0a, 0x07, 0x08, 0xe7, 0x07, 0x01, 0x02, 0x00, 0x01,
    0x12, 0x03, 0x03, 0x07, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x08, 0xe7, 0x07, 0x01, 0x03, 0x12, 0x03,
    0x03, 0x1d, 0x22, 0x0a, 0x09, 0x0a, 0x01, 0x07, 0x12, 0x04, 0x05, 0x00, 0x08, 0x01, 0x0a, 0x09,
    0x0a, 0x02, 0x07, 0x00, 0x12, 0x03, 0x06, 0x08, 0x41, 0x0a, 0x0a, 0x0a, 0x03, 0x07, 0x00, 0x02,
    0x12, 0x03, 0x05, 0x07, 0x26, 0x0a, 0x0a, 0x0a, 0x03, 0x07, 0x00, 0x04, 0x12, 0x03, 0x06, 0x08,
    0x10, 0x0a, 0x0a, 0x0a, 0x03, 0x07, 0x00, 0x05, 0x12, 0x03, 0x06, 0x11, 0x16, 0x0a, 0x0a, 0x0a,
    0x03, 0x07, 0x00, 0x01, 0x12, 0x03, 0x06, 0x17, 0x29, 0x0a, 0x0a, 0x0a, 0x03, 0x07, 0x00, 0x03,
    0x12, 0x03, 0x06, 0x2c, 0x31, 0x0a, 0x0a, 0x0a, 0x03, 0x07, 0x00, 0x08, 0x12, 0x03, 0x06, 0x32,
    0x40, 0x0a, 0x0a, 0x0a, 0x03, 0x07, 0x00, 0x07, 0x12, 0x03, 0x06, 0x3d, 0x3f, 0x0a, 0x09, 0x0a,
    0x02, 0x07, 0x01, 0x12, 0x03, 0x07, 0x08, 0x42, 0x0a, 0x0a, 0x0a, 0x03, 0x07, 0x01, 0x02, 0x12,
    0x03, 0x05, 0x07, 0x26, 0x0a, 0x0a, 0x0a, 0x03, 0x07, 0x01, 0x04, 0x12, 0x03, 0x07, 0x08, 0x10,
    0x0a, 0x0a, 0x0a, 0x03, 0x07, 0x01, 0x05, 0x12, 0x03, 0x07, 0x11, 0x16, 0x0a, 0x0a, 0x0a, 0x03,
    0x07, 0x01, 0x01, 0x12, 0x03, 0x07, 0x17, 0x29, 0x0a, 0x0a, 0x0a, 0x03, 0x07, 0x01, 0x03, 0x12,
    0x03, 0x07, 0x2c, 0x31, 0x0a, 0x0a, 0x0a, 0x03, 0x07, 0x01, 0x08, 0x12, 0x03, 0x07, 0x32, 0x41,
    0x0a, 0x0a, 0x0a, 0x03, 0x07, 0x01, 0x07, 0x12, 0x03, 0x07, 0x3d, 0x40, 0x0a, 0x0a, 0x0a, 0x02,
    0x04, 0x00, 0x12, 0x04, 0x0a, 0x00, 0x1e, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12,
    0x03, 0x0a, 0x08, 0x1a, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x0b, 0x08,
    0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x04, 0x12, 0x03, 0x0b, 0x08, 0x10, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x05, 0x12, 0x03, 0x0b, 0x11, 0x18, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x0b, 0x19, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x0b, 0x23, 0x24, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02,
    0x01, 0x12, 0x03, 0x0c, 0x08, 0x2c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x04, 0x12,
    0x03, 0x0c, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x05, 0x12, 0x03, 0x0c,
    0x11, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x0c, 0x17, 0x27,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x0c, 0x2a, 0x2b, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x00, 0x02, 0x02, 0x12, 0x03, 0x0d, 0x08, 0x2a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x02, 0x04, 0x12, 0x03, 0x0d, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x02, 0x05, 0x12, 0x03, 0x0d, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x01,
    0x12, 0x03, 0x0d, 0x18, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x03, 0x12, 0x03,
    0x0d, 0x28, 0x29, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x03, 0x12, 0x03, 0x0e, 0x08, 0x4c,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x04, 0x12, 0x03, 0x0e, 0x08, 0x10, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x05, 0x12, 0x03, 0x0e, 0x11, 0x18, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x03, 0x01, 0x12, 0x03, 0x0e, 0x19, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x03, 0x03, 0x12, 0x03, 0x0e, 0x28, 0x2a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03,
    0x08, 0x12, 0x03, 0x0e, 0x2b, 0x4b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x07, 0x12,
    0x03, 0x0e, 0x36, 0x4a, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x04, 0x12, 0x03, 0x0f, 0x08,
    0x4c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x04, 0x12, 0x03, 0x0f, 0x08, 0x10, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x05, 0x12, 0x03, 0x0f, 0x11, 0x18, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x04, 0x01, 0x12, 0x03, 0x0f, 0x19, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x04, 0x03, 0x12, 0x03, 0x0f, 0x28, 0x2a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x04, 0x08, 0x12, 0x03, 0x0f, 0x2b, 0x4b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x07,
    0x12, 0x03, 0x0f, 0x36, 0x4a, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x05, 0x12, 0x03, 0x10,
    0x08, 0x2d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x05, 0x04, 0x12, 0x03, 0x10, 0x08, 0x10,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x05, 0x05, 0x12, 0x03, 0x10, 0x11, 0x17, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x05, 0x01, 0x12, 0x03, 0x10, 0x18, 0x27, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x05, 0x03, 0x12, 0x03, 0x10, 0x2a, 0x2c, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00,
    0x02, 0x06, 0x12, 0x03, 0x11, 0x08, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x06, 0x04,
    0x12, 0x03, 0x11, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x06, 0x05, 0x12, 0x03,
    0x11, 0x11, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x06, 0x01, 0x12, 0x03, 0x11, 0x17,
    0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x06, 0x03, 0x12, 0x03, 0x11, 0x21, 0x23, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x07, 0x12, 0x03, 0x12, 0x08, 0x32, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x07, 0x04, 0x12, 0x03, 0x12, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x07, 0x05, 0x12, 0x03, 0x12, 0x11, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x07,
    0x01, 0x12, 0x03, 0x12, 0x17, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x07, 0x03, 0x12,
    0x03, 0x12, 0x21, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x07, 0x08, 0x12, 0x03, 0x12,
    0x24, 0x31, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x07, 0x07, 0x12, 0x03, 0x12, 0x2f, 0x30,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x08, 0x12, 0x03, 0x13, 0x08, 0x2b, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x08, 0x04, 0x12, 0x03, 0x13, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x08, 0x05, 0x12, 0x03, 0x13, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x08, 0x01, 0x12, 0x03, 0x13, 0x18, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x08, 0x03,
    0x12, 0x03, 0x13, 0x28, 0x2a, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x09, 0x12, 0x03, 0x14,
    0x08, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x09, 0x04, 0x12, 0x03, 0x14, 0x08, 0x10,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x09, 0x05, 0x12, 0x03, 0x14, 0x11, 0x17, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x09, 0x01, 0x12, 0x03, 0x14, 0x18, 0x1a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x09, 0x03, 0x12, 0x03, 0x14, 0x1d, 0x1f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00,
    0x02, 0x0a, 0x12, 0x03, 0x15, 0x08, 0x30, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0a, 0x04,
    0x12, 0x03, 0x15, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0a, 0x05, 0x12, 0x03,
    0x15, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0a, 0x01, 0x12, 0x03, 0x15, 0x18,
    0x2a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0a, 0x03, 0x12, 0x03, 0x15, 0x2d, 0x2f, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x0b, 0x12, 0x03, 0x16, 0x08, 0x2a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x0b, 0x04, 0x12, 0x03, 0x16, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x0b, 0x05, 0x12, 0x03, 0x16, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0b,
    0x01, 0x12, 0x03, 0x16, 0x18, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0b, 0x03, 0x12,
    0x03, 0x16, 0x27, 0x29, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x0c, 0x12, 0x03, 0x17, 0x08,
    0x2f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0c, 0x04, 0x12, 0x03, 0x17, 0x08, 0x10, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0c, 0x05, 0x12, 0x03, 0x17, 0x11, 0x15, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x0c, 0x01, 0x12, 0x03, 0x17, 0x16, 0x29, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x0c, 0x03, 0x12, 0x03, 0x17, 0x2c, 0x2e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02,
    0x0d, 0x12, 0x03, 0x18, 0x08, 0x3a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0d, 0x04, 0x12,
    0x03, 0x18, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0d, 0x05, 0x12, 0x03, 0x18,
    0x11, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0d, 0x01, 0x12, 0x03, 0x18, 0x17, 0x26,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0d, 0x03, 0x12, 0x03, 0x18, 0x29, 0x2b, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x0d, 0x08, 0x12, 0x03, 0x18, 0x2c, 0x39, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x0d, 0x07, 0x12, 0x03, 0x18, 0x37, 0x38, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00,
    0x02, 0x0e, 0x12, 0x03, 0x19, 0x08, 0x48, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0e, 0x04,
    0x12, 0x03, 0x19, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0e, 0x05, 0x12, 0x03,
    0x19, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0e, 0x01, 0x12, 0x03, 0x19, 0x18,
    0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0e, 0x03, 0x12, 0x03, 0x19, 0x24, 0x26, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0e, 0x08, 0x12, 0x03, 0x19, 0x27, 0x47, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x0e, 0x07, 0x12, 0x03, 0x19, 0x32, 0x46, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x00, 0x02, 0x0f, 0x12, 0x03, 0x1a, 0x08, 0x30, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0f,
    0x04, 0x12, 0x03, 0x1a, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0f, 0x05, 0x12,
    0x03, 0x1a, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0f, 0x01, 0x12, 0x03, 0x1a,
    0x18, 0x2a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0f, 0x03, 0x12, 0x03, 0x1a, 0x2d, 0x2f,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x10, 0x12, 0x03, 0x1b, 0x08, 0x23, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x10, 0x04, 0x12, 0x03, 0x1b, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x10, 0x05, 0x12, 0x03, 0x1b, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x10, 0x01, 0x12, 0x03, 0x1b, 0x18, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x10, 0x03,
    0x12, 0x03, 0x1b, 0x20, 0x22, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x11, 0x12, 0x03, 0x1c,
    0x08, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x11, 0x04, 0x12, 0x03, 0x1c, 0x08, 0x10,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x11, 0x05, 0x12, 0x03, 0x1c, 0x11, 0x17, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x11, 0x01, 0x12, 0x03, 0x1c, 0x18, 0x21, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x11, 0x03, 0x12, 0x03, 0x1c, 0x24, 0x26, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00,
    0x02, 0x12, 0x12, 0x03, 0x1d, 0x08, 0x2b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x12, 0x04,
    0x12, 0x03, 0x1d, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x12, 0x05, 0x12, 0x03,
    0x1d, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x12, 0x01, 0x12, 0x03, 0x1d, 0x18,
    0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x12, 0x03, 0x12, 0x03, 0x1d, 0x28, 0x2a, 0x0a,
    0x0a, 0x0a, 0x02, 0x04, 0x01, 0x12, 0x04, 0x20, 0x00, 0x23, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04,
    0x01, 0x01, 0x12, 0x03, 0x20, 0x08, 0x11, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x00, 0x12,
    0x03, 0x21, 0x08, 0x2a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x04, 0x12, 0x03, 0x21,
    0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x05, 0x12, 0x03, 0x21, 0x11, 0x17,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x01, 0x12, 0x03, 0x21, 0x18, 0x25, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x03, 0x12, 0x03, 0x21, 0x28, 0x29, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x01, 0x02, 0x01, 0x12, 0x03, 0x22, 0x08, 0x28, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02,
    0x01, 0x04, 0x12, 0x03, 0x22, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x05,
    0x12, 0x03, 0x22, 0x11, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x01, 0x12, 0x03,
    0x22, 0x17, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x03, 0x12, 0x03, 0x22, 0x26,
    0x27, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x02, 0x12, 0x04, 0x25, 0x00, 0x27, 0x01, 0x0a, 0x0a, 0x0a,
    0x03, 0x04, 0x02, 0x01, 0x12, 0x03, 0x25, 0x08, 0x1b, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02,
    0x00, 0x12, 0x03, 0x26, 0x08, 0x28, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x04, 0x12,
    0x03, 0x26, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x05, 0x12, 0x03, 0x26,
    0x11, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x01, 0x12, 0x03, 0x26, 0x17, 0x23,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x03, 0x12, 0x03, 0x26, 0x26, 0x27, 0x0a, 0x0a,
    0x0a, 0x02, 0x04, 0x03, 0x12, 0x04, 0x29, 0x00, 0x31, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x03,
    0x01, 0x12, 0x03, 0x29, 0x08, 0x16, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x00, 0x12, 0x03,
    0x2a, 0x08, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x04, 0x12, 0x03, 0x2a, 0x08,
    0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x05, 0x12, 0x03, 0x2a, 0x11, 0x17, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x01, 0x12, 0x03, 0x2a, 0x18, 0x1e, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x03, 0x02, 0x00, 0x03, 0x12, 0x03, 0x2a, 0x21, 0x22, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x03, 0x02, 0x01, 0x12, 0x03, 0x2b, 0x08, 0x32, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01,
    0x04, 0x12, 0x03, 0x2b, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x05, 0x12,
    0x03, 0x2b, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x01, 0x12, 0x03, 0x2b,
    0x18, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x03, 0x12, 0x03, 0x2b, 0x22, 0x23,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x08, 0x12, 0x03, 0x2b, 0x24, 0x31, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x07, 0x12, 0x03, 0x2b, 0x2f, 0x30, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x03, 0x02, 0x02, 0x12, 0x03, 0x2c, 0x08, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02,
    0x02, 0x04, 0x12, 0x03, 0x2c, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x02, 0x05,
    0x12, 0x03, 0x2c, 0x11, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x02, 0x01, 0x12, 0x03,
    0x2c, 0x19, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x02, 0x03, 0x12, 0x03, 0x2c, 0x23,
    0x24, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x03, 0x12, 0x03, 0x2d, 0x08, 0x24, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x03, 0x02, 0x03, 0x04, 0x12, 0x03, 0x2d, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x03, 0x02, 0x03, 0x05, 0x12, 0x03, 0x2d, 0x11, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03,
    0x02, 0x03, 0x01, 0x12, 0x03, 0x2d, 0x19, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x03,
    0x03, 0x12, 0x03, 0x2d, 0x22, 0x23, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x04, 0x12, 0x03,
    0x2e, 0x08, 0x29, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x04, 0x04, 0x12, 0x03, 0x2e, 0x08,
    0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x04, 0x05, 0x12, 0x03, 0x2e, 0x11, 0x17, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x04, 0x01, 0x12, 0x03, 0x2e, 0x18, 0x24, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x03, 0x02, 0x04, 0x03, 0x12, 0x03, 0x2e, 0x27, 0x28, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x03, 0x02, 0x05, 0x12, 0x03, 0x2f, 0x08, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x05,
    0x04, 0x12, 0x03, 0x2f, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x05, 0x05, 0x12,
    0x03, 0x2f, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x05, 0x01, 0x12, 0x03, 0x2f,
    0x18, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x05, 0x03, 0x12, 0x03, 0x2f, 0x25, 0x26,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x06, 0x12, 0x03, 0x30, 0x08, 0x22, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x03, 0x02, 0x06, 0x04, 0x12, 0x03, 0x30, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x03, 0x02, 0x06, 0x05, 0x12, 0x03, 0x30, 0x11, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02,
    0x06, 0x01, 0x12, 0x03, 0x30, 0x17, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x06, 0x03,
    0x12, 0x03, 0x30, 0x20, 0x21, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x04, 0x12, 0x04, 0x33, 0x00, 0x40,
    0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x04, 0x01, 0x12, 0x03, 0x33, 0x08, 0x1c, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x04, 0x02, 0x00, 0x12, 0x03, 0x34, 0x08, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04,
    0x02, 0x00, 0x04, 0x12, 0x03, 0x34, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00,
    0x05, 0x12, 0x03, 0x34, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x01, 0x12,
    0x03, 0x34, 0x18, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x03, 0x12, 0x03, 0x34,
    0x20, 0x21, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x01, 0x12, 0x03, 0x35, 0x08, 0x21, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x04, 0x12, 0x03, 0x35, 0x08, 0x10, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x04, 0x02, 0x01, 0x05, 0x12, 0x03, 0x35, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x04, 0x02, 0x01, 0x01, 0x12, 0x03, 0x35, 0x18, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02,
    0x01, 0x03, 0x12, 0x03, 0x35, 0x1f, 0x20, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x02, 0x12,
    0x03, 0x36, 0x08, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x02, 0x04, 0x12, 0x03, 0x36,
    0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x02, 0x05, 0x12, 0x03, 0x36, 0x11, 0x17,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x02, 0x01, 0x12, 0x03, 0x36, 0x18, 0x1c, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x04, 0x02, 0x02, 0x03, 0x12, 0x03, 0x36, 0x1f, 0x20, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x04, 0x02, 0x03, 0x12, 0x03, 0x37, 0x08, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02,
    0x03, 0x04, 0x12, 0x03, 0x37, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x03, 0x05,
    0x12, 0x03, 0x37, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x03, 0x01, 0x12, 0x03,
    0x37, 0x18, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x03, 0x03, 0x12, 0x03, 0x37, 0x1f,
    0x20, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x04, 0x12, 0x03, 0x38, 0x08, 0x27, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x04, 0x02, 0x04, 0x04, 0x12, 0x03, 0x38, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x04, 0x02, 0x04, 0x05, 0x12, 0x03, 0x38, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04,
    0x02, 0x04, 0x01, 0x12, 0x03, 0x38, 0x18, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x04,
    0x03, 0x12, 0x03, 0x38, 0x25, 0x26, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x05, 0x12, 0x03,
    0x39, 0x08, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x05, 0x04, 0x12, 0x03, 0x39, 0x08,
    0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x05, 0x05, 0x12, 0x03, 0x39, 0x11, 0x15, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x05, 0x01, 0x12, 0x03, 0x39, 0x16, 0x1a, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x04, 0x02, 0x05, 0x03, 0x12, 0x03, 0x39, 0x1d, 0x1e, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x04, 0x02, 0x06, 0x12, 0x03, 0x3a, 0x08, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x06,
    0x04, 0x12, 0x03, 0x3a, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x06, 0x05, 0x12,
    0x03, 0x3a, 0x11, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x06, 0x01, 0x12, 0x03, 0x3a,
    0x16, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x06, 0x03, 0x12, 0x03, 0x3a, 0x1d, 0x1e,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x07, 0x12, 0x03, 0x3b, 0x08, 0x20, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x04, 0x02, 0x07, 0x04, 0x12, 0x03, 0x3b, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x04, 0x02, 0x07, 0x05, 0x12, 0x03, 0x3b, 0x11, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02,
    0x07, 0x01, 0x12, 0x03, 0x3b, 0x16, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x07, 0x03,
    0x12, 0x03, 0x3b, 0x1e, 0x1f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x08, 0x12, 0x03, 0x3c,
    0x08, 0x32, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x08, 0x04, 0x12, 0x03, 0x3c, 0x08, 0x10,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x08, 0x05, 0x12, 0x03, 0x3c, 0x11, 0x15, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x04, 0x02, 0x08, 0x01, 0x12, 0x03, 0x3c, 0x16, 0x2d, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x04, 0x02, 0x08, 0x03, 0x12, 0x03, 0x3c, 0x30, 0x31, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04,
    0x02, 0x09, 0x12, 0x03, 0x3d, 0x08, 0x2b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x09, 0x04,
    0x12, 0x03, 0x3d, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x09, 0x05, 0x12, 0x03,
    0x3d, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x09, 0x01, 0x12, 0x03, 0x3d, 0x18,
    0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x09, 0x03, 0x12, 0x03, 0x3d, 0x28, 0x2a, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x0a, 0x12, 0x03, 0x3e, 0x08, 0x29, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x04, 0x02, 0x0a, 0x04, 0x12, 0x03, 0x3e, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04,
    0x02, 0x0a, 0x05, 0x12, 0x03, 0x3e, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x0a,
    0x01, 0x12, 0x03, 0x3e, 0x18, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x0a, 0x03, 0x12,
    0x03, 0x3e, 0x26, 0x28, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x0b, 0x12, 0x03, 0x3f, 0x08,
    0x2d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x0b, 0x04, 0x12, 0x03, 0x3f, 0x08, 0x10, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x0b, 0x05, 0x12, 0x03, 0x3f, 0x11, 0x15, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x04, 0x02, 0x0b, 0x01, 0x12, 0x03, 0x3f, 0x16, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x04, 0x02, 0x0b, 0x03, 0x12, 0x03, 0x3f, 0x2a, 0x2c, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x05, 0x12,
    0x04, 0x42, 0x00, 0x52, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x05, 0x01, 0x12, 0x03, 0x42, 0x08,
    0x15, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x00, 0x12, 0x03, 0x43, 0x08, 0x24, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x04, 0x12, 0x03, 0x43, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x05, 0x02, 0x00, 0x05, 0x12, 0x03, 0x43, 0x11, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05,
    0x02, 0x00, 0x01, 0x12, 0x03, 0x43, 0x16, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00,
    0x03, 0x12, 0x03, 0x43, 0x22, 0x23, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x01, 0x12, 0x03,
    0x44, 0x08, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x01, 0x04, 0x12, 0x03, 0x44, 0x08,
    0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x01, 0x05, 0x12, 0x03, 0x44, 0x11, 0x15, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x01, 0x01, 0x12, 0x03, 0x44, 0x16, 0x1d, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x05, 0x02, 0x01, 0x03, 0x12, 0x03, 0x44, 0x20, 0x21, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x05, 0x02, 0x02, 0x12, 0x03, 0x45, 0x08, 0x2a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x02,
    0x04, 0x12, 0x03, 0x45, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x02, 0x05, 0x12,
    0x03, 0x45, 0x11, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x02, 0x01, 0x12, 0x03, 0x45,
    0x16, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x02, 0x03, 0x12, 0x03, 0x45, 0x28, 0x29,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x03, 0x12, 0x03, 0x46, 0x08, 0x23, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x05, 0x02, 0x03, 0x04, 0x12, 0x03, 0x46, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x05, 0x02, 0x03, 0x05, 0x12, 0x03, 0x46, 0x11, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02,
    0x03, 0x01, 0x12, 0x03, 0x46, 0x16, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x03, 0x03,
    0x12, 0x03, 0x46, 0x21, 0x22, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x04, 0x12, 0x03, 0x47,
    0x08, 0x28, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x04, 0x04, 0x12, 0x03, 0x47, 0x08, 0x10,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x04, 0x05, 0x12, 0x03, 0x47, 0x11, 0x15, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x05, 0x02, 0x04, 0x01, 0x12, 0x03, 0x47, 0x16, 0x23, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x05, 0x02, 0x04, 0x03, 0x12, 0x03, 0x47, 0x26, 0x27, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x05,
    0x02, 0x05, 0x12, 0x03, 0x48, 0x08, 0x2a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x05, 0x04,
    0x12, 0x03, 0x48, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x05, 0x05, 0x12, 0x03,
    0x48, 0x11, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x05, 0x01, 0x12, 0x03, 0x48, 0x16,
    0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x05, 0x03, 0x12, 0x03, 0x48, 0x28, 0x29, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x06, 0x12, 0x03, 0x49, 0x08, 0x2a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x05, 0x02, 0x06, 0x04, 0x12, 0x03, 0x49, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05,
    0x02, 0x06, 0x05, 0x12, 0x03, 0x49, 0x11, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x06,
    0x01, 0x12, 0x03, 0x49, 0x16, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x06, 0x03, 0x12,
    0x03, 0x49, 0x28, 0x29, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x07, 0x12, 0x03, 0x4a, 0x08,
    0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x07, 0x04, 0x12, 0x03, 0x4a, 0x08, 0x10, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x07, 0x05, 0x12, 0x03, 0x4a, 0x11, 0x15, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x05, 0x02, 0x07, 0x01, 0x12, 0x03, 0x4a, 0x16, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x05, 0x02, 0x07, 0x03, 0x12, 0x03, 0x4a, 0x23, 0x24, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x05, 0x02,
    0x08, 0x12, 0x03, 0x4b, 0x08, 0x29, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x08, 0x04, 0x12,
    0x03, 0x4b, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x08, 0x05, 0x12, 0x03, 0x4b,
    0x11, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x08, 0x01, 0x12, 0x03, 0x4b, 0x16, 0x24,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x08, 0x03, 0x12, 0x03, 0x4b, 0x27, 0x28, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x05, 0x02, 0x09, 0x12, 0x03, 0x4c, 0x08, 0x29, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x05, 0x02, 0x09, 0x04, 0x12, 0x03, 0x4c, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02,
    0x09, 0x05, 0x12, 0x03, 0x4c, 0x11, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x09, 0x01,
    0x12, 0x03, 0x4c, 0x16, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x09, 0x03, 0x12, 0x03,
    0x4c, 0x26, 0x28, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x0a, 0x12, 0x03, 0x4d, 0x08, 0x2a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x0a, 0x04, 0x12, 0x03, 0x4d, 0x08, 0x10, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x05, 0x02, 0x0a, 0x05, 0x12, 0x03, 0x4d, 0x11, 0x15, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x05, 0x02, 0x0a, 0x01, 0x12, 0x03, 0x4d, 0x16, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05,
    0x02, 0x0a, 0x03, 0x12, 0x03, 0x4d, 0x27, 0x29, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x0b,
    0x12, 0x03, 0x4e, 0x08, 0x2b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x0b, 0x04, 0x12, 0x03,
    0x4e, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x0b, 0x05, 0x12, 0x03, 0x4e, 0x11,
    0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x0b, 0x01, 0x12, 0x03, 0x4e, 0x16, 0x25, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x0b, 0x03, 0x12, 0x03, 0x4e, 0x28, 0x2a, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x05, 0x02, 0x0c, 0x12, 0x03, 0x4f, 0x08, 0x36, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05,
    0x02, 0x0c, 0x04, 0x12, 0x03, 0x4f, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x0c,
    0x05, 0x12, 0x03, 0x4f, 0x11, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x0c, 0x01, 0x12,
    0x03, 0x4f, 0x16, 0x30, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x0c, 0x03, 0x12, 0x03, 0x4f,
    0x33, 0x35, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x0d, 0x12, 0x03, 0x50, 0x08, 0x2a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x0d, 0x04, 0x12, 0x03, 0x50, 0x08, 0x10, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x05, 0x02, 0x0d, 0x05, 0x12, 0x03, 0x50, 0x11, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x05, 0x02, 0x0d, 0x01, 0x12, 0x03, 0x50, 0x16, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02,
    0x0d, 0x03, 0x12, 0x03, 0x50, 0x27, 0x29, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x0e, 0x12,
    0x03, 0x51, 0x08, 0x2a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x0e, 0x04, 0x12, 0x03, 0x51,
    0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x0e, 0x05, 0x12, 0x03, 0x51, 0x11, 0x15,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x0e, 0x01, 0x12, 0x03, 0x51, 0x16, 0x24, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x05, 0x02, 0x0e, 0x03, 0x12, 0x03, 0x51, 0x27, 0x29,
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
