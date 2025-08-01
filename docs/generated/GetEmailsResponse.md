# GetEmailsResponse (Authorized)

JSON Schema missing a description, provide it using the `description` key in the root of the JSON document.

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

No description provided for this model.

#### Type: `object`

| Property | Type | Required | Possible values | Deprecated | Default | Description | Examples |
| -------- | ---- | -------- | --------------- | ---------- | ------- | ----------- | -------- |
| email | `string` | ✅ | string |  |  |  |  |
| pow_classification | `string` | ✅ | [PowClassification](#powclassification) |  |  |  |  |
| source | `string` | ✅ | string |  |  |  |  |

## GetEmailsResponseAuthed

No description provided for this model.

#### Type: `array`

## PowClassification

No description provided for this model.

#### Type: `string`

**Possible Values (string):** `Minimum` or `Accepted` or `Personal`


---

Markdown generated with [jsonschema-markdown](https://github.com/elisiariocouto/jsonschema-markdown).
