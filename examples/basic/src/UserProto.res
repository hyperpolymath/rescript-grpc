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


module GetUserRequest = {
  type t = {
    id: int,
  }

  let make = (
    ~id
  ): t => {
    id,
  }

  // JSON serialization
  let toJson = (msg: t): Js.Json.t => {
    Json.Encode.object(Json.Encode.fields(
      [
        Json.Encode.required("id", msg.id, Json.Encode.int),
      ],
      [
      ],
    ))
  }

  // JSON deserialization
  let fromJson = (json: Js.Json.t): option<t> => {
    switch Json.Decode.object(json) {
    | Some(obj) =>
        let id = Json.Decode.required(obj, "id", Json.Decode.int)
        switch (id) {
        | (Ok(id)) =>
          Some({
            id,
          })
        | _ => None
        }
    | None => None
    }
  }
}


module ListUsersRequest = {
  type t = {
    pageSize: int,
    pageToken: string,
  }

  let make = (
    ~pageSize,
    ~pageToken
  ): t => {
    pageSize,
    pageToken,
  }

  // JSON serialization
  let toJson = (msg: t): Js.Json.t => {
    Json.Encode.object(Json.Encode.fields(
      [
        Json.Encode.required("page_size", msg.pageSize, Json.Encode.int),
        Json.Encode.required("page_token", msg.pageToken, Json.Encode.string),
      ],
      [
      ],
    ))
  }

  // JSON deserialization
  let fromJson = (json: Js.Json.t): option<t> => {
    switch Json.Decode.object(json) {
    | Some(obj) =>
        let pageSize = Json.Decode.required(obj, "page_size", Json.Decode.int)
        let pageToken = Json.Decode.required(obj, "page_token", Json.Decode.string)
        switch (pageSize, pageToken) {
        | (Ok(pageSize), Ok(pageToken)) =>
          Some({
            pageSize,
            pageToken,
          })
        | _ => None
        }
    | None => None
    }
  }
}


module Address = {
  type t = {
    street: string,
    city: string,
    country: string,
    postalCode: option<string>,
  }

  let make = (
    ~street,
    ~city,
    ~country,
    ~postalCode=?
  ): t => {
    street,
    city,
    country,
    postalCode,
  }

  // JSON serialization
  let toJson = (msg: t): Js.Json.t => {
    Json.Encode.object(Json.Encode.fields(
      [
        Json.Encode.required("street", msg.street, Json.Encode.string),
        Json.Encode.required("city", msg.city, Json.Encode.string),
        Json.Encode.required("country", msg.country, Json.Encode.string),
      ],
      [
        Json.Encode.optional("postal_code", msg.postalCode, Json.Encode.string),
      ],
    ))
  }

  // JSON deserialization
  let fromJson = (json: Js.Json.t): option<t> => {
    switch Json.Decode.object(json) {
    | Some(obj) =>
        let street = Json.Decode.required(obj, "street", Json.Decode.string)
        let city = Json.Decode.required(obj, "city", Json.Decode.string)
        let country = Json.Decode.required(obj, "country", Json.Decode.string)
        let postalCode = Json.Decode.optional(obj, "postal_code", Json.Decode.string)->Result.getOr(None)
        switch (street, city, country) {
        | (Ok(street), Ok(city), Ok(country)) =>
          Some({
            street,
            city,
            country,
            postalCode,
          })
        | _ => None
        }
    | None => None
    }
  }
}


module User = {
  type t = {
    name: string,
    id: int,
    email: option<string>,
    status: Status.t,
    tags: array<string>,
    address: option<Address.t>,
  }

  let make = (
    ~name,
    ~id,
    ~email=?,
    ~status,
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

  // JSON serialization
  let toJson = (msg: t): Js.Json.t => {
    Json.Encode.object(Json.Encode.fields(
      [
        Json.Encode.required("name", msg.name, Json.Encode.string),
        Json.Encode.required("id", msg.id, Json.Encode.int),
        Json.Encode.required("status", msg.status, Status.toInt->Json.Encode.int),
      ],
      [
        Json.Encode.optional("email", msg.email, Json.Encode.string),
        Json.Encode.repeated("tags", msg.tags, Json.Encode.string),
        Json.Encode.optional("address", msg.address, Address.toJson),
      ],
    ))
  }

  // JSON deserialization
  let fromJson = (json: Js.Json.t): option<t> => {
    switch Json.Decode.object(json) {
    | Some(obj) =>
        let name = Json.Decode.required(obj, "name", Json.Decode.string)
        let id = Json.Decode.required(obj, "id", Json.Decode.int)
        let email = Json.Decode.optional(obj, "email", Json.Decode.string)->Result.getOr(None)
        let status = Json.Decode.required(obj, "status", json => Json.Decode.int(json)->Option.flatMap(Status.fromInt))
        let tags = Json.Decode.repeated(obj, "tags", Json.Decode.string)->Result.getOr([])
        let address = Json.Decode.optional(obj, "address", Address.fromJson)->Result.getOr(None)
        switch (name, id, status) {
        | (Ok(name), Ok(id), Ok(status)) =>
          Some({
            name,
            id,
            email,
            status,
            tags,
            address,
          })
        | _ => None
        }
    | None => None
    }
  }
}


module ListUsersResponse = {
  type t = {
    users: array<User.t>,
    nextPageToken: string,
  }

  let make = (
    ~users=[],
    ~nextPageToken
  ): t => {
    users,
    nextPageToken,
  }

  // JSON serialization
  let toJson = (msg: t): Js.Json.t => {
    Json.Encode.object(Json.Encode.fields(
      [
        Json.Encode.required("next_page_token", msg.nextPageToken, Json.Encode.string),
      ],
      [
        Json.Encode.repeated("users", msg.users, User.toJson),
      ],
    ))
  }

  // JSON deserialization
  let fromJson = (json: Js.Json.t): option<t> => {
    switch Json.Decode.object(json) {
    | Some(obj) =>
        let users = Json.Decode.repeated(obj, "users", User.fromJson)->Result.getOr([])
        let nextPageToken = Json.Decode.required(obj, "next_page_token", Json.Decode.string)
        switch (nextPageToken) {
        | (Ok(nextPageToken)) =>
          Some({
            users,
            nextPageToken,
          })
        | _ => None
        }
    | None => None
    }
  }
}


