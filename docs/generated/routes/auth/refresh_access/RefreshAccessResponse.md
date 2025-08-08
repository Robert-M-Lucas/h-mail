# RefreshAccessResponse
> Defined in [RefreshAccessResponse.md.rs](../../../../../interface/src/interface/routes/auth/refresh_access)

## Description
Returns an access token on success

## Schema

| Type | Constraints |
| --- | --- |
| `String` | One of: `"Failure"`, `"BadRequest"` |

*OR*

| Property | Required | Type | Constraints |
| --- | --- | --- | --- |
| `Success` | ✅ | [AuthTokenDataField](../../../fields/auth_token/AuthTokenDataField.md) |     | 


