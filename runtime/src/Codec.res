// SPDX-License-Identifier: MPL-2.0
// WASM codec wrapper for protobuf encode/decode

// Re-export Wasm types from rescript-wasm-runtime
module Memory = Wasm.Memory
module Allocator = Wasm.Allocator

// Codec instance holding loaded WASM module
type t = {
  instance: Wasm.Instance.t,
  memory: Memory.t,
  allocator: Allocator.t,
}

// Load codec from WASM file
let load = async (wasmPath: string): t => {
  let instance = await Wasm.loadModule(wasmPath, ())
  let exports = Wasm.Instance.exports(instance)
  let memory: Memory.t = %raw(`exports.memory`)
  let allocator = Allocator.fromExports(exports)
  {instance, memory, allocator}
}

// Encode a message to bytes using WASM codec
let encode = (
  codec: t,
  encodeFn: Allocator.ptr => (Allocator.ptr, int),
  serialize: 'msg => Js.Typed_array.Uint8Array.t,
  msg: 'msg,
): Js.Typed_array.Uint8Array.t => {
  // Serialize message to intermediate format
  let msgBytes = serialize(msg)

  // Allocate in WASM memory and copy
  let inputPtr = Allocator.allocBytes(codec.allocator, codec.memory, msgBytes)

  // Call WASM encode function
  let (outputPtr, outputLen) = encodeFn(inputPtr)

  // Copy result back and free
  let result = Allocator.readAndFree(codec.allocator, codec.memory, outputPtr, outputLen)

  // Free input buffer
  codec.allocator.free(inputPtr)

  result
}

// Decode bytes to a message using WASM codec
let decode = (
  codec: t,
  decodeFn: (Allocator.ptr, int) => Allocator.ptr,
  deserialize: Js.Typed_array.Uint8Array.t => 'msg,
  bytes: Js.Typed_array.Uint8Array.t,
): 'msg => {
  // Allocate in WASM memory and copy input
  let inputPtr = Allocator.allocBytes(codec.allocator, codec.memory, bytes)
  let inputLen = Js.Typed_array.Uint8Array.length(bytes)

  // Call WASM decode function
  let outputPtr = decodeFn(inputPtr, inputLen)

  // Read length from first 4 bytes of output
  let outputLen = Memory.readInt32(codec.memory, outputPtr)

  // Copy result (skip length prefix)
  let result = Memory.copyFromWasm(codec.memory, outputPtr + 4, outputLen)

  // Free buffers
  codec.allocator.free(inputPtr)
  codec.allocator.free(outputPtr)

  // Deserialize to message type
  deserialize(result)
}

// Error codes from WASM codec
module Error = {
  type t =
    | DecodeError(int)
    | EncodeError(int)
    | AllocationFailed

  let fromCode = (code: int): t => {
    if code < 0 {
      DecodeError(-code)
    } else {
      EncodeError(code)
    }
  }

  let toString = (err: t): string => {
    switch err {
    | DecodeError(code) => `Decode error: ${Int.toString(code)}`
    | EncodeError(code) => `Encode error: ${Int.toString(code)}`
    | AllocationFailed => "WASM memory allocation failed"
    }
  }
}
