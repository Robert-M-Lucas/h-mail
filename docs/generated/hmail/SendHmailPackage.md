# SendHmailPackage
> Defined in [hmail.rs](../../../interface/src/interface/hmail.rs)

## Description
Represents an email being sent. The email's hash is used to identify an email uniquely (for
replying to emails), with the `random_id` being used to differentiate two exactly identical
emails. As the `random_id` is client-chosen, the hash of the email should not be used as a UID
for servers as a client can easily construct two emails with identical hashes.

## Schema

| Property | Required | Type | Constraints |
| --- | :---: | --- | --- |
| `body` | ✅ | `String` |  -  |
| `ccs` | ✅ | `Array` | With items of type [HmailUser](../hmail/HmailUser.md) |
| `parent` |    | [BigUintField](../fields/big_uint/BigUintField.md) *OR* `null` |  -  |
| `random_id` | ✅ | `Integer` | `uint32` - Bounds: [0, -] |
| `recipients` | ✅ | `Array` | With items of type [HmailUser](../hmail/HmailUser.md) |
| `reply_to` |    | [HmailUser](../hmail/HmailUser.md) *OR* `null` |  -  |
| `sent_at` | ✅ | [SystemTimeField](../fields/system_time/SystemTimeField.md) |  -  |
| `subject` | ✅ | `String` |  -  |


