# GetHmailsHmail
> Defined in [get_hmails.rs](../../../../../interface/src/interface/routes/native/get_hmails.rs)

## Description
An individual h-mail in a user's inbox

## Schema

| Property | Required | Type | Constraints |
| --- | :---: | --- | --- |
| `body` | ✅ | `String` |  -  |
| `ccc` | ✅ | `Array` | With items of type [HmailUser](../../../hmail/HmailUser.md) |
| `hash` | ✅ | [BigUintField](../../../fields/big_uint/BigUintField.md) |  -  |
| `incrementing_id` | ✅ | `Integer` | `int32` |
| `parent` |    | [BigUintField](../../../fields/big_uint/BigUintField.md) *OR* `null` |  -  |
| `pow_classification` | ✅ | [PowClassification](../../../pow/PowClassification.md) |  -  |
| `received_at` | ✅ | [SystemTimeField](../../../fields/system_time/SystemTimeField.md) |  -  |
| `recipients` | ✅ | `Array` | With items of type [HmailUser](../../../hmail/HmailUser.md) |
| `reply_to` |    | [HmailUser](../../../hmail/HmailUser.md) *OR* `null` |  -  |
| `sent_at` | ✅ | [SystemTimeField](../../../fields/system_time/SystemTimeField.md) |  -  |
| `source` | ✅ | `String` |  -  |
| `subject` | ✅ | `String` |  -  |


