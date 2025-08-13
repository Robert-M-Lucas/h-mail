# AuthTokenField
> Defined in [auth_token.rs](../../../../interface/src/interface/fields/auth_token.rs)

## Description
Represents a base-64 encoded authentication token - you will not need to decode this.
Used in bearer tokens for authentication and for sender IP verification.

Note that the length of this token is server-implementation-dependent. As the token will only
ever be decoded/checked/used by the server it issued from, there is no need for standardisation.

## Schema

| Type | Constraints |
| --- | --- |
| `String` | - |

