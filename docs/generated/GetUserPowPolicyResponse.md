# GetUserPowPolicyResponse

Returns the users POW policy, if they exist

### Type: `object`

| Property | Type | Required | Possible values | Deprecated | Default | Description | Examples |
| -------- | ---- | -------- | --------------- | ---------- | ------- | ----------- | -------- |
| data | `object` or `null` |  | [PowPolicy](#powpolicy) |  |  |  |  |


---

# Definitions

## PowPolicy

Represents a user's pow policy that dictates how an incoming email is categorised

#### Type: `object`

| Property | Type | Required | Possible values | Deprecated | Default | Description | Examples |
| -------- | ---- | -------- | --------------- | ---------- | ------- | ----------- | -------- |
| accepted | `integer` | ✅ | `0 <= x ` |  |  |  |  |
| minimum | `integer` | ✅ | `0 <= x ` |  |  |  |  |
| personal | `integer` | ✅ | `0 <= x ` |  |  |  |  |


---

Markdown generated with [jsonschema-markdown](https://github.com/elisiariocouto/jsonschema-markdown).
