# SetPowPolicyRequest
> Defined in [set_pow_policy.rs](../../../../../interface/src/interface/routes/native/set_pow_policy.rs)

## Route
- Path: `/native/set_pow_policy`
- Method: `POST`
- Requires [authentication](../../../../Flows/Authentication%20Flow.md): ✅
- Response: [SetPowPolicyResponse](SetPowPolicyResponse.md)

## Description
Changes the authenticated user's POW policy

## Schema

| Property | Required | Type | Constraints |
| --- | :---: | --- | --- |
| `policy` | ✅ | [PowPolicy](../../../pow/PowPolicy.md) |  -  |


