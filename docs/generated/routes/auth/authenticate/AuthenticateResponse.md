# AuthenticateResponse
> Defined in [authenticate.rs](../../../../../interface/src/interface/routes/auth/authenticate.rs)

## Description
Returns a refresh token if successful

## Schema

| Type | Constraints |
| --- | --- |
| `String` | One of: `"Failure"` |

*OR*

| Property | Required | Type | Constraints |
| --- | :---: | --- | --- |
| `Success` | ✅ | [AuthTokenDataField](../../../fields/auth_token/AuthTokenDataField.md) |     | 


