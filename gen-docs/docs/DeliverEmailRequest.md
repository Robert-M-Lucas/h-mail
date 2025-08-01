# DeliverEmailRequest

JSON Schema missing a description, provide it using the `description` key in the root of the JSON document.

### Type: `object`

| Property | Type | Required | Possible values | Deprecated | Default | Description | Examples |
| -------- | ---- | -------- | --------------- | ---------- | ------- | ----------- | -------- |
| email | `object` | ✅ | [WithPow](#withpow) |  |  |  |  |
| source_domain | `string` | ✅ | string |  |  |  |  |
| source_user | `string` | ✅ | string |  |  |  |  |
| verify_ip | `object` | ✅ | [AuthTokenDataField](#authtokendatafield) |  |  |  |  |
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

Represents a base-64 encoded authentication token.
Used in bearer tokens and in some requests.

#### Type: `string`

## BigUintField

A base-64 little-endian encoding of a large unsigned integer

#### Type: `string`

## EmailPackage

No description provided for this model.

#### Type: `object`

| Property | Type | Required | Possible values | Deprecated | Default | Description | Examples |
| -------- | ---- | -------- | --------------- | ---------- | ------- | ----------- | -------- |
| contents | `string` | ✅ | string |  |  |  |  |
| destination_user | `string` | ✅ | string |  |  |  |  |

## SystemTimeField

A timestamp represented as milliseconds since epoch

#### Type: `integer`

## WithPow

No description provided for this model.

#### Type: `object`

| Property | Type | Required | Possible values | Deprecated | Default | Description | Examples |
| -------- | ---- | -------- | --------------- | ---------- | ------- | ----------- | -------- |
| inner | `object` | ✅ | [EmailPackage](#emailpackage) |  |  |  |  |
| iters | `integer` | ✅ | `0 <= x ` |  |  |  |  |
| pow_result | `string` | ✅ | [BigUintField](#biguintfield) |  |  |  |  |
| token | `string` | ✅ | [BigUintField](#biguintfield) |  |  |  |  |


---

Markdown generated with [jsonschema-markdown](https://github.com/elisiariocouto/jsonschema-markdown).
