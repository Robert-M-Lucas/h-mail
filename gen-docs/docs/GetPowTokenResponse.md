# GetPowTokenResponse

JSON Schema missing a description, provide it using the `description` key in the root of the JSON document.

### Type: `object`

| Property | Type | Required | Possible values | Deprecated | Default | Description | Examples |
| -------- | ---- | -------- | --------------- | ---------- | ------- | ----------- | -------- |
| expires_at | `integer` | ✅ | [SystemTimeField](#systemtimefield) |  |  |  |  |
| token | `string` | ✅ | [BigUintField](#biguintfield) |  |  |  |  |


---

# Definitions

## BigUintField

A base-64 little-endian encoding of a large unsigned integer

#### Type: `string`

## SystemTimeField

A timestamp represented as milliseconds since epoch

#### Type: `integer`


---

Markdown generated with [jsonschema-markdown](https://github.com/elisiariocouto/jsonschema-markdown).
