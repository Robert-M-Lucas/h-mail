# CreateAccountRequest
*Alias of [WithPow](../../../pow/WithPow.md)\<[CreateAccountPackage](../../../routes/native/create_account/CreateAccountPackage.md)\>* - see [WithPow](../../../pow/WithPow.md) for description
> Defined in [create_account.rs](../../../../../interface/src/interface/routes/native/create_account.rs)

## Route
- Path: `/native/create_account`
- Method: `POST`
- Requires [authentication](../../../../Flows/Authentication%20Flow.md): ❌

## Schema

| Property | Required | Type | Constraints |
| --- | :---: | --- | --- |
| `inner_dangerous` | ✅ | [CreateAccountPackage](../../../routes/native/create_account/CreateAccountPackage.md) |  -  |
| `pow_result` |    | [PowResult](../../../pow/PowResult.md) *OR* `null` |  -  |


