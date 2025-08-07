# CreateAccountRequest (alias of [WithPow](../../../pow/WithPow.md)\<[CreateAccountPackage](../../../routes/native/create_account/CreateAccountPackage.md)\>)

## Description:
See [WithPow](../../../pow/WithPow.md)

## Schema

| Property | Required | Type | Constraints |
| --- | --- | --- | --- |
| `inner` | ✅ | [CreateAccountPackage](../../../routes/native/create_account/CreateAccountPackage.md) |     | 
| `iters` | ✅ | `Integer` | `uint32` - Bounds: [0, -] | 
| `pow_result` | ✅ | [BigUintField](../../../fields/big_uint/BigUintField.md) |     | 
| `token` | ✅ | [BigUintField](../../../fields/big_uint/BigUintField.md) |     | 


