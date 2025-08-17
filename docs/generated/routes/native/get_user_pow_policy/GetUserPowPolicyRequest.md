# GetUserPowPolicyRequest
> Defined in [get_user_pow_policy.rs](../../../../../interface/src/interface/routes/native/get_user_pow_policy.rs)

## Route
- Path: `/native/get_user_pow_policy`
- Method: `POST`
- Requires [authentication](../../../../Flows/Authentication%20Flow.md): ✅
- Response: [GetUserPowPolicyResponse](GetUserPowPolicyResponse.md)

## Description
Asks whether this authenticated user is whitelisted by the recipient

## Schema

| Property | Required | Type | Constraints |
| --- | :---: | --- | --- |
| `address` | ✅ | `String` |  -  |


