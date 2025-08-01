# CreateAccountPackage

POST: Requests an account be created. Requires POW burden obtained through
`GetCreateAccountPowPolicyRequest`. The hash of `username` will be used for the POW hash.

### Type: `object`

| Property | Type | Required | Possible values | Deprecated | Default | Description | Examples |
| -------- | ---- | -------- | --------------- | ---------- | ------- | ----------- | -------- |
| password | `string` | ✅ | string |  |  |  |  |
| username | `string` | ✅ | string |  |  |  |  |


---

Markdown generated with [jsonschema-markdown](https://github.com/elisiariocouto/jsonschema-markdown).
