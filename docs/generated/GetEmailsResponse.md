# GetEmailsResponse (Authorized)

A wrapper around a response indicating whether a request that requires authorisation was
successful.

See `Success`'s value for the underlying type.

### Type: `object(?)`

**Possible Values (string):** `Unauthorized`

_OR_ 

> ⚠️ Additional properties are not allowed.

| Property | Type | Required | Possible values | Deprecated | Default | Description | Examples |
| -------- | ---- | -------- | --------------- | ---------- | ------- | ----------- | -------- |
| Success | `array` | ✅ | [GetEmailsResponseAuthed](#getemailsresponseauthed) |  |  |  |  |


---

# Definitions

## GetEmailsEmail

An individual email in a user's inbox

#### Type: `object`

| Property | Type | Required | Possible values | Deprecated | Default | Description | Examples |
| -------- | ---- | -------- | --------------- | ---------- | ------- | ----------- | -------- |
| email | `string` | ✅ | string |  |  |  |  |
| pow_classification | `string` | ✅ | [PowClassification](#powclassification) |  |  |  |  |
| source | `string` | ✅ | string |  |  |  |  |

## GetEmailsResponseAuthed

Returns the emails in a user's inbox

#### Type: `array`

## PowClassification

Represents a classification in the `PowPolicy`

#### Type: `string`

**Possible Values (string):** `Minimum` or `Accepted` or `Personal`


---

Markdown generated with [jsonschema-markdown](https://github.com/elisiariocouto/jsonschema-markdown).
