# GetPowPolicyInterserverRequest
> Defined in [get_pow_policy_interserver.rs](../../../../../interface/src/interface/routes/foreign/get_pow_policy_interserver.rs)

## Route
- Path: `/foreign/get_pow_policy_interserver`
- Method: `POST`
- Requires [authentication](../../../../Flows/Authentication%20Flow.md): ❌
- Response: [GetPowPolicyInterserverResponse](GetPowPolicyInterserverResponse.md)

## Description
Asks whether a sender is whitelisted from POW by a user

## Schema

| Property | Required | Type | Constraints |
| --- | :---: | --- | --- |
| `ip_verification` | ✅ | [AuthTokenDataField](../../../fields/auth_token/AuthTokenDataField.md) |  -  |
| `recipient_username` | ✅ | `String` |  -  |
| `sender` | ✅ | [HmailAddress](../../../fields/hmail_address/HmailAddress.md) |  -  |
| `verify_ip_port` | ✅ | `Integer` | `uint16` - Bounds: [0, 65535] |


