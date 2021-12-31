// This file is generated by rust-protobuf 2.22.1. Do not edit
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
#![allow(unused_imports)]
#![allow(unused_results)]
//! Generated file from `trash_create.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_22_1;

#[derive(PartialEq,Clone,Default)]
pub struct RepeatedTrashId {
    // message fields
    pub items: ::protobuf::RepeatedField<TrashId>,
    pub delete_all: bool,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a RepeatedTrashId {
    fn default() -> &'a RepeatedTrashId {
        <RepeatedTrashId as ::protobuf::Message>::default_instance()
    }
}

impl RepeatedTrashId {
    pub fn new() -> RepeatedTrashId {
        ::std::default::Default::default()
    }

    // repeated .TrashId items = 1;


    pub fn get_items(&self) -> &[TrashId] {
        &self.items
    }
    pub fn clear_items(&mut self) {
        self.items.clear();
    }

    // Param is passed by value, moved
    pub fn set_items(&mut self, v: ::protobuf::RepeatedField<TrashId>) {
        self.items = v;
    }

    // Mutable pointer to the field.
    pub fn mut_items(&mut self) -> &mut ::protobuf::RepeatedField<TrashId> {
        &mut self.items
    }

    // Take field
    pub fn take_items(&mut self) -> ::protobuf::RepeatedField<TrashId> {
        ::std::mem::replace(&mut self.items, ::protobuf::RepeatedField::new())
    }

    // bool delete_all = 2;


    pub fn get_delete_all(&self) -> bool {
        self.delete_all
    }
    pub fn clear_delete_all(&mut self) {
        self.delete_all = false;
    }

    // Param is passed by value, moved
    pub fn set_delete_all(&mut self, v: bool) {
        self.delete_all = v;
    }
}

