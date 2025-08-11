# GetEmailsEmail
> Defined in [get_emails.rs](../../../../../interface/src/interface/routes/native/get_emails.rs)

## Description
An individual email in a user's inbox

## Schema

| Property | Required | Type | Constraints |
| --- | :---: | --- | --- |
| `body` | ✅ | `String` |     | 
| `cc` | ✅ | `Array` | With items of type [EmailUser](../../../email/EmailUser.md) | 
| `content_type` | ✅ | `String` |     | 
| `hash` | ✅ | [BigUintField](../../../fields/big_uint/BigUintField.md) |     | 
| `mime_version` | ✅ | `String` |     | 
| `parent` |     | [BigUintField](../../../fields/big_uint/BigUintField.md) *OR* `null` |     | 
| `pow_classification` | ✅ | [PowClassification](../../../pow/PowClassification.md) |     | 
| `received_at` | ✅ | [SystemTimeField](../../../fields/system_time/SystemTimeField.md) |     | 
| `reply_to` |     | [EmailUser](../../../email/EmailUser.md) *OR* `null` |     | 
| `sent_at` | ✅ | [SystemTimeField](../../../fields/system_time/SystemTimeField.md) |     | 
| `source` | ✅ | `String` |     | 
| `subject` | ✅ | `String` |     | 
| `to` | ✅ | `Array` | With items of type [EmailUser](../../../email/EmailUser.md) | 


