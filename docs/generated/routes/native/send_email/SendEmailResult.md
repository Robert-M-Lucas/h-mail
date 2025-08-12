# SendEmailResult
> Defined in [send_email.rs](../../../../../interface/src/interface/routes/native/send_email.rs)

## Description
The result of trying to send an email

## Schema

| Type | Constraints |
| --- | --- |
| `String` | One of: `"Failed"` |

*OR*

| Property | Required | Type | Constraints |
| --- | :---: | --- | --- |
| `DeliveryResult` | ✅ | [DeliverEmailResponse](../../../routes/foreign/deliver_email/DeliverEmailResponse.md) |  -  |


