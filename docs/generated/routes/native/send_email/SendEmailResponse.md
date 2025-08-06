# SendEmailResponse ([Authorized](../../../routes/native/send_email/SendEmailResponse.md)\<[SendEmailResponseAuthed](../../../routes/native/send_email/SendEmailResponseAuthed.md)\>)

## Description of `SendEmailResponseAuthed`
A wrapper around a response indicating whether a request that requires authorisation was
successful.

See `Success`'s value for the underlying type.

## Schema

| Type | Constraints |
| --- | --- |
| `String` | One of: `Unauthorized` |

*OR*

| Property | Required | Type | Constraints |
| --- | --- | --- | --- |
| `Success` | ✅ | [SendEmailResponseAuthed](../../../routes/native/send_email/SendEmailResponseAuthed.md) |     | 


