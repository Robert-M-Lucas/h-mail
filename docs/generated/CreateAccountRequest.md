# CreateAccountRequest (WithPow)

A wrapper around a request requiring a proof-of-work (POW). The `token` is obtained from a
`GetPowTokenRequest`. Some hash of `inner` is squared `iters` times (modulo `token`) to obtain
`pow_result`.

See `inner`'s value for the underlying type.

### Type: `object`

| Property | Type | Required | Possible values | Deprecated | Default | Description | Examples |
| -------- | ---- | -------- | --------------- | ---------- | ------- | ----------- | -------- |
| inner | `object` | ✅ | [CreateAccountPackage](#createaccountpackage) |  |  |  |  |
| iters | `integer` | ✅ | `0 <= x ` |  |  |  |  |
| pow_result | `string` | ✅ | [BigUintField](#biguintfield) |  |  |  |  |
| token | `string` | ✅ | [BigUintField](#biguintfield) |  |  |  |  |


---

# Definitions

## BigUintField

A base-64 (standard alphabet, with padding) little-endian encoding of a large unsigned integer

#### Type: `string`

## CreateAccountPackage

POST: Requests an account be created. Requires POW burden obtained through
`GetCreateAccountPowPolicyRequest`. The hash of `username` will be used for the POW hash.

#### Type: `object`

| Property | Type | Required | Possible values | Deprecated | Default | Description | Examples |
| -------- | ---- | -------- | --------------- | ---------- | ------- | ----------- | -------- |
| password | `string` | ✅ | string |  |  |  |  |
| username | `string` | ✅ | string |  |  |  |  |


---

Markdown generated with [jsonschema-markdown](https://github.com/elisiariocouto/jsonschema-markdown).
