// SPDX-License-Identifier: MPL-2.0
//! Code generation logic for ReScript from protobuf descriptors

use anyhow::Result;
use prost_types::compiler::{code_generator_response, CodeGeneratorRequest, CodeGeneratorResponse};
use prost_types::{DescriptorProto, EnumDescriptorProto, FieldDescriptorProto, FileDescriptorProto};

use crate::templates::{EnumTemplate, FieldInfo, MessageTemplate, ModuleTemplate};
use crate::Options;

pub struct Generator {
    options: Options,
}

impl Generator {
    pub fn new(options: Options) -> Self {
        Self { options }
    }

    pub fn generate(&self, request: &CodeGeneratorRequest) -> Result<CodeGeneratorResponse> {
        let mut response = CodeGeneratorResponse::default();

        // Set supported features
        response.supported_features = Some(
            code_generator_response::Feature::Proto3Optional as u64,
        );

        // Process each file to generate
        for file_name in &request.file_to_generate {
            // Find the file descriptor
            let file_desc = request
                .proto_file
                .iter()
                .find(|f| f.name.as_deref() == Some(file_name.as_str()));

            if let Some(desc) = file_desc {
                let generated = self.generate_file(desc)?;
                response.file.push(generated);
            }
        }

        Ok(response)
    }

    fn generate_file(&self, file: &FileDescriptorProto) -> Result<code_generator_response::File> {
        let file_name = file.name.as_deref().unwrap_or("unknown");
        let package = file.package.as_deref().unwrap_or("");

        // Convert file name to ReScript module name
        // e.g., "user.proto" -> "UserProto.res"
        let module_name = self.proto_to_module_name(file_name);

        let mut modules = Vec::new();

        // Generate enums
        for enum_desc in &file.enum_type {
            modules.push(self.generate_enum(enum_desc)?);
        }

        // Generate messages
        for msg_desc in &file.message_type {
            modules.push(self.generate_message(msg_desc, &self.options)?);
        }

        // TODO: Generate services if grpc option is enabled

        let template = ModuleTemplate {
            package: package.to_string(),
            source_file: file_name.to_string(),
            modules,
            use_wasm: self.options.wasm,
        };

        let content = template.render();

        Ok(code_generator_response::File {
            name: Some(format!("{}.res", module_name)),
            content: Some(content),
            ..Default::default()
        })
    }

    fn generate_enum(&self, desc: &EnumDescriptorProto) -> Result<String> {
        let name = desc.name.as_deref().unwrap_or("UnknownEnum");

        let variants: Vec<(String, i32)> = desc
            .value
            .iter()
            .map(|v| {
                let variant_name = v.name.as_deref().unwrap_or("UNKNOWN");
                let number = v.number.unwrap_or(0);
                (self.to_rescript_variant(variant_name), number)
            })
            .collect();

        let template = EnumTemplate {
            name: self.to_rescript_type_name(name),
            variants,
        };

        Ok(template.render())
    }

    fn generate_message(&self, desc: &DescriptorProto, options: &Options) -> Result<String> {
        let name = desc.name.as_deref().unwrap_or("UnknownMessage");

        let fields: Vec<FieldInfo> = desc
            .field
            .iter()
            .map(|f| self.field_to_info(f))
            .collect();

        // Handle nested types
        let mut nested = Vec::new();
        for nested_enum in &desc.enum_type {
            nested.push(self.generate_enum(nested_enum)?);
        }
        for nested_msg in &desc.nested_type {
            // Skip map entry types (auto-generated)
            if nested_msg.options.as_ref().map(|o| o.map_entry()).unwrap_or(false) {
                continue;
            }
            nested.push(self.generate_message(nested_msg, options)?);
        }

        let template = MessageTemplate {
            name: self.to_rescript_type_name(name),
            fields,
            nested,
            use_wasm: options.wasm,
        };

        Ok(template.render())
    }

    fn field_to_info(&self, field: &FieldDescriptorProto) -> FieldInfo {
        let name = field.name.as_deref().unwrap_or("unknown");
        let number = field.number.unwrap_or(0);
        let is_repeated = field.label() == prost_types::field_descriptor_proto::Label::Repeated;
        let is_optional = field.proto3_optional.unwrap_or(false)
            || field.label() == prost_types::field_descriptor_proto::Label::Optional;

        let rescript_type = self.proto_type_to_rescript(field);

        FieldInfo {
            name: self.to_rescript_field_name(name),
            proto_name: name.to_string(),
            number,
            rescript_type,
            is_optional,
            is_repeated,
        }
    }

