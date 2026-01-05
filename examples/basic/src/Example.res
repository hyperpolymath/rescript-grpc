// SPDX-License-Identifier: MPL-2.0
// Example usage of generated protobuf types

open UserProto

// Create a user with the generated make function
// Required fields must be provided, optional fields use ~field=?
let alice = User.make(
  ~name="Alice",
  ~id=1,
  ~email="alice@example.com",  // optional field
  ~status=#StatusActive,
  ~tags=["admin", "developer"],
  ~address=Address.make(
    ~street="123 Main St",
    ~city="San Francisco",
    ~country="USA",
    ~postalCode="94102",  // optional field
  ),
)

// User without optional fields
let bob = User.make(
  ~name="Bob",
  ~id=2,
  ~status=#StatusInactive,
)

// Create a request (required field)
let request = GetUserRequest.make(~id=1)

// Create a list response
let response = ListUsersResponse.make(
  ~users=[alice, bob],
  ~nextPageToken="abc123",
)

// Convert enum to int
let statusCode = Status.toInt(#StatusActive) // 1

// Convert int to enum
let status = Status.fromInt(1) // Some(#StatusActive)

// Test JSON encode/decode round-trip
let testJsonCodec = () => {
  // Encode alice to JSON
  let aliceJson = User.toJson(alice)
  Console.log("Alice as JSON:")
  Console.log(Js.Json.stringify(aliceJson))

  // Decode back
  switch User.fromJson(aliceJson) {
  | Some(decoded) =>
    Console.log("Decoded user:")
    Console.log(decoded)
    // Verify fields match
    if decoded.name == alice.name && decoded.id == alice.id {
      Console.log("Round-trip successful!")
    } else {
      Console.error("Round-trip failed: fields don't match")
    }
  | None =>
    Console.error("Failed to decode JSON")
  }
}

// Log for testing
let main = () => {
  Console.log("=== Proto Type Tests ===")
  Console.log("Users created:")
  Console.log(alice)
  Console.log(bob)
  Console.log2("Status code:", statusCode)
  Console.log2("Status from int:", status)

  Console.log("\n=== JSON Codec Tests ===")
  testJsonCodec()
}

// Run main
main()
