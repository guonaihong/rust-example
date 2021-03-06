// This file is generated by rust-protobuf 2.18.1. Do not edit
// @generated

// https://github.com/rust-lang/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![allow(unused_attributes)]
#![rustfmt::skip]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unused_imports)]
#![allow(unused_results)]
//! Generated file from `small-model.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_18_1;

#[derive(PartialEq,Clone,Default)]
pub struct HotWordRequest {
    // message fields
    pub model_type: ::std::string::String,
    pub model_text: ::std::string::String,
    pub session_id: ::std::string::String,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a HotWordRequest {
    fn default() -> &'a HotWordRequest {
        <HotWordRequest as ::protobuf::Message>::default_instance()
    }
}

impl HotWordRequest {
    pub fn new() -> HotWordRequest {
        ::std::default::Default::default()
    }

    // string model_type = 1;


    pub fn get_model_type(&self) -> &str {
        &self.model_type
    }
    pub fn clear_model_type(&mut self) {
        self.model_type.clear();
    }

    // Param is passed by value, moved
    pub fn set_model_type(&mut self, v: ::std::string::String) {
        self.model_type = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_model_type(&mut self) -> &mut ::std::string::String {
        &mut self.model_type
    }

    // Take field
    pub fn take_model_type(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.model_type, ::std::string::String::new())
    }

    // string model_text = 2;


    pub fn get_model_text(&self) -> &str {
        &self.model_text
    }
    pub fn clear_model_text(&mut self) {
        self.model_text.clear();
    }

    // Param is passed by value, moved
    pub fn set_model_text(&mut self, v: ::std::string::String) {
        self.model_text = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_model_text(&mut self) -> &mut ::std::string::String {
        &mut self.model_text
    }

    // Take field
    pub fn take_model_text(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.model_text, ::std::string::String::new())
    }

    // string session_id = 3;


    pub fn get_session_id(&self) -> &str {
        &self.session_id
    }
    pub fn clear_session_id(&mut self) {
        self.session_id.clear();
    }

    // Param is passed by value, moved
    pub fn set_session_id(&mut self, v: ::std::string::String) {
        self.session_id = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_session_id(&mut self) -> &mut ::std::string::String {
        &mut self.session_id
    }

    // Take field
    pub fn take_session_id(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.session_id, ::std::string::String::new())
    }
}

