# SendHmailResultPerDestination
> Defined in [send_hmail.rs](../../../../../interface/src/interface/routes/native/send_hmail.rs)

## Description
The result of trying to send an h-ail to one recipient

## Schema

| Property | Required | Type | Constraints |
| --- | :---: | --- | --- |
| `recipient` | ✅ | [HmailAddress](../../../fields/hmail_address/HmailAddress.md) |  -  |
| `result` | ✅ | [SendHmailResult](../../../routes/native/send_hmail/SendHmailResult.md) |  -  |


