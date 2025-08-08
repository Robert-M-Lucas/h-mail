# SendEmailResponseAuthed
> Defined in [SendEmailResponseAuthed.md.rs](../../../routes/native/send_email/interface/src/interface/routes/native/send_email)

## Description
Returns whether sending the email succeeded and, if not, why

## Schema

| Type | Constraints |
| --- | --- |
| `String` | One of: `"SendingFailed"` |

*OR*

| Property | Required | Type | Constraints |
| --- | --- | --- | --- |
| `DeliverResponse` | ✅ | [DeliverEmailResponse](../../../routes/foreign/deliver_email/DeliverEmailResponse.md) |     | 


