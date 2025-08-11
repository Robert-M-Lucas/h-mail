# IsWhitelistedInterserverRequest
> Defined in [is_whitelisted_interserver.rs](../../../../../interface/src/interface/routes/foreign/is_whitelisted_interserver.rs)

## Route
- Path: `/foreign/is_whitelisted`
- Method: `POST`
- Requires [authentication](../../../../Flows/Authentication%20Flow.md): ❌

## Description
Asks whether a user is whitelisted from POW

## Schema

| Property | Required | Type | Constraints |
| --- | :---: | --- | --- |
| `ip_verification` | ✅ | [AuthTokenDataField](../../../fields/auth_token/AuthTokenDataField.md) |  -  |
| `recipient` | ✅ | `String` |  -  |
| `sender` | ✅ | `String` |  -  |
| `verify_ip_port` | ✅ | `Integer` | `uint16` - Bounds: [0, 65535] |


