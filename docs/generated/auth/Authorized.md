# Authorized
> Defined in [auth.rs](../../../interface/src/interface/auth.rs)

## Route
- Method: `GET`
- Path: `test/test`
- Requires Auth Header: ✅

## Description
A wrapper around a response indicating whether a request that requires authorisation was
successful.

See `Success`'s value for the underlying type.

## Schema

| Type | Constraints |
| --- | --- |
| `String` | One of: `"Unauthorized"` |

*OR*

| Property | Required | Type | Constraints |
| --- | :---: | --- | --- |
| `Success` | ✅ | [T](.././T.md) |     | 


