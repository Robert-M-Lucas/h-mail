# CreateAccountResponse
> Defined in [CreateAccountResponse.md.rs](../../../routes/native/create_account/../../interface/src/interface/routes/native/create_account)

## Description
Returns whether the account creation succeeded and, if not, why

## Schema

| Type | Constraints |
| --- | --- |
| `String` | One of: `"Success"`, `"BadUsername"`, `"UsernameInUse"`, `"BadPassword"` |

*OR*

| Property | Required | Type | Constraints |
| --- | --- | --- | --- |
| `DoesNotMeetPolicy` | ✅ | `Integer` | `uint32` - Bounds: [0, -] | 


*OR*

| Property | Required | Type | Constraints |
| --- | --- | --- | --- |
| `PowFailure` | ✅ | [PowFailureReason](../../../pow/PowFailureReason.md) |     | 


