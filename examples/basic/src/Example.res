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

// Log for testing
let main = () => {
  Console.log("Users created:")
  Console.log(alice)
  Console.log(bob)
  Console.log("Status code:", statusCode)
  Console.log("Status from int:", status)
}
