# CheckPowResponse

Returns whether the POW was solved correctly and, if not, why

## Schema

| Type | Constraints |
| --- | --- |
| `String` | One of: `Success` |

*OR*

| Property | Required | Type | Constraints |
| --- | --- | --- | --- |
| `Failure` | ✅ | [[PowFailureReason]] |     | 


