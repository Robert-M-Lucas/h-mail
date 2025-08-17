# DeliverHmailResponse
> Defined in [deliver_hmail.rs](../../../../../interface/src/interface/routes/foreign/deliver_hmail.rs)

## Description
Returns whether the h-mail delivery succeeded and, if not, why

## Schema

| Type | Constraints |
| --- | --- |
| `String` | One of: `"Success"`, `"UserNotFound"`, `"BadRequest"`, `"SenderIpNotAuthed"`, `"WrongDomain"` |

*OR*

| Property | Required | Type | Constraints |
| --- | :---: | --- | --- |
| `DoesNotMeetPolicy` | ✅ | [PowPolicy](../../../pow/PowPolicy.md) |  -  |


*OR*

| Property | Required | Type | Constraints |
| --- | :---: | --- | --- |
| `PowFailure` | ✅ | [PowFailureReason](../../../pow/PowFailureReason.md) |  -  |


