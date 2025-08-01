# CheckPowRequest (WithPow)

A wrapper around a request requiring a proof-of-work (POW). The `token` is obtained from a
`GetPowTokenRequest`. Some hash of `inner` is squared `iters` times (modulo `token`) to obtain
`pow_result`.

See `inner`'s value for the underlying type.

### Type: `object`

| Property | Type | Required | Possible values | Deprecated | Default | Description | Examples |
| -------- | ---- | -------- | --------------- | ---------- | ------- | ----------- | -------- |
| inner | `object` | ✅ | [CheckPowPackage](#checkpowpackage) |  |  |  |  |
| iters | `integer` | ✅ | `0 <= x ` |  |  |  |  |
| pow_result | `string` | ✅ | [BigUintField](#biguintfield) |  |  |  |  |
| token | `string` | ✅ | [BigUintField](#biguintfield) |  |  |  |  |


---

# Definitions

## BigUintField

A base-64 (standard alphabet, with padding) little-endian encoding of a large unsigned integer

#### Type: `string`

## CheckPowPackage

POST: Utility function to check POW. Note that checking POW will invalidate the POW token,
preventing it from being used for other purposes. The hash of `challenge` will be used as the
hash for POW.

#### Type: `object`

| Property | Type | Required | Possible values | Deprecated | Default | Description | Examples |
| -------- | ---- | -------- | --------------- | ---------- | ------- | ----------- | -------- |
| challenge | `string` | ✅ | string |  |  |  |  |


---

Markdown generated with [jsonschema-markdown](https://github.com/elisiariocouto/jsonschema-markdown).
