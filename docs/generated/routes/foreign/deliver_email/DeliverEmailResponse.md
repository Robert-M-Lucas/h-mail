# DeliverEmailResponse

Returns whether the email delivery succeeded and, if not, why

## Schema

| Type | Constraints |
| --- | --- |
| `String` | One of: `Success`, `UserNotFound`, `BadRequest`, `SenderIpNotAuthed` |

*OR*

| Property | Required | Type | Constraints |
| --- | --- | --- | --- |
| `DoesNotMeetPolicy` | ✅ | [[PowPolicy]] |     | 


*OR*

| Property | Required | Type | Constraints |
| --- | --- | --- | --- |
| `PowFailure` | ✅ | [[PowFailureReason]] |     | 


