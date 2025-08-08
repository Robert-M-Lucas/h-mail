# CreateAccountPackage
> Defined in [create_account.rs](../../../../../interface/src/interface/routes/native/create_account.rs)

## Description
POST: Requests an account be created. Requires POW burden obtained through
`GetCreateAccountPowPolicyRequest`.

## Schema

| Property | Required | Type | Constraints |
| --- | --- | --- | --- |
| `password` | ✅ | `String` |     | 
| `username` | ✅ | `String` |     | 


