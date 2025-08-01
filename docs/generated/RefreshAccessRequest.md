# RefreshAccessRequest

POST: Requests a new access token authorised by a refresh token

### Type: `object`

| Property | Type | Required | Possible values | Deprecated | Default | Description | Examples |
| -------- | ---- | -------- | --------------- | ---------- | ------- | ----------- | -------- |
| refresh_token | `string` | ✅ | [AuthTokenField](#authtokenfield) |  |  |  |  |


---

# Definitions

## AuthTokenField

Represents a base-64 encoded authentication token - you will not need to decode this.
Used in bearer tokens and in some requests.

#### Type: `string`


---

Markdown generated with [jsonschema-markdown](https://github.com/elisiariocouto/jsonschema-markdown).
