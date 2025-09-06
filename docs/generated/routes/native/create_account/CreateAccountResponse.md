# CreateAccountResponse
> Defined in [create_account.rs](../../../../../interface/src/interface/routes/native/create_account.rs)

## Description
Returns whether the account creation succeeded and, if not, why

## Schema

| Type | Constraints |
| --- | --- |
| `String` | One of: `"UsernameInUse"` |

*OR*

| Property | Required | Type | Constraints |
| --- | :---: | --- | --- |
| `Success` | ✅ | `String` |  -  |


*OR*

| Property | Required | Type | Constraints |
| --- | :---: | --- | --- |
| `BadUsername` | ✅ | `String` |  -  |


*OR*

| Property | Required | Type | Constraints |
| --- | :---: | --- | --- |
| `BadPassword` | ✅ | `String` |  -  |


*OR*

| Property | Required | Type | Constraints |
| --- | :---: | --- | --- |
| `DoesNotMeetPolicy` | ✅ | `Integer` | `uint32` - Bounds: [0, -] |


*OR*

| Property | Required | Type | Constraints |
| --- | :---: | --- | --- |
| `PowFailure` | ✅ | [PowFailureReason](../../../pow/PowFailureReason.md) |  -  |