impl ::protobuf::Message for RepeatedTrashId {
    fn is_initialized(&self) -> bool {
        for v in &self.items {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.items)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.delete_all = tmp;
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
        for value in &self.items {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if self.delete_all != false {
            my_size += 2;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        for v in &self.items {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if self.delete_all != false {
            os.write_bool(2, self.delete_all)?;
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

    fn new() -> RepeatedTrashId {
        RepeatedTrashId::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<TrashId>>(
                "items",
                |m: &RepeatedTrashId| { &m.items },
                |m: &mut RepeatedTrashId| { &mut m.items },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                "delete_all",
                |m: &RepeatedTrashId| { &m.delete_all },
                |m: &mut RepeatedTrashId| { &mut m.delete_all },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<RepeatedTrashId>(
                "RepeatedTrashId",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static RepeatedTrashId {
        static instance: ::protobuf::rt::LazyV2<RepeatedTrashId> = ::protobuf::rt::LazyV2::INIT;
        instance.get(RepeatedTrashId::new)
    }
}

impl ::protobuf::Clear for RepeatedTrashId {
    fn clear(&mut self) {
        self.items.clear();
        self.delete_all = false;
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RepeatedTrashId {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RepeatedTrashId {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct TrashId {
    // message fields
    pub id: ::std::string::String,
    pub ty: TrashType,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a TrashId {
    fn default() -> &'a TrashId {
        <TrashId as ::protobuf::Message>::default_instance()
    }
}

impl TrashId {
    pub fn new() -> TrashId {
        ::std::default::Default::default()
    }

    // string id = 1;


    pub fn get_id(&self) -> &str {
        &self.id
    }
    pub fn clear_id(&mut self) {
        self.id.clear();
    }

    // Param is passed by value, moved
    pub fn set_id(&mut self, v: ::std::string::String) {
        self.id = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_id(&mut self) -> &mut ::std::string::String {
        &mut self.id
    }

    // Take field
    pub fn take_id(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.id, ::std::string::String::new())
    }

    // .TrashType ty = 2;


    pub fn get_ty(&self) -> TrashType {
        self.ty
    }
    pub fn clear_ty(&mut self) {
        self.ty = TrashType::Unknown;
    }

    // Param is passed by value, moved
    pub fn set_ty(&mut self, v: TrashType) {
        self.ty = v;
    }
}

impl ::protobuf::Message for TrashId {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.id)?;
                },
                2 => {
                    ::protobuf::rt::read_proto3_enum_with_unknown_fields_into(wire_type, is, &mut self.ty, 2, &mut self.unknown_fields)?
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
        if !self.id.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.id);
        }
        if self.ty != TrashType::Unknown {
            my_size += ::protobuf::rt::enum_size(2, self.ty);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if !self.id.is_empty() {
            os.write_string(1, &self.id)?;
        }
        if self.ty != TrashType::Unknown {
            os.write_enum(2, ::protobuf::ProtobufEnum::value(&self.ty))?;
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

    fn new() -> TrashId {
        TrashId::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "id",
                |m: &TrashId| { &m.id },
                |m: &mut TrashId| { &mut m.id },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<TrashType>>(
                "ty",
                |m: &TrashId| { &m.ty },
                |m: &mut TrashId| { &mut m.ty },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<TrashId>(
                "TrashId",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static TrashId {
        static instance: ::protobuf::rt::LazyV2<TrashId> = ::protobuf::rt::LazyV2::INIT;
        instance.get(TrashId::new)
    }
}

impl ::protobuf::Clear for TrashId {
    fn clear(&mut self) {
        self.id.clear();
        self.ty = TrashType::Unknown;
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for TrashId {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for TrashId {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Trash {
    // message fields
    pub id: ::std::string::String,
    pub name: ::std::string::String,
    pub modified_time: i64,
    pub create_time: i64,
    pub ty: TrashType,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a Trash {
    fn default() -> &'a Trash {
        <Trash as ::protobuf::Message>::default_instance()
    }
}

impl Trash {
    pub fn new() -> Trash {
        ::std::default::Default::default()
    }

    // string id = 1;


    pub fn get_id(&self) -> &str {
        &self.id
    }
    pub fn clear_id(&mut self) {
        self.id.clear();
    }

    // Param is passed by value, moved
    pub fn set_id(&mut self, v: ::std::string::String) {
        self.id = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_id(&mut self) -> &mut ::std::string::String {
        &mut self.id
    }

    // Take field
    pub fn take_id(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.id, ::std::string::String::new())
    }

    // string name = 2;


    pub fn get_name(&self) -> &str {
        &self.name
    }
    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name(&mut self) -> &mut ::std::string::String {
        &mut self.name
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.name, ::std::string::String::new())
    }

    // int64 modified_time = 3;


    pub fn get_modified_time(&self) -> i64 {
        self.modified_time
    }
    pub fn clear_modified_time(&mut self) {
        self.modified_time = 0;
    }

    // Param is passed by value, moved
    pub fn set_modified_time(&mut self, v: i64) {
        self.modified_time = v;
    }

    // int64 create_time = 4;


    pub fn get_create_time(&self) -> i64 {
        self.create_time
    }
    pub fn clear_create_time(&mut self) {
        self.create_time = 0;
    }

    // Param is passed by value, moved
    pub fn set_create_time(&mut self, v: i64) {
        self.create_time = v;
    }

    // .TrashType ty = 5;


    pub fn get_ty(&self) -> TrashType {
        self.ty
    }
    pub fn clear_ty(&mut self) {
        self.ty = TrashType::Unknown;
    }

    // Param is passed by value, moved
    pub fn set_ty(&mut self, v: TrashType) {
        self.ty = v;
    }
}

impl ::protobuf::Message for Trash {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.id)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.name)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.modified_time = tmp;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.create_time = tmp;
                },
                5 => {
                    ::protobuf::rt::read_proto3_enum_with_unknown_fields_into(wire_type, is, &mut self.ty, 5, &mut self.unknown_fields)?
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
        if !self.id.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.id);
        }
        if !self.name.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.name);
        }
        if self.modified_time != 0 {
            my_size += ::protobuf::rt::value_size(3, self.modified_time, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.create_time != 0 {
            my_size += ::protobuf::rt::value_size(4, self.create_time, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.ty != TrashType::Unknown {
            my_size += ::protobuf::rt::enum_size(5, self.ty);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if !self.id.is_empty() {
            os.write_string(1, &self.id)?;
        }
        if !self.name.is_empty() {
            os.write_string(2, &self.name)?;
        }
        if self.modified_time != 0 {
            os.write_int64(3, self.modified_time)?;
        }
        if self.create_time != 0 {
            os.write_int64(4, self.create_time)?;
        }
        if self.ty != TrashType::Unknown {
            os.write_enum(5, ::protobuf::ProtobufEnum::value(&self.ty))?;
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

    fn new() -> Trash {
        Trash::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "id",
                |m: &Trash| { &m.id },
                |m: &mut Trash| { &mut m.id },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "name",
                |m: &Trash| { &m.name },
                |m: &mut Trash| { &mut m.name },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                "modified_time",
                |m: &Trash| { &m.modified_time },
                |m: &mut Trash| { &mut m.modified_time },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                "create_time",
                |m: &Trash| { &m.create_time },
                |m: &mut Trash| { &mut m.create_time },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<TrashType>>(
                "ty",
                |m: &Trash| { &m.ty },
                |m: &mut Trash| { &mut m.ty },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<Trash>(
                "Trash",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static Trash {
        static instance: ::protobuf::rt::LazyV2<Trash> = ::protobuf::rt::LazyV2::INIT;
        instance.get(Trash::new)
    }
}

impl ::protobuf::Clear for Trash {
    fn clear(&mut self) {
        self.id.clear();
        self.name.clear();
        self.modified_time = 0;
        self.create_time = 0;
        self.ty = TrashType::Unknown;
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Trash {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Trash {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct RepeatedTrash {
    // message fields
    pub items: ::protobuf::RepeatedField<Trash>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a RepeatedTrash {
    fn default() -> &'a RepeatedTrash {
        <RepeatedTrash as ::protobuf::Message>::default_instance()
    }
}

impl RepeatedTrash {
    pub fn new() -> RepeatedTrash {
        ::std::default::Default::default()
    }

    // repeated .Trash items = 1;


    pub fn get_items(&self) -> &[Trash] {
        &self.items
    }
    pub fn clear_items(&mut self) {
        self.items.clear();
    }

    // Param is passed by value, moved
    pub fn set_items(&mut self, v: ::protobuf::RepeatedField<Trash>) {
        self.items = v;
    }

    // Mutable pointer to the field.
    pub fn mut_items(&mut self) -> &mut ::protobuf::RepeatedField<Trash> {
        &mut self.items
    }

    // Take field
    pub fn take_items(&mut self) -> ::protobuf::RepeatedField<Trash> {
        ::std::mem::replace(&mut self.items, ::protobuf::RepeatedField::new())
    }
}

impl ::protobuf::Message for RepeatedTrash {
    fn is_initialized(&self) -> bool {
        for v in &self.items {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.items)?;
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
        for value in &self.items {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        for v in &self.items {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
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

    fn new() -> RepeatedTrash {
        RepeatedTrash::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Trash>>(
                "items",
                |m: &RepeatedTrash| { &m.items },
                |m: &mut RepeatedTrash| { &mut m.items },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<RepeatedTrash>(
                "RepeatedTrash",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static RepeatedTrash {
        static instance: ::protobuf::rt::LazyV2<RepeatedTrash> = ::protobuf::rt::LazyV2::INIT;
        instance.get(RepeatedTrash::new)
    }
}

impl ::protobuf::Clear for RepeatedTrash {
    fn clear(&mut self) {
        self.items.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RepeatedTrash {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RepeatedTrash {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum TrashType {
    Unknown = 0,
    View = 1,
    App = 2,
}

impl ::protobuf::ProtobufEnum for TrashType {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<TrashType> {
        match value {
            0 => ::std::option::Option::Some(TrashType::Unknown),
            1 => ::std::option::Option::Some(TrashType::View),
            2 => ::std::option::Option::Some(TrashType::App),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [TrashType] = &[
            TrashType::Unknown,
            TrashType::View,
            TrashType::App,
        ];
        values
    }

    fn enum_descriptor_static() -> &'static ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            ::protobuf::reflect::EnumDescriptor::new_pb_name::<TrashType>("TrashType", file_descriptor_proto())
        })
    }
}

impl ::std::marker::Copy for TrashType {
}

impl ::std::default::Default for TrashType {
    fn default() -> Self {
        TrashType::Unknown
    }
}

impl ::protobuf::reflect::ProtobufValue for TrashType {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Enum(::protobuf::ProtobufEnum::descriptor(self))
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x12trash_create.proto\"P\n\x0fRepeatedTrashId\x12\x1e\n\x05items\x18\
    \x01\x20\x03(\x0b2\x08.TrashIdR\x05items\x12\x1d\n\ndelete_all\x18\x02\
    \x20\x01(\x08R\tdeleteAll\"5\n\x07TrashId\x12\x0e\n\x02id\x18\x01\x20\
    \x01(\tR\x02id\x12\x1a\n\x02ty\x18\x02\x20\x01(\x0e2\n.TrashTypeR\x02ty\
    \"\x8d\x01\n\x05Trash\x12\x0e\n\x02id\x18\x01\x20\x01(\tR\x02id\x12\x12\
    \n\x04name\x18\x02\x20\x01(\tR\x04name\x12#\n\rmodified_time\x18\x03\x20\
    \x01(\x03R\x0cmodifiedTime\x12\x1f\n\x0bcreate_time\x18\x04\x20\x01(\x03\
    R\ncreateTime\x12\x1a\n\x02ty\x18\x05\x20\x01(\x0e2\n.TrashTypeR\x02ty\"\
    -\n\rRepeatedTrash\x12\x1c\n\x05items\x18\x01\x20\x03(\x0b2\x06.TrashR\
    \x05items*+\n\tTrashType\x12\x0b\n\x07Unknown\x10\0\x12\x08\n\x04View\
    \x10\x01\x12\x07\n\x03App\x10\x02J\xc7\x06\n\x06\x12\x04\0\0\x18\x01\n\
    \x08\n\x01\x0c\x12\x03\0\0\x12\n\n\n\x02\x04\0\x12\x04\x02\0\x05\x01\n\n\
    \n\x03\x04\0\x01\x12\x03\x02\x08\x17\n\x0b\n\x04\x04\0\x02\0\x12\x03\x03\
    \x04\x1f\n\x0c\n\x05\x04\0\x02\0\x04\x12\x03\x03\x04\x0c\n\x0c\n\x05\x04\
    \0\x02\0\x06\x12\x03\x03\r\x14\n\x0c\n\x05\x04\0\x02\0\x01\x12\x03\x03\
    \x15\x1a\n\x0c\n\x05\x04\0\x02\0\x03\x12\x03\x03\x1d\x1e\n\x0b\n\x04\x04\
    \0\x02\x01\x12\x03\x04\x04\x18\n\x0c\n\x05\x04\0\x02\x01\x05\x12\x03\x04\
    \x04\x08\n\x0c\n\x05\x04\0\x02\x01\x01\x12\x03\x04\t\x13\n\x0c\n\x05\x04\
    \0\x02\x01\x03\x12\x03\x04\x16\x17\n\n\n\x02\x04\x01\x12\x04\x06\0\t\x01\
    \n\n\n\x03\x04\x01\x01\x12\x03\x06\x08\x0f\n\x0b\n\x04\x04\x01\x02\0\x12\
    \x03\x07\x04\x12\n\x0c\n\x05\x04\x01\x02\0\x05\x12\x03\x07\x04\n\n\x0c\n\
    \x05\x04\x01\x02\0\x01\x12\x03\x07\x0b\r\n\x0c\n\x05\x04\x01\x02\0\x03\
    \x12\x03\x07\x10\x11\n\x0b\n\x04\x04\x01\x02\x01\x12\x03\x08\x04\x15\n\
    \x0c\n\x05\x04\x01\x02\x01\x06\x12\x03\x08\x04\r\n\x0c\n\x05\x04\x01\x02\
    \x01\x01\x12\x03\x08\x0e\x10\n\x0c\n\x05\x04\x01\x02\x01\x03\x12\x03\x08\
    \x13\x14\n\n\n\x02\x04\x02\x12\x04\n\0\x10\x01\n\n\n\x03\x04\x02\x01\x12\
    \x03\n\x08\r\n\x0b\n\x04\x04\x02\x02\0\x12\x03\x0b\x04\x12\n\x0c\n\x05\
    \x04\x02\x02\0\x05\x12\x03\x0b\x04\n\n\x0c\n\x05\x04\x02\x02\0\x01\x12\
    \x03\x0b\x0b\r\n\x0c\n\x05\x04\x02\x02\0\x03\x12\x03\x0b\x10\x11\n\x0b\n\
    \x04\x04\x02\x02\x01\x12\x03\x0c\x04\x14\n\x0c\n\x05\x04\x02\x02\x01\x05\
    \x12\x03\x0c\x04\n\n\x0c\n\x05\x04\x02\x02\x01\x01\x12\x03\x0c\x0b\x0f\n\
    \x0c\n\x05\x04\x02\x02\x01\x03\x12\x03\x0c\x12\x13\n\x0b\n\x04\x04\x02\
    \x02\x02\x12\x03\r\x04\x1c\n\x0c\n\x05\x04\x02\x02\x02\x05\x12\x03\r\x04\
    \t\n\x0c\n\x05\x04\x02\x02\x02\x01\x12\x03\r\n\x17\n\x0c\n\x05\x04\x02\
    \x02\x02\x03\x12\x03\r\x1a\x1b\n\x0b\n\x04\x04\x02\x02\x03\x12\x03\x0e\
    \x04\x1a\n\x0c\n\x05\x04\x02\x02\x03\x05\x12\x03\x0e\x04\t\n\x0c\n\x05\
    \x04\x02\x02\x03\x01\x12\x03\x0e\n\x15\n\x0c\n\x05\x04\x02\x02\x03\x03\
    \x12\x03\x0e\x18\x19\n\x0b\n\x04\x04\x02\x02\x04\x12\x03\x0f\x04\x15\n\
    \x0c\n\x05\x04\x02\x02\x04\x06\x12\x03\x0f\x04\r\n\x0c\n\x05\x04\x02\x02\
    \x04\x01\x12\x03\x0f\x0e\x10\n\x0c\n\x05\x04\x02\x02\x04\x03\x12\x03\x0f\
    \x13\x14\n\n\n\x02\x04\x03\x12\x04\x11\0\x13\x01\n\n\n\x03\x04\x03\x01\
    \x12\x03\x11\x08\x15\n\x0b\n\x04\x04\x03\x02\0\x12\x03\x12\x04\x1d\n\x0c\
    \n\x05\x04\x03\x02\0\x04\x12\x03\x12\x04\x0c\n\x0c\n\x05\x04\x03\x02\0\
    \x06\x12\x03\x12\r\x12\n\x0c\n\x05\x04\x03\x02\0\x01\x12\x03\x12\x13\x18\
    \n\x0c\n\x05\x04\x03\x02\0\x03\x12\x03\x12\x1b\x1c\n\n\n\x02\x05\0\x12\
    \x04\x14\0\x18\x01\n\n\n\x03\x05\0\x01\x12\x03\x14\x05\x0e\n\x0b\n\x04\
    \x05\0\x02\0\x12\x03\x15\x04\x10\n\x0c\n\x05\x05\0\x02\0\x01\x12\x03\x15\
    \x04\x0b\n\x0c\n\x05\x05\0\x02\0\x02\x12\x03\x15\x0e\x0f\n\x0b\n\x04\x05\
    \0\x02\x01\x12\x03\x16\x04\r\n\x0c\n\x05\x05\0\x02\x01\x01\x12\x03\x16\
    \x04\x08\n\x0c\n\x05\x05\0\x02\x01\x02\x12\x03\x16\x0b\x0c\n\x0b\n\x04\
    \x05\0\x02\x02\x12\x03\x17\x04\x0c\n\x0c\n\x05\x05\0\x02\x02\x01\x12\x03\
    \x17\x04\x07\n\x0c\n\x05\x05\0\x02\x02\x02\x12\x03\x17\n\x0bb\x06proto3\
";

static file_descriptor_proto_lazy: ::protobuf::rt::LazyV2<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::rt::LazyV2::INIT;

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::Message::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    file_descriptor_proto_lazy.get(|| {
        parse_descriptor_proto()
    })
}
