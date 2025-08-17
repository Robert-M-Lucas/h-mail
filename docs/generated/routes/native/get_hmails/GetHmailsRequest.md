# GetHmailsRequest
> Defined in [get_hmails.rs](../../../../../interface/src/interface/routes/native/get_hmails.rs)

## Route
- Path: `/native/get_hmails`
- Method: `GET`
- Requires [authentication](../../../../Flows/Authentication%20Flow.md): ✅
- Response: [GetHmailsResponse](GetHmailsResponse.md)

## Description
Requests a user's h-mails

## Schema
> [!NOTE]
> This route expects query parameters (e.g. https://example.com/method?variable=value), not JSON

| Property | Required | Type | Constraints |
| --- | :---: | --- | --- |
| `since` | ✅ | [SystemTimeField](../../../fields/system_time/SystemTimeField.md) |  -  |