impl ::protobuf::Message for HotWordRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.model_type)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.model_text)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.session_id)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if !self.model_type.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.model_type);
        }
        if !self.model_text.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.model_text);
        }
        if !self.session_id.is_empty() {
            my_size += ::protobuf::rt::string_size(3, &self.session_id);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if !self.model_type.is_empty() {
            os.write_string(1, &self.model_type)?;
        }
        if !self.model_text.is_empty() {
            os.write_string(2, &self.model_text)?;
        }
        if !self.session_id.is_empty() {
            os.write_string(3, &self.session_id)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: ::std::boxed::Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> HotWordRequest {
        HotWordRequest::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "model_type",
                |m: &HotWordRequest| { &m.model_type },
                |m: &mut HotWordRequest| { &mut m.model_type },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "model_text",
                |m: &HotWordRequest| { &m.model_text },
                |m: &mut HotWordRequest| { &mut m.model_text },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "session_id",
                |m: &HotWordRequest| { &m.session_id },
                |m: &mut HotWordRequest| { &mut m.session_id },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<HotWordRequest>(
                "HotWordRequest",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static HotWordRequest {
        static instance: ::protobuf::rt::LazyV2<HotWordRequest> = ::protobuf::rt::LazyV2::INIT;
        instance.get(HotWordRequest::new)
    }
}

impl ::protobuf::Clear for HotWordRequest {
    fn clear(&mut self) {
        self.model_type.clear();
        self.model_text.clear();
        self.session_id.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for HotWordRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for HotWordRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct HotWordResponse {
    // message fields
    pub errcode: i64,
    pub errmsg: ::std::string::String,
    pub session_id: ::std::string::String,
    pub gen_model: ::std::vec::Vec<u8>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a HotWordResponse {
    fn default() -> &'a HotWordResponse {
        <HotWordResponse as ::protobuf::Message>::default_instance()
    }
}

impl HotWordResponse {
    pub fn new() -> HotWordResponse {
        ::std::default::Default::default()
    }

    // int64 errcode = 1;


    pub fn get_errcode(&self) -> i64 {
        self.errcode
    }
    pub fn clear_errcode(&mut self) {
        self.errcode = 0;
    }

    // Param is passed by value, moved
    pub fn set_errcode(&mut self, v: i64) {
        self.errcode = v;
    }

    // string errmsg = 2;


    pub fn get_errmsg(&self) -> &str {
        &self.errmsg
    }
    pub fn clear_errmsg(&mut self) {
        self.errmsg.clear();
    }

    // Param is passed by value, moved
    pub fn set_errmsg(&mut self, v: ::std::string::String) {
        self.errmsg = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_errmsg(&mut self) -> &mut ::std::string::String {
        &mut self.errmsg
    }

    // Take field
    pub fn take_errmsg(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.errmsg, ::std::string::String::new())
    }

    // string session_id = 3;


    pub fn get_session_id(&self) -> &str {
        &self.session_id
    }
    pub fn clear_session_id(&mut self) {
        self.session_id.clear();
    }

    // Param is passed by value, moved
    pub fn set_session_id(&mut self, v: ::std::string::String) {
        self.session_id = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_session_id(&mut self) -> &mut ::std::string::String {
        &mut self.session_id
    }

    // Take field
    pub fn take_session_id(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.session_id, ::std::string::String::new())
    }

    // bytes gen_model = 4;


    pub fn get_gen_model(&self) -> &[u8] {
        &self.gen_model
    }
    pub fn clear_gen_model(&mut self) {
        self.gen_model.clear();
    }

    // Param is passed by value, moved
    pub fn set_gen_model(&mut self, v: ::std::vec::Vec<u8>) {
        self.gen_model = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_gen_model(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.gen_model
    }

    // Take field
    pub fn take_gen_model(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.gen_model, ::std::vec::Vec::new())
    }
}

impl ::protobuf::Message for HotWordResponse {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.errcode = tmp;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.errmsg)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.session_id)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.gen_model)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if self.errcode != 0 {
            my_size += ::protobuf::rt::value_size(1, self.errcode, ::protobuf::wire_format::WireTypeVarint);
        }
        if !self.errmsg.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.errmsg);
        }
        if !self.session_id.is_empty() {
            my_size += ::protobuf::rt::string_size(3, &self.session_id);
        }
        if !self.gen_model.is_empty() {
            my_size += ::protobuf::rt::bytes_size(4, &self.gen_model);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if self.errcode != 0 {
            os.write_int64(1, self.errcode)?;
        }
        if !self.errmsg.is_empty() {
            os.write_string(2, &self.errmsg)?;
        }
        if !self.session_id.is_empty() {
            os.write_string(3, &self.session_id)?;
        }
        if !self.gen_model.is_empty() {
            os.write_bytes(4, &self.gen_model)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: ::std::boxed::Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> HotWordResponse {
        HotWordResponse::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                "errcode",
                |m: &HotWordResponse| { &m.errcode },
                |m: &mut HotWordResponse| { &mut m.errcode },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "errmsg",
                |m: &HotWordResponse| { &m.errmsg },
                |m: &mut HotWordResponse| { &mut m.errmsg },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "session_id",
                |m: &HotWordResponse| { &m.session_id },
                |m: &mut HotWordResponse| { &mut m.session_id },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                "gen_model",
                |m: &HotWordResponse| { &m.gen_model },
                |m: &mut HotWordResponse| { &mut m.gen_model },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<HotWordResponse>(
                "HotWordResponse",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static HotWordResponse {
        static instance: ::protobuf::rt::LazyV2<HotWordResponse> = ::protobuf::rt::LazyV2::INIT;
        instance.get(HotWordResponse::new)
    }
}

impl ::protobuf::Clear for HotWordResponse {
    fn clear(&mut self) {
        self.errcode = 0;
        self.errmsg.clear();
        self.session_id.clear();
        self.gen_model.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for HotWordResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for HotWordResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11small-model.proto\"u\n\x0eHotWordRequest\x12\x1f\n\nmodel_type\x18\
    \x01\x20\x01(\tR\tmodelTypeB\0\x12\x1f\n\nmodel_text\x18\x02\x20\x01(\tR\
    \tmodelTextB\0\x12\x1f\n\nsession_id\x18\x03\x20\x01(\tR\tsessionIdB\0:\
    \0\"\x89\x01\n\x0fHotWordResponse\x12\x1a\n\x07errcode\x18\x01\x20\x01(\
    \x03R\x07errcodeB\0\x12\x18\n\x06errmsg\x18\x02\x20\x01(\tR\x06errmsgB\0\
    \x12\x1f\n\nsession_id\x18\x03\x20\x01(\tR\tsessionIdB\0\x12\x1d\n\tgen_\
    model\x18\x04\x20\x01(\x0cR\x08genModelB\0:\0B\0b\x06proto3\
";

static file_descriptor_proto_lazy: ::protobuf::rt::LazyV2<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::rt::LazyV2::INIT;

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    file_descriptor_proto_lazy.get(|| {
        parse_descriptor_proto()
    })
}
