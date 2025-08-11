# CheckPowRequest
*Alias of [WithPow](../../pow/WithPow.md)\<[CheckPowPackage](../../routes/check_pow/CheckPowPackage.md)\>* - see [WithPow](../../pow/WithPow.md) for description
> Defined in [check_pow.rs](../../../../interface/src/interface/routes/check_pow.rs)

## Route
- Path: `/check_pow`
- Method: `POST`
- Requires [authentication](../../../Flows/Authentication%20Flow.md): ❌

## Schema

| Property | Required | Type | Constraints |
| --- | :---: | --- | --- |
| `inner` | ✅ | [CheckPowPackage](../../routes/check_pow/CheckPowPackage.md) |  -  |
| `pow_result` | ✅ | [PowResult](../../pow/PowResult.md) |  -  |


