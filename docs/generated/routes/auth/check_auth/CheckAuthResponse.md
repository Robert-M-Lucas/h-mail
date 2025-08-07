# CheckAuthResponse ([Authorized](../../../routes/auth/check_auth/CheckAuthResponse.md)\<[CheckAuthResponseAuthed](../../../routes/auth/check_auth/CheckAuthResponseAuthed.md)\>)

## Description of `Authorized`
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
| `Success` | ✅ | [CheckAuthResponseAuthed](../../../routes/auth/check_auth/CheckAuthResponseAuthed.md) |     | 


