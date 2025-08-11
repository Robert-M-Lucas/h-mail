# RefreshAccessRequest
> Defined in [refresh_access.rs](../../../../../interface/src/interface/routes/auth/refresh_access.rs)

## Route
- Path: `/auth/refresh_access`
- Method: `POST`
- Requires [authentication](../../../../Flows/Authentication%20Flow.md): ❌
- Response: [RefreshAccessResponse](RefreshAccessResponse.md)

## Description
Requests a new access token authorised by a refresh token

## Schema

| Property | Required | Type | Constraints |
| --- | :---: | --- | --- |
| `refresh_token` | ✅ | [AuthTokenField](../../../fields/auth_token/AuthTokenField.md) |  -  |


