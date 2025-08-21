# GetForeignPowPolicyResponseAuthed
> Defined in [get_foreign_pow_policy.rs](../../../../../interface/src/interface/routes/native/get_foreign_pow_policy.rs)

## Description
Returns whether this authenticated user is whitelisted by the recipient (and their POW policy
if not)

## Schema

| Type | Constraints |
| --- | --- |
| `String` | One of: `"RequestFailed"`, `"BadRequest"` |

*OR*

| Property | Required | Type | Constraints |
| --- | :---: | --- | --- |
| `Whitelisted` | ✅ | [ForeignWhitelistedResponse](../../../routes/native/get_foreign_pow_policy/ForeignWhitelistedResponse.md) |  -  |


*OR*

| Property | Required | Type | Constraints |
| --- | :---: | --- | --- |
| `NotWhitelisted` | ✅ | [PowPolicy](../../../pow/PowPolicy.md) |  -  |


