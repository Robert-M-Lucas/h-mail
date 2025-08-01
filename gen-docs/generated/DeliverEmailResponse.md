# DeliverEmailResponse

JSON Schema missing a description, provide it using the `description` key in the root of the JSON document.

### Type: `object(?)`

**Possible Values (string):** `Success` or `UserNotFound` or `BadRequest` or `SenderIpNotAuthed`

_OR_ 

> ⚠️ Additional properties are not allowed.

| Property | Type | Required | Possible values | Deprecated | Default | Description | Examples |
| -------- | ---- | -------- | --------------- | ---------- | ------- | ----------- | -------- |
| DoesNotMeetPolicy | `object` | ✅ | [PowPolicy](#powpolicy) |  |  |  |  |

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

## PowPolicy

No description provided for this model.

#### Type: `object`

| Property | Type | Required | Possible values | Deprecated | Default | Description | Examples |
| -------- | ---- | -------- | --------------- | ---------- | ------- | ----------- | -------- |
| accepted | `integer` | ✅ | `0 <= x ` |  |  | Description B |  |
| minimum | `integer` | ✅ | `0 <= x ` |  |  | Description A |  |
| personal | `integer` | ✅ | `0 <= x ` |  |  | Description C |  |


---

Markdown generated with [jsonschema-markdown](https://github.com/elisiariocouto/jsonschema-markdown).
