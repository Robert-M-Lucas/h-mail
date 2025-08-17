# GetHmailsHmail
> Defined in [get_hmails.rs](../../../../../interface/src/interface/routes/native/get_hmails.rs)

## Description
An individual h-mail in a user's inbox

## Schema

| Property | Required | Type | Constraints |
| --- | :---: | --- | --- |
| `body` | ✅ | `String` |  -  |
| `cc` | ✅ | `Array` | With items of type [HmailUser](../../../hmail/HmailUser.md) |
| `hash` | ✅ | [BigUintField](../../../fields/big_uint/BigUintField.md) |  -  |
| `parent` |    | [BigUintField](../../../fields/big_uint/BigUintField.md) *OR* `null` |  -  |
| `pow_classification` | ✅ | [PowClassification](../../../pow/PowClassification.md) |  -  |
| `received_at` | ✅ | [SystemTimeField](../../../fields/system_time/SystemTimeField.md) |  -  |
| `reply_to` |    | [HmailUser](../../../hmail/HmailUser.md) *OR* `null` |  -  |
| `sent_at` | ✅ | [SystemTimeField](../../../fields/system_time/SystemTimeField.md) |  -  |
| `source` | ✅ | `String` |  -  |
| `subject` | ✅ | `String` |  -  |
| `to` | ✅ | `Array` | With items of type [HmailUser](../../../hmail/HmailUser.md) |


