# VerifyIpRequest
> Defined in [VerifyIpRequest.md.rs](../../../routes/foreign/verify_ip/interface/src/interface/routes/foreign/verify_ip)

## Description
POST: A `DeliverEmailRequest` will cause the target server to issue a `VerifyIpRequest` back
to the sender to ensure the IP is not being spoofed. The `ip_verification` token verifies that
the IP belongs to the sender.

## Schema

| Property | Required | Type | Constraints |
| --- | --- | --- | --- |
| `ip_verification` | ✅ | [AuthTokenField](../../../fields/auth_token/AuthTokenField.md) |     | 


