# CheckAuthResponse (Authorized)

A wrapper around a response indicating whether a request that requires authorisation was
successful.

See `Success`'s value for the underlying type.

### Type: `object(?)`

**Possible Values (string):** `Unauthorized`

_OR_ 

> ⚠️ Additional properties are not allowed.

| Property | Type | Required | Possible values | Deprecated | Default | Description | Examples |
| -------- | ---- | -------- | --------------- | ---------- | ------- | ----------- | -------- |
| Success | `object` | ✅ | [CheckAuthResponseAuthed](#checkauthresponseauthed) |  |  |  |  |


---

# Definitions

## CheckAuthResponseAuthed

Returns the name of the user, should the user be authorised

#### Type: `object`

| Property | Type | Required | Possible values | Deprecated | Default | Description | Examples |
| -------- | ---- | -------- | --------------- | ---------- | ------- | ----------- | -------- |
| username | `string` | ✅ | string |  |  |  |  |


---

Markdown generated with [jsonschema-markdown](https://github.com/elisiariocouto/jsonschema-markdown).
