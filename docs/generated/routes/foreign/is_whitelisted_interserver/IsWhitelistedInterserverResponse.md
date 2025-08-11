# IsWhitelistedInterserverResponse
> Defined in [is_whitelisted_interserver.rs](../../../../../interface/src/interface/routes/foreign/is_whitelisted_interserver.rs)

## Description
Returns whether the user is whitelisted from POW (and the POW policy if not)

## Schema

| Type | Constraints |
| --- | --- |
| `String` | One of: `"SenderIpNotAuthed"`, `"BadRequest"` |

*OR*

| Property | Required | Type | Constraints |
| --- | :---: | --- | --- |
| `Whitelisted` | ✅ | [PowClassification](../../../pow/PowClassification.md) |  -  |


*OR*

| Property | Required | Type | Constraints |
| --- | :---: | --- | --- |
| `NotWhitelisted` | ✅ | [PowPolicy](../../../pow/PowPolicy.md) |  -  |


