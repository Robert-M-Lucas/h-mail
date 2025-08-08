# AuthenticateRequest
> Defined in [authenticate.rs](../../../../../interface/src/interface/routes/auth/authenticate.rs)

## Route
- Path: `/auth/authenticate`
- Method: `GET`
- Requires authentication: `✅`

## Description
POST: Requests an access token using a username and password

## Schema

| Property | Required | Type | Constraints |
| --- | :---: | --- | --- |
| `password` | ✅ | `String` |     | 
| `username` | ✅ | `String` |     | 


