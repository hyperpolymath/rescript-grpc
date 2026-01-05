// SPDX-License-Identifier: MPL-2.0
//! ReScript code templates for generated output

/// Information about a message field
#[derive(Debug, Clone)]
pub struct FieldInfo {
    pub name: String,
    pub proto_name: String,
    pub number: i32,
    pub rescript_type: String,
    pub is_optional: bool,
    pub is_repeated: bool,
    pub is_message: bool,
    pub is_enum: bool,
}

impl FieldInfo {
    /// Get the full ReScript type including option/array wrappers
    pub fn full_type(&self) -> String {
        let base = &self.rescript_type;
        if self.is_repeated {
            format!("array<{}>", base)
        } else if self.is_optional {
            format!("option<{}>", base)
        } else {
            base.clone()
        }
    }

    /// Get the JSON encoder for this field's base type
    pub fn json_encoder(&self) -> String {
        if self.is_message {
            format!("{}.toJson", self.rescript_type.trim_end_matches(".t"))
        } else if self.is_enum {
            // For enums, we need a lambda that converts then encodes
            let enum_name = self.rescript_type.trim_end_matches(".t");
            format!("v => Json.Encode.int({}.toInt(v))", enum_name)
        } else {
            match self.rescript_type.as_str() {
                "string" => "Json.Encode.string".to_string(),
                "int" => "Json.Encode.int".to_string(),
                "float" => "Json.Encode.float".to_string(),
                "bool" => "Json.Encode.bool".to_string(),
                "bigint" => "Json.Encode.int64".to_string(),
                "Js.Typed_array.Uint8Array.t" => "Json.Encode.bytes".to_string(),
                _ => "Json.Encode.string".to_string(),
            }
        }
    }

    /// Get the JSON decoder for this field's base type
    pub fn json_decoder(&self) -> String {
        if self.is_message {
            format!("{}.fromJson", self.rescript_type.trim_end_matches(".t"))
        } else if self.is_enum {
            let enum_name = self.rescript_type.trim_end_matches(".t");
            format!("json => Json.Decode.int(json)->Option.flatMap({}.fromInt)", enum_name)
        } else {
            match self.rescript_type.as_str() {
                "string" => "Json.Decode.string".to_string(),
                "int" => "Json.Decode.int".to_string(),
                "float" => "Json.Decode.float".to_string(),
                "bool" => "Json.Decode.bool".to_string(),
                "bigint" => "Json.Decode.int64".to_string(),
                "Js.Typed_array.Uint8Array.t" => "Json.Decode.bytes".to_string(),
                _ => "Json.Decode.string".to_string(),
            }
        }
    }
}

/// Template for generating a ReScript module from a proto file
pub struct ModuleTemplate {
    pub package: String,
    pub source_file: String,
    pub modules: Vec<String>,
    pub use_wasm: bool,
}

impl ModuleTemplate {
    pub fn render(&self) -> String {
        let mut out = String::new();

        // Header comment
        out.push_str(&format!(
            "// Generated from {} by protoc-gen-rescript\n",
            self.source_file
        ));
        out.push_str("// SPDX-License-Identifier: MPL-2.0\n");
        out.push_str("// DO NOT EDIT - regenerate from .proto source\n\n");

        if !self.package.is_empty() {
            out.push_str(&format!("// Package: {}\n\n", self.package));
        }

        // WASM codec import if enabled
        if self.use_wasm {
            out.push_str("// WASM codec for encode/decode\n");
            out.push_str("@module(\"./proto_codec.wasm\")\n");
            out.push_str("external wasmCodec: Wasm.Instance.t = \"default\"\n\n");
        }

        // Render all modules (enums + messages)
        for module in &self.modules {
            out.push_str(module);
            out.push_str("\n\n");
        }

        out
    }
}

/// Template for generating a ReScript enum from proto enum
pub struct EnumTemplate {
    pub name: String,
    pub variants: Vec<(String, i32)>,
}

impl EnumTemplate {
    pub fn render(&self) -> String {
        let mut out = String::new();

        out.push_str(&format!("module {} = {{\n", self.name));

        // Polymorphic variant type
        out.push_str("  type t = [\n");
        for (variant, _) in &self.variants {
            out.push_str(&format!("    | #{}\n", variant));
        }
        out.push_str("  ]\n\n");

        // To int conversion
        out.push_str("  let toInt = (v: t): int => {\n");
        out.push_str("    switch v {\n");
        for (variant, number) in &self.variants {
            out.push_str(&format!("    | #{} => {}\n", variant, number));
        }
        out.push_str("    }\n");
        out.push_str("  }\n\n");

        // From int conversion
        out.push_str("  let fromInt = (i: int): option<t> => {\n");
        out.push_str("    switch i {\n");
        for (variant, number) in &self.variants {
            out.push_str(&format!("    | {} => Some(#{})\n", number, variant));
        }
        out.push_str("    | _ => None\n");
        out.push_str("    }\n");
        out.push_str("  }\n");

        out.push_str("}\n");

        out
    }
}

