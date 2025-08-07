# SendEmailRequest

## Description
POST: Requests the server sends an email to another server

AUTH: Requires an access token as the bearer token

## Schema

| Property | Required | Type | Constraints |
| --- | --- | --- | --- |
| `destination_domain` | ✅ | `String` |     | 
| `email` | ✅ | [Email](../../../email/Email.md) ([WithPow](../../../pow/WithPow.md)\<[EmailPackage](../../../email/EmailPackage.md)\>) |     | 


