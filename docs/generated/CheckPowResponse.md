# CheckPowResponse

Returns whether the POW was solved correctly and, if not, why

### Type: `object(?)`

**Possible Values (string):** `Success`

_OR_ 

> ⚠️ Additional properties are not allowed.

| Property | Type | Required | Possible values | Deprecated | Default | Description | Examples |
| -------- | ---- | -------- | --------------- | ---------- | ------- | ----------- | -------- |
| Failure | Missing type | ✅ | [PowFailureReason](#powfailurereason) |  |  |  |  |


---

# Definitions

## PowFailureReason

Reason for a POW check failing

#### Type: `object(?)`

**Possible Values (string):** `FailedNoRetry` or `NotFoundCanRetry` or `BadRequestCanRetry`

_OR_ 

> ⚠️ Additional properties are not allowed.

| Property | Type | Required | Possible values | Deprecated | Default | Description | Examples |
| -------- | ---- | -------- | --------------- | ---------- | ------- | ----------- | -------- |
| DoesNotMeetPolicyMinimum | `integer` | ✅ | `0 <= x ` |  |  |  |  |


---

Markdown generated with [jsonschema-markdown](https://github.com/elisiariocouto/jsonschema-markdown).
