// This file is generated by rust-protobuf 3.0.0-alpha.13. Do not edit
// .proto file is parsed by protoc 3.19.4
// @generated

// https://github.com/rust-lang/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![allow(unused_attributes)]
#![cfg_attr(rustfmt, rustfmt::skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unused_results)]
#![allow(unused_mut)]

//! Generated file from `notification.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_0_0_ALPHA_13;

#[derive(PartialEq,Clone,Default,Debug)]
// @@protoc_insertion_point(message:StatusRequest)
pub struct StatusRequest {
    // message fields
    // @@protoc_insertion_point(field:StatusRequest.status)
    pub status: ::protobuf::EnumOrUnknown<StatusNotification>,
    // special fields
    // @@protoc_insertion_point(special_field:StatusRequest.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a StatusRequest {
    fn default() -> &'a StatusRequest {
        <StatusRequest as ::protobuf::Message>::default_instance()
    }
}

impl StatusRequest {
    pub fn new() -> StatusRequest {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(1);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "status",
            |m: &StatusRequest| { &m.status },
            |m: &mut StatusRequest| { &mut m.status },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<StatusRequest>(
            "StatusRequest",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for StatusRequest {
    const NAME: &'static str = "StatusRequest";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                8 => {
                    self.status = is.read_enum_or_unknown()?;
                },
                tag => {
                    ::protobuf::rt::read_unknown_or_skip_group(tag, is, self.special_fields.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u64 {
        let mut my_size = 0;
        if self.status != ::protobuf::EnumOrUnknown::new(StatusNotification::UNKNOWN) {
            my_size += ::protobuf::rt::int32_size(1, self.status.value());
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.status != ::protobuf::EnumOrUnknown::new(StatusNotification::UNKNOWN) {
            os.write_enum(1, ::protobuf::EnumOrUnknown::value(&self.status))?;
        }
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> StatusRequest {
        StatusRequest::new()
    }

    fn clear(&mut self) {
        self.status = ::protobuf::EnumOrUnknown::new(StatusNotification::UNKNOWN);
        self.special_fields.clear();
    }

    fn default_instance() -> &'static StatusRequest {
        static instance: StatusRequest = StatusRequest {
            status: ::protobuf::EnumOrUnknown::from_i32(0),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for StatusRequest {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("StatusRequest").unwrap()).clone()
    }
}

impl ::std::fmt::Display for StatusRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for StatusRequest {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

#[derive(Clone,Copy,PartialEq,Eq,Debug,Hash)]
// @@protoc_insertion_point(enum:StatusNotification)
pub enum StatusNotification {
    // @@protoc_insertion_point(enum_value:StatusNotification.UNKNOWN)
    UNKNOWN = 0,
    // @@protoc_insertion_point(enum_value:StatusNotification.Running)
    Running = 1,
    // @@protoc_insertion_point(enum_value:StatusNotification.Reload)
    Reload = 2,
    // @@protoc_insertion_point(enum_value:StatusNotification.Exit)
    Exit = 3,
}

impl ::protobuf::Enum for StatusNotification {
    const NAME: &'static str = "StatusNotification";

    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<StatusNotification> {
        match value {
            0 => ::std::option::Option::Some(StatusNotification::UNKNOWN),
            1 => ::std::option::Option::Some(StatusNotification::Running),
            2 => ::std::option::Option::Some(StatusNotification::Reload),
            3 => ::std::option::Option::Some(StatusNotification::Exit),
            _ => ::std::option::Option::None
        }
    }

    const VALUES: &'static [StatusNotification] = &[
        StatusNotification::UNKNOWN,
        StatusNotification::Running,
        StatusNotification::Reload,
        StatusNotification::Exit,
    ];
}

impl ::protobuf::EnumFull for StatusNotification {
    fn enum_descriptor() -> ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().enum_by_package_relative_name("StatusNotification").unwrap()).clone()
    }

    fn descriptor(&self) -> ::protobuf::reflect::EnumValueDescriptor {
        let index = *self as usize;
        Self::enum_descriptor().value_by_index(index)
    }
}

impl ::std::default::Default for StatusNotification {
    fn default() -> Self {
        StatusNotification::UNKNOWN
    }
}

impl StatusNotification {
    fn generated_enum_descriptor_data() -> ::protobuf::reflect::GeneratedEnumDescriptorData {
        ::protobuf::reflect::GeneratedEnumDescriptorData::new::<StatusNotification>("StatusNotification")
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x12notification.proto\"<\n\rStatusRequest\x12+\n\x06status\x18\x01\
    \x20\x01(\x0e2\x13.StatusNotificationR\x06status*D\n\x12StatusNotificati\
    on\x12\x0b\n\x07UNKNOWN\x10\0\x12\x0b\n\x07Running\x10\x01\x12\n\n\x06Re\
    load\x10\x02\x12\x08\n\x04Exit\x10\x03b\x06proto3\
";

/// `FileDescriptorProto` object which was a source for this generated file
pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    static file_descriptor_proto_lazy: ::protobuf::rt::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::rt::Lazy::new();
    file_descriptor_proto_lazy.get(|| {
        ::protobuf::Message::parse_from_bytes(file_descriptor_proto_data).unwrap()
    })
}

/// `FileDescriptor` object which allows dynamic access to files
pub fn file_descriptor() -> ::protobuf::reflect::FileDescriptor {
    static file_descriptor_lazy: ::protobuf::rt::Lazy<::protobuf::reflect::GeneratedFileDescriptor> = ::protobuf::rt::Lazy::new();
    let file_descriptor = file_descriptor_lazy.get(|| {
        let mut deps = ::std::vec::Vec::with_capacity(0);
        let mut messages = ::std::vec::Vec::with_capacity(1);
        messages.push(StatusRequest::generated_message_descriptor_data());
        let mut enums = ::std::vec::Vec::with_capacity(1);
        enums.push(StatusNotification::generated_enum_descriptor_data());
        ::protobuf::reflect::GeneratedFileDescriptor::new_generated(
            file_descriptor_proto(),
            deps,
            messages,
            enums,
        )
    });
    ::protobuf::reflect::FileDescriptor::new_generated_2(file_descriptor)
}
