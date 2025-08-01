# RefreshAccessResponse

JSON Schema missing a description, provide it using the `description` key in the root of the JSON document.

### Type: `object(?)`

**Possible Values (string):** `Failure` or `BadRequest`

_OR_ 

> ⚠️ Additional properties are not allowed.

| Property | Type | Required | Possible values | Deprecated | Default | Description | Examples |
| -------- | ---- | -------- | --------------- | ---------- | ------- | ----------- | -------- |
| Success | `object` | ✅ | [AuthTokenDataField](#authtokendatafield) |  |  |  |  |


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

Represents a base-64 encoded authentication token.
Used in bearer tokens and in some requests.

#### Type: `string`

## SystemTimeField

A timestamp represented as milliseconds since epoch

#### Type: `integer`


---

Markdown generated with [jsonschema-markdown](https://github.com/elisiariocouto/jsonschema-markdown).
