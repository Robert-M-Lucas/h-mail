# AuthenticateResponse

Returns a refresh token if successful

### Type: `object(?)`

**Possible Values (string):** `Failure`

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

Represents a base-64 encoded authentication token - you will not need to decode this.
Used in bearer tokens and in some requests.

#### Type: `string`

## SystemTimeField

A timestamp represented as milliseconds since epoch

#### Type: `integer`


---

Markdown generated with [jsonschema-markdown](https://github.com/elisiariocouto/jsonschema-markdown).
