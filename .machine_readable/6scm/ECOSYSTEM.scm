;; SPDX-License-Identifier: MPL-2.0
;; ECOSYSTEM.scm - Ecosystem positioning for rescript-grpc
;; Media Type: application/vnd.ecosystem+scm

(ecosystem
 (version "1.0")
 (name "rescript-grpc")
 (type "library" "code-generator")
 (purpose "Protocol Buffers to ReScript codegen with gRPC-web support")

 (position-in-ecosystem
  (layer "NETWORK/API")
  (category "code-generation" "rpc" "serialization")
  (parent-ecosystem "rescript-full-stack")
  (role "Enables type-safe protobuf communication in ReScript applications"))

 (related-projects
  ((name "rescript-full-stack")
   (relationship umbrella-project)
   (url "https://github.com/hyperpolymath/rescript-full-stack")
   (integration "Listed as active component"))

  ((name "rescript-openapi")
   (relationship sibling-standard)
   (url "https://github.com/hyperpolymath/rescript-openapi")
   (integration "Similar codegen approach for OpenAPI vs Protobuf"))

  ((name "rescript-wasm-runtime")
   (relationship potential-consumer)
   (url "https://github.com/hyperpolymath/rescript-wasm-runtime")
   (integration "Future WASM codec could use SharedMemory for binary proto"))

  ((name "rescript-http-server")
   (relationship potential-consumer)
   (url "https://github.com/hyperpolymath/rescript-http-server")
   (integration "Server-side handlers could integrate with HTTP server"))

  ((name "prost")
   (relationship upstream-dependency)
   (url "https://github.com/tokio-rs/prost")
   (integration "Used for protobuf descriptor parsing in Rust"))

  ((name "@rescript/core")
   (relationship runtime-dependency)
   (url "https://github.com/rescript-lang/rescript-core")
   (integration "Runtime uses Core for BigInt, Array, Result types"))

  ((name "rescript-schema")
   (relationship inspiration)
   (url "https://github.com/DZakh/rescript-schema")
   (integration "JSON codec design influenced by rescript-schema patterns")))

 (what-this-is
  "A protoc plugin that generates type-safe ReScript code from .proto files"
  "JSON encode/decode codecs following proto3 JSON mapping"
  "gRPC-web client stubs for browser-based RPC calls"
  "Part of the Hyperpolymath ReScript full-stack ecosystem")

 (what-this-is-not
  "Not a full gRPC server implementation (HTTP/2 streaming)"
  "Not a binary protobuf codec (yet - WASM version planned)"
  "Not a replacement for direct protobuf bindings in Node.js"
  "Not tied to any specific backend framework"))
