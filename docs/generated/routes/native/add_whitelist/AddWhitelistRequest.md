# AddWhitelistRequest
> Defined in [add_whitelist.rs](../../../../../interface/src/interface/routes/native/add_whitelist.rs)

## Route
- Path: `/native/add_whitelist`
- Method: `POST`
- Requires [authentication](../../../../Flows/Authentication%20Flow.md): ✅
- Response: [AddWhitelistResponse](AddWhitelistResponse.md)

## Description
Adds an address to the authenticated user's whitelist

## Schema

| Property | Required | Type | Constraints |
| --- | :---: | --- | --- |
| `address` | ✅ | [HmailAddress](../../../fields/hmail_address/HmailAddress.md) |  -  |
| `place_into` | ✅ | [PowClassification](../../../pow/PowClassification.md) |  -  |


