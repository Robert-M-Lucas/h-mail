# DeliverEmailRequest

POST: Delivers an email from another server. The `ip_verification` token will be used in a
`VerifyIpRequest` to the `source_domain` on port `verify_ip_port` to ensure that the IP
is not being spoofed. Requires POW (in `email`) for which the hash of
`Email->inner (EmailPackage)` will be used as the POW hash.

## Schema

| Property | Required | Type | Constraints |
| --- | --- | --- | --- |
| `email` | ✅ | [[WithPow]]\<[[None]]\> |     | 
| `ip_verification` | ✅ | [[AuthTokenDataField]] |     | 
| `source_domain` | ✅ | `String` |     | 
| `source_user` | ✅ | `String` |     | 
| `verify_ip_port` | ✅ | `Integer` | `uint16` - Bounds: [0, 65535] | 


