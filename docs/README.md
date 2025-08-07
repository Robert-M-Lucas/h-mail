The generated documentation (in the `generated` folder) describes the JSON API.

There are two 'wrappers', [Authorized](generated/auth/Authorized.md) and [WithPow](generated/pow/WithPow.md). When a name is an alias for one of these (e.g. [Email](generated/email/Email.md) is [WithPow](generated/pow/WithPow.md)<[EmailPackage](generated/email/EmailPackage.md)>) the first line of the documentation will appear as *Name* (alias of *Alias-Of*<*Type-Parameter*>) e.g. [Email](generated/email/Email.md) ([WithPow](generated/pow/WithPow.md)<[EmailPackage](generated/email/EmailPackage.md)>).

Get started by reading [Flows](Flows.md)