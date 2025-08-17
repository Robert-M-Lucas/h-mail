# SendHmailRequest
> Defined in [send_hmail.rs](../../../../../interface/src/interface/routes/native/send_hmail.rs)

## Route
- Path: `/native/send_hmail`
- Method: `POST`
- Requires [authentication](../../../../Flows/Authentication%20Flow.md): ✅
- Response: [SendHmailResponse](SendHmailResponse.md)

## Description
Requests the server sends an h-mail to destinations specified in `hmail.to`,
`hmail.ccs` and `bccs`.
Requires all destinations to have a POW solved in `solved_pows`.

## Schema

| Property | Required | Type | Constraints |
| --- | :---: | --- | --- |
| `bccs` | ✅ | `Array` | With items of type [HmailAddress](../../../fields/hmail_address/HmailAddress.md) |
| `hmail` | ✅ | [SendHmailPackage](../../../hmail/SendHmailPackage.md) |  -  |
| `solved_pows` | ✅ | `Array` | With items of type [SolvedPowFor](../../../routes/native/send_hmail/SolvedPowFor.md) |


