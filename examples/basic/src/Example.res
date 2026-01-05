// SPDX-License-Identifier: MPL-2.0
// Example usage of generated protobuf types

open UserProto

// Create a user with the generated make function
let alice = User.make(
  ~name="Alice",
  ~id=1,
  ~email="alice@example.com",
  ~status=#StatusActive,
  ~tags=["admin", "developer"],
  ~address=Address.make(
    ~street="123 Main St",
    ~city="San Francisco",
    ~country="USA",
    ~postalCode="94102",
  ),
)

// Create a request
let request = GetUserRequest.make(~id=1)

// Create a list response
let response = ListUsersResponse.make(
  ~users=[alice],
  ~nextPageToken="abc123",
)

// Convert enum to int
let statusCode = Status.toInt(#StatusActive) // 1

// Convert int to enum
let status = Status.fromInt(1) // Some(#StatusActive)

// Log for testing
let main = () => {
  Console.log("User created:")
  Console.log(alice)
  Console.log("Status code:", statusCode)
  Console.log("Status from int:", status)
}
