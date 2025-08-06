# CreateAccountRequest ([WithPow](../../../routes/native/create_account/CreateAccountRequest.md)\<[CreateAccountPackage](../../../routes/native/create_account/CreateAccountPackage.md)\>)

## Description of `CreateAccountPackage`
A wrapper around a request requiring a proof-of-work (POW). The `token` is obtained from a
`GetPowTokenRequest`. Some hash of `inner` is squared `iters` times (modulo `token`) to obtain
`pow_result`.

See `inner`'s value for the underlying type.

## Schema

| Property | Required | Type | Constraints |
| --- | --- | --- | --- |
| `inner` | ✅ | [CreateAccountPackage](../../../routes/native/create_account/CreateAccountPackage.md) |     | 
| `iters` | ✅ | `Integer` | `uint32` - Bounds: [0, -] | 
| `pow_result` | ✅ | [BigUintField](../../../fields/big_uint/BigUintField.md) |     | 
| `token` | ✅ | [BigUintField](../../../fields/big_uint/BigUintField.md) |     | 


