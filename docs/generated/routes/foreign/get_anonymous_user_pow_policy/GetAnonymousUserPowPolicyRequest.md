# GetAnonymousUserPowPolicyRequest
> Defined in [get_anonymous_user_pow_policy.rs](../../../../../interface/src/interface/routes/foreign/get_anonymous_user_pow_policy.rs)

## Route
- Path: `/foreign/get_anonymous_user_pow_policy`
- Method: `GET`
- Requires [authentication](../../../../Flows/Authentication%20Flow.md): ❌
- Response: [GetAnonymousUserPowPolicyResponse](GetAnonymousUserPowPolicyResponse.md)

## Description
Requests a user's POW policy. Use your servers `IsWhitelistedRequest` to get the POW policy,
also checking whether the sender is whitelisted and, therefore, does not need to complete POW.

## Schema
> [!NOTE]
> This route expects query parameters (e.g. https://example.com/method?variable=value), not JSON

| Property | Required | Type | Constraints |
| --- | :---: | --- | --- |
| `recipient_username` | ✅ | `String` |  -  |


