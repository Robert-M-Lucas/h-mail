# SendEmailRequest

POST: Requests the server sends an email to another server

AUTH: Requires an access token as the bearer token

### Type: `object`

| Property | Type | Required | Possible values | Deprecated | Default | Description | Examples |
| -------- | ---- | -------- | --------------- | ---------- | ------- | ----------- | -------- |
| destination_domain | `string` | ✅ | string |  |  |  |  |
| email | `object` | ✅ | [WithPow](#withpow) |  |  |  |  |


---

# Definitions

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
