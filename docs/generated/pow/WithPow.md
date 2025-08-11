# WithPow
> Defined in [pow.rs](../../../interface/src/interface/pow.rs)

## Description
A wrapper around a request requiring a proof-of-work (POW). The `token` is obtained from a
`GetPowTokenRequest`. The hash of `inner` (`inner.pow_hash()`) is squared `pow_result.iters` times (modulo `pow_result.token`) to obtain
`pow_result.pow_result`.

See `inner`'s value for the underlying type.

## Schema

| Property | Required | Type | Constraints |
| --- | :---: | --- | --- |
| `inner` | ✅ | [T](.././T.md) |  -  |
| `pow_result` | ✅ | [PowResult](../pow/PowResult.md) |  -  |