/// Template for generating a ReScript message module
pub struct MessageTemplate {
    pub name: String,
    pub fields: Vec<FieldInfo>,
    pub nested: Vec<String>,
    pub use_wasm: bool,
}

impl MessageTemplate {
    pub fn render(&self) -> String {
        let mut out = String::new();

        out.push_str(&format!("module {} = {{\n", self.name));

        // Nested types first
        for nested in &self.nested {
            // Indent nested content
            for line in nested.lines() {
                out.push_str(&format!("  {}\n", line));
            }
            out.push('\n');
        }

        // Record type
        out.push_str("  type t = {\n");
        for field in &self.fields {
            out.push_str(&format!(
                "    {}: {},\n",
                field.name,
                field.full_type()
            ));
        }
        out.push_str("  }\n\n");

        // Default value constructor
        out.push_str("  let make = (\n");
        for (i, field) in self.fields.iter().enumerate() {
            let prefix = if i == 0 { "    " } else { "    " };
            let suffix = if i == self.fields.len() - 1 { "" } else { "," };

            if field.is_optional {
                out.push_str(&format!("{}~{}=?{}\n", prefix, field.name, suffix));
            } else if field.is_repeated {
                out.push_str(&format!("{}~{}=[]{}\n", prefix, field.name, suffix));
            } else {
                out.push_str(&format!("{}~{}{}\n", prefix, field.name, suffix));
            }
        }
        out.push_str("  ): t => {\n");
        for field in &self.fields {
            out.push_str(&format!("    {},\n", field.name));
        }
        out.push_str("  }\n");

        // JSON codec functions
        out.push_str(&self.render_json_codec());

        // WASM encode/decode stubs if enabled
        if self.use_wasm {
            out.push_str(&self.render_wasm_codec());
        }

        out.push_str("}\n");

        out
    }

    fn render_json_codec(&self) -> String {
        let mut out = String::new();

        // toJson function
        out.push_str("\n  // JSON serialization\n");
        out.push_str("  let toJson = (msg: t): Js.Json.t => {\n");
        out.push_str("    Json.Encode.object(Json.Encode.fields(\n");
        out.push_str("      [\n");

        // Required fields
        for field in &self.fields {
            if !field.is_optional && !field.is_repeated {
                out.push_str(&format!(
                    "        Json.Encode.required(\"{}\", msg.{}, {}),\n",
                    field.proto_name,
                    field.name,
                    field.json_encoder()
                ));
            }
        }

        out.push_str("      ],\n");
        out.push_str("      [\n");

        // Optional and repeated fields
        for field in &self.fields {
            if field.is_optional {
                out.push_str(&format!(
                    "        Json.Encode.optional(\"{}\", msg.{}, {}),\n",
                    field.proto_name,
                    field.name,
                    field.json_encoder()
                ));
            } else if field.is_repeated {
                out.push_str(&format!(
                    "        Json.Encode.repeated(\"{}\", msg.{}, {}),\n",
                    field.proto_name,
                    field.name,
                    field.json_encoder()
                ));
            }
        }

        out.push_str("      ],\n");
        out.push_str("    ))\n");
        out.push_str("  }\n\n");

        // fromJson function
        out.push_str("  // JSON deserialization\n");
        out.push_str("  let fromJson = (json: Js.Json.t): option<t> => {\n");
        out.push_str("    switch Json.Decode.object(json) {\n");
        out.push_str("    | Some(obj) =>\n");

        // Build the result
        let mut decode_lines = Vec::new();
        for field in &self.fields {
            if field.is_repeated {
                decode_lines.push(format!(
                    "        let {} = Json.Decode.repeated(obj, \"{}\", {})->Result.getOr([])",
                    field.name, field.proto_name, field.json_decoder()
                ));
            } else if field.is_optional {
                decode_lines.push(format!(
                    "        let {} = Json.Decode.optional(obj, \"{}\", {})->Result.getOr(None)",
                    field.name, field.proto_name, field.json_decoder()
                ));
            } else {
                decode_lines.push(format!(
                    "        let {} = Json.Decode.required(obj, \"{}\", {})",
                    field.name, field.proto_name, field.json_decoder()
                ));
            }
        }

        for line in &decode_lines {
            out.push_str(line);
            out.push('\n');
        }

        // Check required fields and build result
        let required_fields: Vec<_> = self.fields.iter()
            .filter(|f| !f.is_optional && !f.is_repeated)
            .collect();

        if required_fields.is_empty() {
            out.push_str("        Some({\n");
            for field in &self.fields {
                out.push_str(&format!("          {},\n", field.name));
            }
            out.push_str("        })\n");
        } else {
            // Check all required fields are Ok
            out.push_str("        switch (");
            for (i, field) in required_fields.iter().enumerate() {
                if i > 0 {
                    out.push_str(", ");
                }
                out.push_str(&field.name);
            }
            out.push_str(") {\n");
            out.push_str("        | (");
            for (i, field) in required_fields.iter().enumerate() {
                if i > 0 {
                    out.push_str(", ");
                }
                out.push_str(&format!("Ok({})", field.name));
            }
            out.push_str(") =>\n");
            out.push_str("          Some({\n");
            for field in &self.fields {
                out.push_str(&format!("            {},\n", field.name));
            }
            out.push_str("          })\n");
            out.push_str("        | _ => None\n");
            out.push_str("        }\n");
        }

        out.push_str("    | None => None\n");
        out.push_str("    }\n");
        out.push_str("  }\n");

        out
    }

