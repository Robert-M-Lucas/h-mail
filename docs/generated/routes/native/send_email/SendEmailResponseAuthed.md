# SendEmailResponseAuthed
> Defined in [send_email.rs](../../../../../interface/src/interface/routes/native/send_email.rs)

## Description
Returns whether sending the email succeeded and, if not, why

## Schema

| Type | Constraints |
| --- | --- |
| `String` | One of: `"DuplicateDestination"`, `"BadRequest"` |

*OR*

| Property | Required | Type | Constraints |
| --- | :---: | --- | --- |
| `DeliverResponse` | ✅ | `Array` | With items of type [SendEmailResultPerDestination](../../../routes/native/send_email/SendEmailResultPerDestination.md) |


*OR*

| Property | Required | Type | Constraints |
| --- | :---: | --- | --- |
| `MissingPowFor` | ✅ | `String` |  -  |


