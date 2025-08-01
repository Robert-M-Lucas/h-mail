# CreateAccountResponse

JSON Schema missing a description, provide it using the `description` key in the root of the JSON document.

### Type: `object(?)`

**Possible Values (string):** `Success` or `BadUsername` or `UsernameInUse` or `BadPassword`

_OR_ 

> ⚠️ Additional properties are not allowed.

| Property | Type | Required | Possible values | Deprecated | Default | Description | Examples |
| -------- | ---- | -------- | --------------- | ---------- | ------- | ----------- | -------- |
| DoesNotMeetPolicy | `integer` | ✅ | `0 <= x ` |  |  |  |  |

_OR_ 

> ⚠️ Additional properties are not allowed.

| Property | Type | Required | Possible values | Deprecated | Default | Description | Examples |
| -------- | ---- | -------- | --------------- | ---------- | ------- | ----------- | -------- |
| PowFailure | Missing type | ✅ | [PowFailureReason](#powfailurereason) |  |  |  |  |


---

# Definitions

## PowFailureReason

No description provided for this model.

#### Type: `object(?)`

**Possible Values (string):** `FailedNoRetry` or `NotFoundCanRetry` or `BadRequestCanRetry` or `BadIPCanRetry`

_OR_ 

> ⚠️ Additional properties are not allowed.

| Property | Type | Required | Possible values | Deprecated | Default | Description | Examples |
| -------- | ---- | -------- | --------------- | ---------- | ------- | ----------- | -------- |
| DoesNotMeetPolicyMinimum | `integer` | ✅ | `0 <= x ` |  |  |  |  |


---

Markdown generated with [jsonschema-markdown](https://github.com/elisiariocouto/jsonschema-markdown).
