# GetHmailByHashRequest
> Defined in [get_hmail_by_hash.rs](../../../../../interface/src/interface/routes/native/get_hmail_by_hash.rs)

## Route
- Path: `/native/get_hmail_by_hash`
- Method: `GET`
- Requires [authentication](../../../../Flows/Authentication%20Flow.md): ✅
- Response: [GetHmailByHashResponse](GetHmailByHashResponse.md)

## Description
Requests a user's h-mails

## Schema
> [!NOTE]
> This route expects query parameters (e.g. https://example.com/method?variable=value), not JSON

| Property | Required | Type | Constraints |
| --- | :---: | --- | --- |
| `hash` | ✅ | [BigUintField](../../../fields/big_uint/BigUintField.md) |  -  |


