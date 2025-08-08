# CheckPowRequest
*(alias of [WithPow](../../pow/WithPow.md)\<[CheckPowPackage](../../routes/check_pow/CheckPowPackage.md)\>)* - see [WithPow](../../pow/WithPow.md) for description
> Defined in [CheckPowRequest.md.rs](../../../../interface/src/interface/routes/check_pow)

## Schema

| Property | Required | Type | Constraints |
| --- | --- | --- | --- |
| `inner` | ✅ | [CheckPowPackage](../../routes/check_pow/CheckPowPackage.md) |     | 
| `iters` | ✅ | `Integer` | `uint32` - Bounds: [0, -] | 
| `pow_result` | ✅ | [BigUintField](../../fields/big_uint/BigUintField.md) |     | 
| `token` | ✅ | [BigUintField](../../fields/big_uint/BigUintField.md) |     | 