    fn proto_type_to_rescript(&self, field: &FieldDescriptorProto) -> String {
        use prost_types::field_descriptor_proto::Type;

        match field.r#type() {
            Type::Double | Type::Float => "float".to_string(),
            Type::Int64 | Type::Uint64 | Type::Sint64 | Type::Fixed64 | Type::Sfixed64 => {
                "Int64.t".to_string()
            }
            Type::Int32 | Type::Uint32 | Type::Sint32 | Type::Fixed32 | Type::Sfixed32 => {
                "int".to_string()
            }
            Type::Bool => "bool".to_string(),
            Type::String => "string".to_string(),
            Type::Bytes => "Js.Typed_array.Uint8Array.t".to_string(),
            Type::Message | Type::Enum => {
                // Extract type name from fully qualified name
                let type_name = field.type_name.as_deref().unwrap_or("unknown");
                let simple_name = type_name.rsplit('.').next().unwrap_or(type_name);
                format!("{}.t", self.to_rescript_type_name(simple_name))
            }
            Type::Group => "unit".to_string(), // Deprecated, treat as unit
        }
    }

    fn proto_to_module_name(&self, file_name: &str) -> String {
        // "path/to/user.proto" -> "UserProto"
        let base = file_name
            .rsplit('/')
            .next()
            .unwrap_or(file_name)
            .trim_end_matches(".proto");

        let mut result = String::new();
        let mut capitalize_next = true;

        for c in base.chars() {
            if c == '_' || c == '-' {
                capitalize_next = true;
            } else if capitalize_next {
                result.push(c.to_ascii_uppercase());
                capitalize_next = false;
            } else {
                result.push(c);
            }
        }

        format!("{}Proto", result)
    }

    fn to_rescript_type_name(&self, name: &str) -> String {
        // PascalCase for type/module names
        let mut result = String::new();
        let mut capitalize_next = true;

        for c in name.chars() {
            if c == '_' {
                capitalize_next = true;
            } else if capitalize_next {
                result.push(c.to_ascii_uppercase());
                capitalize_next = false;
            } else {
                result.push(c);
            }
        }

        result
    }

    fn to_rescript_field_name(&self, name: &str) -> String {
        // camelCase for field names
        let mut result = String::new();
        let mut capitalize_next = false;

        for c in name.chars() {
            if c == '_' {
                capitalize_next = true;
            } else if capitalize_next {
                result.push(c.to_ascii_uppercase());
                capitalize_next = false;
            } else {
                result.push(c);
            }
        }

        // Escape ReScript keywords
        match result.as_str() {
            "type" | "open" | "let" | "module" | "switch" | "if" | "else" | "while" | "for"
            | "try" | "catch" | "as" | "and" | "or" | "true" | "false" | "rec" | "external"
            | "mutable" | "include" | "private" | "constraint" | "lazy" | "assert"
            | "exception" => {
                format!("{}_", result)
            }
            _ => result,
        }
    }

    fn to_rescript_variant(&self, name: &str) -> String {
        // Variant names should be PascalCase and start with uppercase
        let pascal = self.to_rescript_type_name(name);

        // Handle SCREAMING_SNAKE_CASE enum values
        if name.chars().all(|c| c.is_uppercase() || c == '_' || c.is_numeric()) {
            // SOME_VALUE -> SomeValue
            let mut result = String::new();
            let mut capitalize_next = true;

            for c in name.chars() {
                if c == '_' {
                    capitalize_next = true;
                } else if capitalize_next {
                    result.push(c.to_ascii_uppercase());
                    capitalize_next = false;
                } else {
                    result.push(c.to_ascii_lowercase());
                }
            }

            result
        } else {
            pascal
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_proto_to_module_name() {
        let gen = Generator::new(Options::default());
        assert_eq!(gen.proto_to_module_name("user.proto"), "UserProto");
        assert_eq!(
            gen.proto_to_module_name("path/to/user_service.proto"),
            "UserServiceProto"
        );
    }

    #[test]
    fn test_to_rescript_field_name() {
        let gen = Generator::new(Options::default());
        assert_eq!(gen.to_rescript_field_name("user_name"), "userName");
        assert_eq!(gen.to_rescript_field_name("type"), "type_");
    }

    #[test]
    fn test_to_rescript_variant() {
        let gen = Generator::new(Options::default());
        assert_eq!(gen.to_rescript_variant("SOME_VALUE"), "SomeValue");
        assert_eq!(gen.to_rescript_variant("Active"), "Active");
    }
}
