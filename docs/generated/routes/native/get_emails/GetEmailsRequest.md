# GetEmailsRequest
> Defined in [get_emails.rs](../../../../../interface/src/interface/routes/native/get_emails.rs)

## Route
- Path: `/native/get_emails`
- Method: `GET`
- Requires [authentication](../../../../Flows/Authentication%20Flow.md): ✅

## Description
GET: Requests a user's emails

AUTH: Requires an access token as the bearer token

## Schema
> [!NOTE]
> This route expects query parameters (e.g. https://example.com/method?variable=value), not JSON

| Property | Required | Type | Constraints |
| --- | :---: | --- | --- |
| `since_id` | ✅ | `Integer` | `int32` | 


