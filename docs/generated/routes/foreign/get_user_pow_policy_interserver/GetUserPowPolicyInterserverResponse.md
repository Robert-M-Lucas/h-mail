# GetUserPowPolicyInterserverResponse
> Defined in [get_user_pow_policy_interserver.rs](../../../../../interface/src/interface/routes/foreign/get_user_pow_policy_interserver.rs)

## Description
Returns whether the user is whitelisted from POW (and the POW policy if not)

## Schema

| Type | Constraints |
| --- | --- |
| `String` | One of: `"SenderIpNotAuthed"`, `"BadRequest"`, `"UserDoesNotExist"` |

*OR*

| Property | Required | Type | Constraints |
| --- | :---: | --- | --- |
| `Whitelisted` | ✅ | [WhitelistedResponse](../../../routes/foreign/get_user_pow_policy_interserver/WhitelistedResponse.md) |  -  |


*OR*

| Property | Required | Type | Constraints |
| --- | :---: | --- | --- |
| `NotWhitelisted` | ✅ | [PowPolicy](../../../pow/PowPolicy.md) |  -  |


