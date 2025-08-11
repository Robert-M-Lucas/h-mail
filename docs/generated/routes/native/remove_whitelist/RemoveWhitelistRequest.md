# RemoveWhitelistRequest
> Defined in [remove_whitelist.rs](../../../../../interface/src/interface/routes/native/remove_whitelist.rs)

## Route
- Path: `/native/remove_whitelist`
- Method: `DELETE`
- Requires [authentication](../../../../Flows/Authentication%20Flow.md): ✅
- Response: [RemoveWhitelistResponse](RemoveWhitelistResponse.md)

## Description
Adds an address to the authenticated user's whitelist

## Schema

| Property | Required | Type | Constraints |
| --- | :---: | --- | --- |
| `address` | ✅ | `String` |  -  |
| `place_into` | ✅ | [PowClassification](../../../pow/PowClassification.md) |  -  |


