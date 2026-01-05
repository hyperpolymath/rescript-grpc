// Generated from user.proto by protoc-gen-rescript
// SPDX-License-Identifier: MPL-2.0
// DO NOT EDIT - regenerate from .proto source

// Package: example

module Status = {
  type t = [
    | #StatusUnknown
    | #StatusActive
    | #StatusInactive
    | #StatusSuspended
  ]

  let toInt = (v: t): int => {
    switch v {
    | #StatusUnknown => 0
    | #StatusActive => 1
    | #StatusInactive => 2
    | #StatusSuspended => 3
    }
  }

  let fromInt = (i: int): option<t> => {
    switch i {
    | 0 => Some(#StatusUnknown)
    | 1 => Some(#StatusActive)
    | 2 => Some(#StatusInactive)
    | 3 => Some(#StatusSuspended)
    | _ => None
    }
  }
}


module Address = {
  type t = {
    street: option<string>,
    city: option<string>,
    country: option<string>,
    postalCode: option<string>,
  }

  let make = (
    ~street=?,
    ~city=?,
    ~country=?,
    ~postalCode=?
  ): t => {
    street,
    city,
    country,
    postalCode,
  }
}


module User = {
  type t = {
    name: option<string>,
    id: option<int>,
    email: option<string>,
    status: option<Status.t>,
    tags: array<string>,
    address: option<Address.t>,
  }

  let make = (
    ~name=?,
    ~id=?,
    ~email=?,
    ~status=?,
    ~tags=[],
    ~address=?
  ): t => {
    name,
    id,
    email,
    status,
    tags,
    address,
  }
}


module ListUsersResponse = {
  type t = {
    users: array<User.t>,
    nextPageToken: option<string>,
  }

  let make = (
    ~users=[],
    ~nextPageToken=?
  ): t => {
    users,
    nextPageToken,
  }
}


module GetUserRequest = {
  type t = {
    id: option<int>,
  }

  let make = (
    ~id=?
  ): t => {
    id,
  }
}


module ListUsersRequest = {
  type t = {
    pageSize: option<int>,
    pageToken: option<string>,
  }

  let make = (
    ~pageSize=?,
    ~pageToken=?
  ): t => {
    pageSize,
    pageToken,
  }
}