    fn render_wasm_codec(&self) -> String {
        let mut out = String::new();
        let lower_name = self.name.to_lowercase();

        out.push_str("\n  // WASM codec functions\n");

        // Encode function
        out.push_str(&format!(
            "  let encode = async (msg: t): Js.Typed_array.Uint8Array.t => {{\n"
        ));
        out.push_str("    let exports = Wasm.Instance.exports(wasmCodec)\n");
        out.push_str("    let memory = %raw(`exports.memory`)\n");
        out.push_str("    let allocator = Wasm.Allocator.fromExports(exports)\n");
        out.push_str(&format!(
            "    let encodeFn: Wasm.Allocator.ptr => (Wasm.Allocator.ptr, int) = %raw(`exports.encode_{}`)\n",
            lower_name
        ));
        out.push_str("    // TODO: Serialize msg to memory, call encodeFn\n");
        out.push_str("    Js.Typed_array.Uint8Array.make([])\n");
        out.push_str("  }\n\n");

        // Decode function
        out.push_str(&format!(
            "  let decode = async (bytes: Js.Typed_array.Uint8Array.t): t => {{\n"
        ));
        out.push_str("    let exports = Wasm.Instance.exports(wasmCodec)\n");
        out.push_str("    let memory = %raw(`exports.memory`)\n");
        out.push_str("    let allocator = Wasm.Allocator.fromExports(exports)\n");
        out.push_str(&format!(
            "    let decodeFn: (Wasm.Allocator.ptr, int) => Wasm.Allocator.ptr = %raw(`exports.decode_{}`)\n",
            lower_name
        ));
        out.push_str("    // TODO: Copy bytes to memory, call decodeFn, deserialize result\n");
        out.push_str(&format!("    make(\n"));
        for field in &self.fields {
            if field.is_optional {
                out.push_str(&format!("      ~{}=None,\n", field.name));
            } else if field.is_repeated {
                out.push_str(&format!("      ~{}=[],\n", field.name));
            } else {
                // Default values for required fields
                let default = match field.rescript_type.as_str() {
                    "int" => "0",
                    "float" => "0.0",
                    "bool" => "false",
                    "string" => "\"\"",
                    _ => "Obj.magic(0)", // Placeholder for complex types
                };
                out.push_str(&format!("      ~{}={},\n", field.name, default));
            }
        }
        out.push_str("    )\n");
        out.push_str("  }\n");

        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_field_full_type() {
        let field = FieldInfo {
            name: "tags".to_string(),
            proto_name: "tags".to_string(),
            number: 1,
            rescript_type: "string".to_string(),
            is_optional: false,
            is_repeated: true,
            is_message: false,
            is_enum: false,
        };
        assert_eq!(field.full_type(), "array<string>");

        let optional = FieldInfo {
            is_optional: true,
            is_repeated: false,
            ..field.clone()
        };
        assert_eq!(optional.full_type(), "option<string>");
    }

    #[test]
    fn test_json_encoders() {
        let string_field = FieldInfo {
            name: "name".to_string(),
            proto_name: "name".to_string(),
            number: 1,
            rescript_type: "string".to_string(),
            is_optional: false,
            is_repeated: false,
            is_message: false,
            is_enum: false,
        };
        assert_eq!(string_field.json_encoder(), "Json.Encode.string");

        let enum_field = FieldInfo {
            name: "status".to_string(),
            proto_name: "status".to_string(),
            number: 2,
            rescript_type: "Status.t".to_string(),
            is_optional: false,
            is_repeated: false,
            is_message: false,
            is_enum: true,
        };
        assert_eq!(enum_field.json_encoder(), "Status.toInt->Json.Encode.int");

        let msg_field = FieldInfo {
            name: "address".to_string(),
            proto_name: "address".to_string(),
            number: 3,
            rescript_type: "Address.t".to_string(),
            is_optional: true,
            is_repeated: false,
            is_message: true,
            is_enum: false,
        };
        assert_eq!(msg_field.json_encoder(), "Address.toJson");
    }

    #[test]
    fn test_enum_template() {
        let template = EnumTemplate {
            name: "Status".to_string(),
            variants: vec![
                ("Unknown".to_string(), 0),
                ("Active".to_string(), 1),
                ("Inactive".to_string(), 2),
            ],
        };
        let output = template.render();
        assert!(output.contains("module Status"));
        assert!(output.contains("#Active"));
        assert!(output.contains("| 1 => Some(#Active)"));
    }
}
