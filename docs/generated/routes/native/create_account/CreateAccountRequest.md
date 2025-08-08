# CreateAccountRequest
*(alias of [WithPow](../../../pow/WithPow.md)\<[CreateAccountPackage](../../../routes/native/create_account/CreateAccountPackage.md)\>)* - see [WithPow](../../../pow/WithPow.md) for description
> Defined in [CreateAccountRequest.md.rs](../../../../../interface/src/interface/routes/native/create_account.rs)

## Schema

| Property | Required | Type | Constraints |
| --- | --- | --- | --- |
| `inner` | ✅ | [CreateAccountPackage](../../../routes/native/create_account/CreateAccountPackage.md) |     | 
| `iters` | ✅ | `Integer` | `uint32` - Bounds: [0, -] | 
| `pow_result` | ✅ | [BigUintField](../../../fields/big_uint/BigUintField.md) |     | 
| `token` | ✅ | [BigUintField](../../../fields/big_uint/BigUintField.md) |     | 


