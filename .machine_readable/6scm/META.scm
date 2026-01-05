;; SPDX-License-Identifier: MPL-2.0
;; META.scm - Meta-level information for rescript-grpc
;; Media Type: application/meta+scheme

(define meta
  `((architecture-decisions
     ((adr-001
       (status accepted)
       (date "2025-01-05")
       (title "Use Rust for protoc plugin")
       (context "Need to parse protobuf descriptors and generate ReScript code")
       (decision "Implement protoc plugin in Rust using prost-types for descriptor parsing")
       (consequences
        "Fast compilation"
        "Type-safe code generation"
        "Single binary distribution"
        "Rust expertise required for maintenance"))
      (adr-002
       (status accepted)
       (date "2025-01-05")
       (title "JSON-first codec approach")
       (context "gRPC-web commonly uses JSON encoding; binary protobuf requires WASM")
       (decision "Implement JSON codecs first, defer binary protobuf to WASM codec")
       (consequences
        "Simpler initial implementation"
        "Works without WASM runtime"
        "Larger message sizes than binary"
        "Compatible with gRPC-web JSON mode"))
      (adr-003
       (status accepted)
       (date "2025-01-05")
       (title "Polymorphic variants for enums")
       (context "ReScript enums can be modeled as variants or polymorphic variants")
       (decision "Use polymorphic variants (#StatusActive) for proto enums")
       (consequences
        "Interoperable across modules without explicit imports"
        "Structural typing for flexibility"
        "JSON serialization as integers preserved"))
      (adr-004
       (status accepted)
       (date "2025-01-05")
       (title "Opt-in gRPC-web clients")
       (context "Not all users need HTTP client code generation")
       (decision "Generate service clients only with --rescript_opt=grpc flag")
       (consequences
        "Smaller output by default"
        "No fetch dependency for type-only use"
        "Users must explicitly enable for RPC calls"))))

    (development-practices
     (code-style
      (language "Rust" "ReScript")
      (formatter "rustfmt" "rescript format")
      (linter "clippy" "rescript compiler"))
     (security
      (audit "cargo audit")
      (review "code review required for protoc plugin"))
     (testing
      (unit "cargo test")
      (integration "rescript build + node/deno execution"))
     (versioning "semver")
     (documentation "AsciiDoc")
     (branching "trunk-based"))

    (design-rationale
     (why-rust-not-typescript
      "Protoc plugins receive binary protobuf on stdin"
      "Rust has excellent protobuf support via prost"
      "Single binary distribution without runtime dependencies"
      "Type safety during code generation")
     (why-json-not-binary
      "gRPC-web commonly uses JSON over HTTP/2"
      "Browser compatibility without WASM"
      "Debugging ease with human-readable messages"
      "Binary codec can be added later via WASM")
     (why-topological-sort
      "ReScript requires types to be defined before use"
      "Proto messages can reference each other in any order"
      "Sorting ensures generated code compiles without forward declarations"))))
