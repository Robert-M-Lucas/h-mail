# VerifyIpRequest
> Defined in [verify_ip.rs](../../../../../interface/src/interface/routes/foreign/verify_ip.rs)

## Route
- Path: `/foreign/verify_ip`
- Method: `POST`
- Requires [authentication](../../../../Flows/Authentication%20Flow.md): ❌
- Response: [VerifyIpResponse](VerifyIpResponse.md)

## Description
A `DeliverEmailRequest` will cause the target server to issue a `VerifyIpRequest` back
to the sender to ensure the IP is not being spoofed. The `ip_verification` token should be
the same as the one in `DeliverEmailRequest`.

## Schema

| Property | Required | Type | Constraints |
| --- | :---: | --- | --- |
| `ip_verification` | ✅ | [AuthTokenField](../../../fields/auth_token/AuthTokenField.md) |  -  |


