# GetForeignPowPolicyRequest
> Defined in [get_user_pow_policy.rs](../../../../../interface/src/interface/routes/native/get_user_pow_policy.rs)

## Route
- Path: `/native/get_foreign_pow_policy`
- Method: `POST`
- Requires [authentication](../../../../Flows/Authentication%20Flow.md): ✅
- Response: [GetForeignPowPolicyResponse](GetForeignPowPolicyResponse.md)

## Description
Asks whether this authenticated user is whitelisted by the recipient

## Schema

| Property | Required | Type | Constraints |
| --- | :---: | --- | --- |
| `recipient` | ✅ | [HmailAddress](../../../fields/hmail_address/HmailAddress.md) |  -  |


