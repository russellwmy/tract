// This file is generated by rust-protobuf 2.9.0. Do not edit
// @generated

// https://github.com/rust-lang/rust-clippy/issues/702


//! Generated file from `tensorflow/core/framework/graph.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_9_0;

#[derive(PartialEq,Clone,Default)]
pub struct GraphDef {
    // message fields
    pub node: ::protobuf::RepeatedField<super::node_def::NodeDef>,
    pub versions: ::protobuf::SingularPtrField<super::versions::VersionDef>,
    pub version: i32,
    pub library: ::protobuf::SingularPtrField<super::function::FunctionDefLibrary>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a GraphDef {
    fn default() -> &'a GraphDef {
        <GraphDef as ::protobuf::Message>::default_instance()
    }
}

impl GraphDef {
    pub fn new() -> GraphDef {
        ::std::default::Default::default()
    }

    // repeated .tensorflow.NodeDef node = 1;


    pub fn get_node(&self) -> &[super::node_def::NodeDef] {
        &self.node
    }
    pub fn clear_node(&mut self) {
        self.node.clear();
    }

    // Param is passed by value, moved
    pub fn set_node(&mut self, v: ::protobuf::RepeatedField<super::node_def::NodeDef>) {
        self.node = v;
    }

    // Mutable pointer to the field.
    pub fn mut_node(&mut self) -> &mut ::protobuf::RepeatedField<super::node_def::NodeDef> {
        &mut self.node
    }

    // Take field
    pub fn take_node(&mut self) -> ::protobuf::RepeatedField<super::node_def::NodeDef> {
        ::std::mem::replace(&mut self.node, ::protobuf::RepeatedField::new())
    }

    // .tensorflow.VersionDef versions = 4;


    pub fn get_versions(&self) -> &super::versions::VersionDef {
        self.versions.as_ref().unwrap_or_else(|| <super::versions::VersionDef as ::protobuf::Message>::default_instance())
    }
    pub fn clear_versions(&mut self) {
        self.versions.clear();
    }

    pub fn has_versions(&self) -> bool {
        self.versions.is_some()
    }

    // Param is passed by value, moved
    pub fn set_versions(&mut self, v: super::versions::VersionDef) {
        self.versions = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_versions(&mut self) -> &mut super::versions::VersionDef {
        if self.versions.is_none() {
            self.versions.set_default();
        }
        self.versions.as_mut().unwrap()
    }

    // Take field
    pub fn take_versions(&mut self) -> super::versions::VersionDef {
        self.versions.take().unwrap_or_else(|| super::versions::VersionDef::new())
    }

    // int32 version = 3;


    pub fn get_version(&self) -> i32 {
        self.version
    }
    pub fn clear_version(&mut self) {
        self.version = 0;
    }

    // Param is passed by value, moved
    pub fn set_version(&mut self, v: i32) {
        self.version = v;
    }

    // .tensorflow.FunctionDefLibrary library = 2;


    pub fn get_library(&self) -> &super::function::FunctionDefLibrary {
        self.library.as_ref().unwrap_or_else(|| <super::function::FunctionDefLibrary as ::protobuf::Message>::default_instance())
    }
    pub fn clear_library(&mut self) {
        self.library.clear();
    }

    pub fn has_library(&self) -> bool {
        self.library.is_some()
    }

    // Param is passed by value, moved
    pub fn set_library(&mut self, v: super::function::FunctionDefLibrary) {
        self.library = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_library(&mut self) -> &mut super::function::FunctionDefLibrary {
        if self.library.is_none() {
            self.library.set_default();
        }
        self.library.as_mut().unwrap()
    }

    // Take field
    pub fn take_library(&mut self) -> super::function::FunctionDefLibrary {
        self.library.take().unwrap_or_else(|| super::function::FunctionDefLibrary::new())
    }
}

impl ::protobuf::Message for GraphDef {
    fn is_initialized(&self) -> bool {
        for v in &self.node {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.versions {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.library {
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
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.node)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.versions)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.version = tmp;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.library)?;
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
        for value in &self.node {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(ref v) = self.versions.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if self.version != 0 {
            my_size += ::protobuf::rt::value_size(3, self.version, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.library.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        for v in &self.node {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(ref v) = self.versions.as_ref() {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if self.version != 0 {
            os.write_int32(3, self.version)?;
        }
        if let Some(ref v) = self.library.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
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
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> GraphDef {
        GraphDef::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::rt::make_repeated_field_accessor::<_, ::protobuf::reflect::types::ProtobufTypeMessage<super::node_def::NodeDef>>(
                "node",
                |m: &GraphDef| { &m.node },
                |m: &mut GraphDef| { &mut m.node },
            ));
            fields.push(::protobuf::reflect::rt::make_option_accessor::<_, ::protobuf::reflect::types::ProtobufTypeMessage<super::versions::VersionDef>, _>(
                "versions",
                |m: &GraphDef| { &m.versions },
                |m: &mut GraphDef| { &mut m.versions },
            ));
            fields.push(::protobuf::reflect::rt::make_simple_field_accessor::<_, ::protobuf::reflect::types::ProtobufTypeInt32>(
                "version",
                |m: &GraphDef| { &m.version },
                |m: &mut GraphDef| { &mut m.version },
            ));
            fields.push(::protobuf::reflect::rt::make_option_accessor::<_, ::protobuf::reflect::types::ProtobufTypeMessage<super::function::FunctionDefLibrary>, _>(
                "library",
                |m: &GraphDef| { &m.library },
                |m: &mut GraphDef| { &mut m.library },
            ));
            ::protobuf::reflect::MessageDescriptor::new::<GraphDef>(
                "GraphDef",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static GraphDef {
        static instance: ::protobuf::lazy::Lazy<GraphDef> = ::protobuf::lazy::Lazy::INIT;
        instance.get(GraphDef::new)
    }
}

impl ::protobuf::Clear for GraphDef {
    fn clear(&mut self) {
        self.node.clear();
        self.versions.clear();
        self.version = 0;
        self.library.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for GraphDef {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GraphDef {
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n%tensorflow/core/framework/graph.proto\x12\ntensorflow\x1a(tensorflow/\
    core/framework/node_def.proto\x1a(tensorflow/core/framework/function.pro\
    to\x1a(tensorflow/core/framework/versions.proto\"\xbf\x01\n\x08GraphDef\
    \x12'\n\x04node\x18\x01\x20\x03(\x0b2\x13.tensorflow.NodeDefR\x04node\
    \x122\n\x08versions\x18\x04\x20\x01(\x0b2\x16.tensorflow.VersionDefR\x08\
    versions\x12\x1c\n\x07version\x18\x03\x20\x01(\x05R\x07versionB\x02\x18\
    \x01\x128\n\x07library\x18\x02\x20\x01(\x0b2\x1e.tensorflow.FunctionDefL\
    ibraryR\x07libraryBk\n\x18org.tensorflow.frameworkB\x0bGraphProtosP\x01Z\
    =github.com/tensorflow/tensorflow/tensorflow/go/core/framework\xf8\x01\
    \x01b\x06proto3\
";

static file_descriptor_proto_lazy: ::protobuf::lazy::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::lazy::Lazy::INIT;

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    file_descriptor_proto_lazy.get(|| {
        parse_descriptor_proto()
    })
}
