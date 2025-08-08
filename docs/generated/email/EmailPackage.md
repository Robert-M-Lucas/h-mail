# EmailPackage
> Defined in [EmailPackage.md.rs](../email/interface/src/interface/email)

## Description
Represents an email being sent

## Schema

| Property | Required | Type | Constraints |
| --- | --- | --- | --- |
| `contents` | ✅ | `String` |     | 
| `destination_user` | ✅ | `String` |     | 
| `random_id` | ✅ | `Integer` | `uint32` - Bounds: [0, -] | 
| `reply_to` |     | [BigUintField](../fields/big_uint/BigUintField.md) *OR* `null` |     | 


