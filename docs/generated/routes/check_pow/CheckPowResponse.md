# CheckPowResponse
> Defined in [CheckPowResponse.md.rs](../../routes/check_pow/interface/src/interface/routes/check_pow)

## Description
Returns whether the POW was solved correctly and, if not, why

## Schema

| Type | Constraints |
| --- | --- |
| `String` | One of: `"Success"` |

*OR*

| Property | Required | Type | Constraints |
| --- | --- | --- | --- |
| `Failure` | ✅ | [PowFailureReason](../../pow/PowFailureReason.md) |     | 


