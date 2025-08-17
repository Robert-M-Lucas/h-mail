# SendHmailResult
> Defined in [send_hmail.rs](../../../../../interface/src/interface/routes/native/send_hmail.rs)

## Description
The result of trying to send an h-mail

## Schema

| Type | Constraints |
| --- | --- |
| `String` | One of: `"Failed"` |

*OR*

| Property | Required | Type | Constraints |
| --- | :---: | --- | --- |
| `DeliveryResult` | ✅ | [DeliverHMmailResponse](../../../routes/foreign/deliver_hmail/DeliverHMmailResponse.md) |  -  |


