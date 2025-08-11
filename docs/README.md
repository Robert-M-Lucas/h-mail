The generated documentation (in the `generated` folder) describes the JSON API.

Note that though this documentation covers all requests supported by the example server implementation, only the `foreign` routes (in `generated/routes/foreign`, as described in [General Communication Flow](Flows/General%20Communication%20Flow.md)) need to be implemented for a server to be compatible with other h-mail servers. With all routes correctly implemented, a server/client will be compatible with the reference client/server.

There are two 'wrappers', [Authorized](generated/auth/Authorized.md) and [WithPow](generated/pow/WithPow.md). When a name is an alias for one of these (e.g. [Email](generated/email/Email.md) is [WithPow](generated/pow/WithPow.md)<[EmailPackage](generated/email/EmailPackage.md)>) the first line of the documentation will appear as *Name* (alias of *Alias-Of*<*Type-Parameter*>) e.g. [Email](generated/email/Email.md) ([WithPow](generated/pow/WithPow.md)<[EmailPackage](generated/email/EmailPackage.md)>).

Get started by reading [Flows](Flows.md)