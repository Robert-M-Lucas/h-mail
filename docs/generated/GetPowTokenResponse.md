# GetPowTokenResponse

Returns a POW token

### Type: `object`

| Property | Type | Required | Possible values | Deprecated | Default | Description | Examples |
| -------- | ---- | -------- | --------------- | ---------- | ------- | ----------- | -------- |
| expires_at | `integer` | ✅ | [SystemTimeField](#systemtimefield) |  |  |  |  |
| token | `string` | ✅ | [BigUintField](#biguintfield) |  |  |  |  |


---

# Definitions

## BigUintField

A base-64 (standard alphabet, with padding) little-endian encoding of a large unsigned integer

#### Type: `string`

## SystemTimeField

A timestamp represented as milliseconds since epoch

#### Type: `integer`


---

Markdown generated with [jsonschema-markdown](https://github.com/elisiariocouto/jsonschema-markdown).
