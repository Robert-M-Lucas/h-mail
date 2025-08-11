# IsWhitelistedRequest
> Defined in [is_whitelisted.rs](../../../../../interface/src/interface/routes/native/is_whitelisted.rs)

## Route
- Path: `/native/is_whitelisted`
- Method: `POST`
- Requires [authentication](../../../../Flows/Authentication%20Flow.md): ✅
- Response: [IsWhitelistedResponse](IsWhitelistedResponse.md)

## Description
Asks whether this authenticated user is whitelisted by the recipient

## Schema

| Property | Required | Type | Constraints |
| --- | :---: | --- | --- |
| `recipient` | ✅ | `String` |  -  |


