# CreateAccountResponse

Returns whether the account creation succeeded and, if not, why

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
