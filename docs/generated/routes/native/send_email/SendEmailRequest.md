# SendEmailRequest
> Defined in [send_email.rs](../../../../../interface/src/interface/routes/native/send_email.rs)

## Route
- Path: `/native/send_email`
- Method: `POST`
- Requires [authentication](../../../../Flows/Authentication%20Flow.md): ✅

## Description
POST: Requests the server sends an email to another server

AUTH: Requires an access token as the bearer token

## Schema

| Property | Required | Type | Constraints |
| --- | :---: | --- | --- |
| `email` | ✅ | [Email](../../../email/Email.md) ([WithPow](../../../pow/WithPow.md)\<[SendEmailPackage](../../../email/SendEmailPackage.md)\>) |     | 
| `solved_pows` | ✅ | `Array` | With items of type [SolvedPowFor](../../../routes/native/send_email/SolvedPowFor.md) | 


