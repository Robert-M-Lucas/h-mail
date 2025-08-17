# SendHmailResponseAuthed
> Defined in [send_hmail.rs](../../../../../interface/src/interface/routes/native/send_hmail.rs)

## Description
Returns whether sending the h-mail succeeded and, if not, why for each recipient

## Schema

| Type | Constraints |
| --- | --- |
| `String` | One of: `"DuplicateDestination"`, `"BadRequest"` |

*OR*

| Property | Required | Type | Constraints |
| --- | :---: | --- | --- |
| `DeliverResponse` | ✅ | `Array` | With items of type [SendHmailResultPerDestination](../../../routes/native/send_hmail/SendHmailResultPerDestination.md) |


*OR*

| Property | Required | Type | Constraints |
| --- | :---: | --- | --- |
| `MissingPowFor` | ✅ | `String` |  -  |


