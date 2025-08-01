# VerifyIpRequest

POST: A `DeliverEmailRequest` will cause the target server to issue a `VerifyIpRequest` back
to the sender to ensure the IP is not being spoofed. The `ip_verification` token verifies that
the IP belongs to the sender.

### Type: `object`

| Property | Type | Required | Possible values | Deprecated | Default | Description | Examples |
| -------- | ---- | -------- | --------------- | ---------- | ------- | ----------- | -------- |
| ip_verification | `string` | ✅ | [AuthTokenField](#authtokenfield) |  |  |  |  |


---

# Definitions

## AuthTokenField

Represents a base-64 encoded authentication token - you will not need to decode this.
Used in bearer tokens and in some requests.

#### Type: `string`


---

Markdown generated with [jsonschema-markdown](https://github.com/elisiariocouto/jsonschema-markdown).
