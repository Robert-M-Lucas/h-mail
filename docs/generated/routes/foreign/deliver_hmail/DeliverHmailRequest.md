# DeliverHmailRequest
> Defined in [deliver_hmail.rs](../../../../../interface/src/interface/routes/foreign/deliver_hmail.rs)

## Route
- Path: `/foreign/deliver_hmail`
- Method: `POST`
- Requires [authentication](../../../../Flows/Authentication%20Flow.md): ❌
- Response: [DeliverHmailResponse](DeliverHmailResponse.md)

## Description
Delivers an h-mail from another server. The `ip_verification` token will be used in a
`VerifyIpRequest` to the `source_domain` on port `verify_ip_port`, expecting a
`VerifyIpResponse` to ensure that the IP is not being spoofed. Requires POW (in `hmail`) for
which the hash of `hmail->inner_dangerous (HmailPackage)` will be used as the POW hash. The
sender's IP will also be checked against the `source_domain`'s SPF records to ensure that the IP
is authorised by the domain to send emails.

## Schema

| Property | Required | Type | Constraints |
| --- | :---: | --- | --- |
| `context` | ✅ | `Array` | With items of type [SendHmailPackage](../../../hmail/SendHmailPackage.md) |
| `hmail` | ✅ | [Hmail](../../../hmail/Hmail.md) ([WithPow](../../../pow/WithPow.md)\<[SendHmailPackage](../../../hmail/SendHmailPackage.md)\>) |  -  |
| `ip_verification` | ✅ | [AuthTokenDataField](../../../fields/auth_token/AuthTokenDataField.md) |  -  |
| `recipient_address` | ✅ | [HmailAddress](../../../fields/hmail_address/HmailAddress.md) |  -  |
| `verify_ip_port` | ✅ | `Integer` | `uint16` - Bounds: [0, 65535] |


