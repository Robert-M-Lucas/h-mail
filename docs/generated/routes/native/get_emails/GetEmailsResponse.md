# GetEmailsResponse ([Authorized](../../../auth/Authorized.md)\<[GetEmailsResponseAuthed](../../../routes/native/get_emails/GetEmailsResponseAuthed.md)\>)

## Description of `Authorized`
A wrapper around a response indicating whether a request that requires authorisation was
successful.

See `Success`'s value for the underlying type.

## Schema

| Type | Constraints |
| --- | --- |
| `String` | One of: `"Unauthorized"` |

*OR*

| Property | Required | Type | Constraints |
| --- | --- | --- | --- |
| `Success` | ✅ | [GetEmailsResponseAuthed](../../../routes/native/get_emails/GetEmailsResponseAuthed.md) |     | 


