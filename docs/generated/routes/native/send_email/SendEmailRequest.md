# SendEmailRequest
> Defined in [send_email.rs](../../../../../interface/src/interface/routes/native/send_email.rs)

## Route
- Path: `/native/send_email`
- Method: `POST`
- Requires [authentication](../../../../Flows/Authentication%20Flow.md): ✅
- Response: [SendEmailResponse](SendEmailResponse.md)

## Description
Requests the server sends an email to destinations specified in `email`.
Requires all destinations to have a POW solved in `solved_pows`.

## Schema

| Property | Required | Type | Constraints |
| --- | :---: | --- | --- |
| `bccs` | ✅ | `Array` | With items of type `String` |
| `email` | ✅ | [SendEmailPackage](../../../email/SendEmailPackage.md) |  -  |
| `solved_pows` | ✅ | `Array` | With items of type [SolvedPowFor](../../../routes/native/send_email/SolvedPowFor.md) |


