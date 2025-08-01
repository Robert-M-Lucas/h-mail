# DeliverEmailRequest

POST: Delivers an email from another server. The `ip_verification` token will be used in a
`VerifyIpRequest` to the `source_domain` on port `verify_ip_port` to ensure that the IP
is not being spoofed. Requires POW (in `email`) for which the hash of
`Email->inner (EmailPackage)` will be used as the POW hash.

### Type: `object`

| Property | Type | Required | Possible values | Deprecated | Default | Description | Examples |
| -------- | ---- | -------- | --------------- | ---------- | ------- | ----------- | -------- |
| email | `object` | ✅ | [WithPow](#withpow) |  |  |  |  |
| ip_verification | `object` | ✅ | [AuthTokenDataField](#authtokendatafield) |  |  |  |  |
| source_domain | `string` | ✅ | string |  |  |  |  |
| source_user | `string` | ✅ | string |  |  |  |  |
| verify_ip_port | `integer` | ✅ | `0 <= x <= 65535` |  |  |  |  |


---

# Definitions

## AuthTokenDataField

An `AuthToken` with attached expiry time

#### Type: `object`

| Property | Type | Required | Possible values | Deprecated | Default | Description | Examples |
| -------- | ---- | -------- | --------------- | ---------- | ------- | ----------- | -------- |
| expires_at | `integer` | ✅ | [SystemTimeField](#systemtimefield) |  |  |  |  |
| token | `string` | ✅ | [AuthTokenField](#authtokenfield) |  |  |  |  |

## AuthTokenField

Represents a base-64 encoded authentication token - you will not need to decode this.
Used in bearer tokens and in some requests.

#### Type: `string`

## BigUintField

A base-64 (standard alphabet, with padding) little-endian encoding of a large unsigned integer

#### Type: `string`

## EmailPackage

Represents an email being sent. The hash of this will be used for POW when sending emails.

#### Type: `object`

| Property | Type | Required | Possible values | Deprecated | Default | Description | Examples |
| -------- | ---- | -------- | --------------- | ---------- | ------- | ----------- | -------- |
| contents | `string` | ✅ | string |  |  |  |  |
| destination_user | `string` | ✅ | string |  |  |  |  |

## SystemTimeField

A timestamp represented as milliseconds since epoch

#### Type: `integer`

## WithPow

A wrapper around a request requiring a proof-of-work (POW). The `token` is obtained from a
`GetPowTokenRequest`. Some hash of `inner` is squared `iters` times (modulo `token`) to obtain
`pow_result`.

See `inner`'s value for the underlying type.

#### Type: `object`

| Property | Type | Required | Possible values | Deprecated | Default | Description | Examples |
| -------- | ---- | -------- | --------------- | ---------- | ------- | ----------- | -------- |
| inner | `object` | ✅ | [EmailPackage](#emailpackage) |  |  |  |  |
| iters | `integer` | ✅ | `0 <= x ` |  |  |  |  |
| pow_result | `string` | ✅ | [BigUintField](#biguintfield) |  |  |  |  |
| token | `string` | ✅ | [BigUintField](#biguintfield) |  |  |  |  |


---

Markdown generated with [jsonschema-markdown](https://github.com/elisiariocouto/jsonschema-markdown).
