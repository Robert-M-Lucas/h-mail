# AuthTokenDataField

An `AuthToken` with attached expiry time

### Type: `object`

| Property | Type | Required | Possible values | Deprecated | Default | Description | Examples |
| -------- | ---- | -------- | --------------- | ---------- | ------- | ----------- | -------- |
| expires_at | `integer` | ✅ | [SystemTimeField](#systemtimefield) |  |  |  |  |
| token | `string` | ✅ | [AuthTokenField](#authtokenfield) |  |  |  |  |


---

# Definitions

## AuthTokenField

Represents a base-64 encoded authentication token.
Used in bearer tokens and in some requests.

#### Type: `string`

## SystemTimeField

A timestamp represented as milliseconds since epoch

#### Type: `integer`


---

Markdown generated with [jsonschema-markdown](https://github.com/elisiariocouto/jsonschema-markdown).
