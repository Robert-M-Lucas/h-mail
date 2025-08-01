# DeliverEmailResponse

Returns whether the email delivery succeeded and, if not, why

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

Reason for a POW check failing

#### Type: `object(?)`

**Possible Values (string):** `FailedNoRetry` or `NotFoundCanRetry` or `BadRequestCanRetry`

_OR_ 

> ⚠️ Additional properties are not allowed.

| Property | Type | Required | Possible values | Deprecated | Default | Description | Examples |
| -------- | ---- | -------- | --------------- | ---------- | ------- | ----------- | -------- |
| DoesNotMeetPolicyMinimum | `integer` | ✅ | `0 <= x ` |  |  |  |  |

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
