# GetHmailsHmail
> Defined in [get_hmails.rs](../../../../../interface/src/interface/routes/native/get_hmails.rs)

## Description
An individual h-mail in a user's inbox

## Schema

| Property | Required | Type | Constraints |
| --- | :---: | --- | --- |
| `body` | ✅ | `String` |  -  |
| `ccs` | ✅ | `Array` | With items of type [HmailUser](../../../hmail/HmailUser.md) |
| `hash` | ✅ | [BigUintField](../../../fields/big_uint/BigUintField.md) |  -  |
| `incrementing_id` | ✅ | `Integer` | `int64` |
| `is_context` | ✅ | `Boolean` |  -  |
| `parent` |    | [BigUintField](../../../fields/big_uint/BigUintField.md) *OR* `null` |  -  |
| `pow_classification` | ✅ | [PowClassification](../../../pow/PowClassification.md) |  -  |
| `random_id` | ✅ | `Integer` | `uint32` - Bounds: [0, -] |
| `received_at` | ✅ | [SystemTimeField](../../../fields/system_time/SystemTimeField.md) |  -  |
| `recipients` | ✅ | `Array` | With items of type [HmailUser](../../../hmail/HmailUser.md) |
| `reply_to` |    | [HmailUser](../../../hmail/HmailUser.md) *OR* `null` |  -  |
| `sender` | ✅ | [HmailUser](../../../hmail/HmailUser.md) |  -  |
| `sent_at` | ✅ | [SystemTimeField](../../../fields/system_time/SystemTimeField.md) |  -  |
| `subject` | ✅ | `String` |  -  |


