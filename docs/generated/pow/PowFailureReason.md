# PowFailureReason

Reason for a POW check failing

## Schema

| Type | Constraints |
| --- | --- |
| `String` | One of: `FailedNoRetry`, `NotFoundCanRetry`, `BadRequestCanRetry` |

*OR*

| Property | Required | Type | Constraints |
| --- | --- | --- | --- |
| `DoesNotMeetPolicyMinimum` | ✅ | `Integer` | `uint32` - Bounds: [0, -] | 


