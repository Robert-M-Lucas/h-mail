# AuthenticateRequest
> Defined in [authenticate.rs](../../../../../interface/src/interface/routes/auth/authenticate.rs)

## Route
- Path: `/auth/authenticate`
- Method: `GET`
- Requires [authentication](../../../../Flows/Authentication%20Flow.md): ✅

## Description
POST: Requests an access token using a username and password

## Schema
> [!NOTE]
> This route expects query parameters (e.g. https://example.com/method?variable=value), not JSON

| Property | Required | Type | Constraints |
| --- | :---: | --- | --- |
| `password` | ✅ | `String` |     | 
| `username` | ✅ | `String` |     | 


