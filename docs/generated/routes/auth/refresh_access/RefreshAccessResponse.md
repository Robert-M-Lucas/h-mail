# RefreshAccessResponse
> Defined in [refresh_access.rs](../../../../../interface/src/interface/routes/auth/refresh_access.rs)

## Description
Returns an access token on success

## Schema

| Type | Constraints |
| --- | --- |
| `String` | One of: `"Failure"`, `"BadRequest"` |

*OR*

| Property | Required | Type | Constraints |
| --- | :---: | --- | --- |
| `Success` | ✅ | [AuthTokenDataField](../../../fields/auth_token/AuthTokenDataField.md) |     | 


