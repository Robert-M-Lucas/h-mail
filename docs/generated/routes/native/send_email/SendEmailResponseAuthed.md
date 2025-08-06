# SendEmailResponseAuthed

Returns whether sending the email succeeded and, if not, why

## Schema

| Type | Constraints |
| --- | --- |
| `String` | One of: `SendingFailed` |

*OR*

| Property | Required | Type | Constraints |
| --- | --- | --- | --- |
| `DeliverResponse` | ✅ | [DeliverEmailResponse](../../../routes/foreign/deliver_email/DeliverEmailResponse.md) |     | 


