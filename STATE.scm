;; SPDX-License-Identifier: PMPL-1.0
;; STATE.scm - Project state for rescript-grpc

(state
  (metadata
    (version "0.1.0")
    (schema-version "1.0")
    (created "2024-06-01")
    (updated "2025-01-17")
    (project "rescript-grpc")
    (repo "hyperpolymath/rescript-grpc"))

  (project-context
    (name "ReScript gRPC")
    (tagline "Protocol Buffers to ReScript codegen with JSON codecs and gRPC-web stubs")
    (tech-stack ("rescript" "rust" "protobuf")))

  (current-position
    (phase "alpha")
    (overall-completion 35)
    (working-features
      ("Protobuf codegen"
       "JSON codecs"
       "gRPC-web client stubs"))))
