# DeliverEmailResponse
> Defined in [DeliverEmailResponse.md.rs](../../../routes/foreign/deliver_email/interface/src/interface/routes/foreign/deliver_email)

## Description
Returns whether the email delivery succeeded and, if not, why

## Schema

| Type | Constraints |
| --- | --- |
| `String` | One of: `"Success"`, `"UserNotFound"`, `"BadRequest"`, `"SenderIpNotAuthed"` |

*OR*

| Property | Required | Type | Constraints |
| --- | --- | --- | --- |
| `DoesNotMeetPolicy` | ✅ | [PowPolicy](../../../pow/PowPolicy.md) |     | 


*OR*

| Property | Required | Type | Constraints |
| --- | --- | --- | --- |
| `PowFailure` | ✅ | [PowFailureReason](../../../pow/PowFailureReason.md) |     | 


