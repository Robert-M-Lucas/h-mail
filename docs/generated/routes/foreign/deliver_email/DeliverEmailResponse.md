# DeliverEmailResponse
> Defined in [deliver_email.rs](../../../../../interface/src/interface/routes/foreign/deliver_email.rs)

## Description
Returns whether the email delivery succeeded and, if not, why

## Schema

| Type | Constraints |
| --- | --- |
| `String` | One of: `"Success"`, `"UserNotFound"`, `"BadRequest"`, `"SenderIpNotAuthed"` |

*OR*

| Property | Required | Type | Constraints |
| --- | :---: | --- | --- |
| `DoesNotMeetPolicy` | ✅ | [PowPolicy](../../../pow/PowPolicy.md) |     | 


*OR*

| Property | Required | Type | Constraints |
| --- | :---: | --- | --- |
| `PowFailure` | ✅ | [PowFailureReason](../../../pow/PowFailureReason.md) |     | 


