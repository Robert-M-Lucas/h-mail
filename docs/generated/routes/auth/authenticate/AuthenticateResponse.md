# AuthenticateResponse
> Defined in [AuthenticateResponse.md.rs](../../../../interface/src/interface/routes/auth/authenticate)

## Description
Returns a refresh token if successful

## Schema

| Type | Constraints |
| --- | --- |
| `String` | One of: `"Failure"` |

*OR*

| Property | Required | Type | Constraints |
| --- | --- | --- | --- |
| `Success` | ✅ | [AuthTokenDataField](../../../fields/auth_token/AuthTokenDataField.md) |     | 


