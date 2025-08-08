# WithPow
> Defined in [WithPow.md.rs](../../../interface/src/interface/pow.rs)

## Description
A wrapper around a request requiring a proof-of-work (POW). The `token` is obtained from a
`GetPowTokenRequest`. The hash of `inner` (`inner.pow_hash()`) is squared `iters` times (modulo `token`) to obtain
`pow_result`.

See `inner`'s value for the underlying type.

## Schema

| Property | Required | Type | Constraints |
| --- | --- | --- | --- |
| `inner` | ✅ | [T](..//T.md) |     | 
| `iters` | ✅ | `Integer` | `uint32` - Bounds: [0, -] | 
| `pow_result` | ✅ | [BigUintField](../fields/big_uint/BigUintField.md) |     | 
| `token` | ✅ | [BigUintField](../fields/big_uint/BigUintField.md) |     | 


