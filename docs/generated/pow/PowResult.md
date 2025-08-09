# PowResult
> Defined in [pow.rs](../../../interface/src/interface/pow.rs)

## Description
The result of solving a POW token. Used in `WithPow`.

## Schema

| Property | Required | Type | Constraints |
| --- | :---: | --- | --- |
| `iters` | ✅ | `Integer` | `uint32` - Bounds: [0, -] | 
| `pow_result` | ✅ | [BigUintField](../fields/big_uint/BigUintField.md) |     | 
| `token` | ✅ | [BigUintField](../fields/big_uint/BigUintField.md) |     | 


