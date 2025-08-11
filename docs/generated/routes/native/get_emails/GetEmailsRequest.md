# GetEmailsRequest
> Defined in [get_emails.rs](../../../../../interface/src/interface/routes/native/get_emails.rs)

## Route
- Path: `/native/get_emails`
- Method: `GET`
- Requires [authentication](../../../../Flows/Authentication%20Flow.md): ✅
- Response: [GetEmailsResponse](GetEmailsResponse.md)

## Description
Requests a user's emails

## Schema
> [!NOTE]
> This route expects query parameters (e.g. https://example.com/method?variable=value), not JSON

| Property | Required | Type | Constraints |
| --- | :---: | --- | --- |
| `since` | ✅ | [SystemTimeField](../../../fields/system_time/SystemTimeField.md) |  -  |


