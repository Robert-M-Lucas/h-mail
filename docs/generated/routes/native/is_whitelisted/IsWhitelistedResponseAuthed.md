# IsWhitelistedResponseAuthed
> Defined in [is_whitelisted.rs](../../../../../interface/src/interface/routes/native/is_whitelisted.rs)

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
| `Whitelisted` | ✅ | [PowClassification](../../../pow/PowClassification.md) |  -  |


*OR*

| Property | Required | Type | Constraints |
| --- | :---: | --- | --- |
| `NotWhitelisted` | ✅ | [PowPolicy](../../../pow/PowPolicy.md) |  -  |


