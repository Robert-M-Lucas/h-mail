# DeliverEmailRequest
> Defined in [deliver_email.rs](../../../../../interface/src/interface/routes/foreign/deliver_email.rs)

## Route
- Path: `/foreign/deliver_email`
- Method: `POST`
- Requires [authentication](../../../../Flows/Authentication%20Flow.md): ❌
- Response: [DeliverEmailResponse](DeliverEmailResponse.md)

## Description
Delivers an email from another server. The `ip_verification` token will be used in a
`VerifyIpRequest` to the `source_domain` on port `verify_ip_port`, expecting a
`VerifyIpResponse` to ensure that the IP is not being spoofed. Requires POW (in `email`) for
which the hash of `Email->inner_dangerous (EmailPackage)` will be used as the POW hash. The
sender's IP will also be checked against the `source_domain`'s SPF records to ensure that the IP
is authorised by the domain to send emails.

## Schema

| Property | Required | Type | Constraints |
| --- | :---: | --- | --- |
| `destination_domain` | ✅ | `String` |  -  |
| `destination_user` | ✅ | `String` |  -  |
| `email` | ✅ | [Email](../../../email/Email.md) ([WithPow](../../../pow/WithPow.md)\<[SendEmailPackage](../../../email/SendEmailPackage.md)\>) |  -  |
| `ip_verification` | ✅ | [AuthTokenDataField](../../../fields/auth_token/AuthTokenDataField.md) |  -  |
| `source_domain` | ✅ | `String` |  -  |
| `source_user` | ✅ | `String` |  -  |
| `verify_ip_port` | ✅ | `Integer` | `uint16` - Bounds: [0, 65535] |


