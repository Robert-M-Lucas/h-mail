# CheckPowPackage

POST: Utility function to check POW. Note that checking POW will invalidate the POW token,
preventing it from being used for other purposes. The hash of `challenge` will be used as the
hash for POW.

## Schema

| Property | Required | Type | Constraints |
| --- | --- | --- | --- |
| `challenge` | ✅ | `String` |     | 


