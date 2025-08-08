# Authorized
> Defined in [Authorized.md.rs](../../../interface/src/interface/auth)

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
| --- | --- | --- | --- |
| `Success` | ✅ | [T](..//T.md) |     | 


