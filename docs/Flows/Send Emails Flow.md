*Only the [Inter-Server Send Emails Flow](../Server-Only%20Flows/Inter-Server%20Send%20Emails%20Flow.md) needs to be implemented to create a h-mail compatible server*
## General Idea
Emails are sent with very similar fields to normal emails - see [SendEmailPackage](../generated/email/SendEmailPackage.md).

1. Obtain the proof-of-work burden (or whether you are whitelisted) for all recipients
2. Solve POW for all recipients
3. Send email to your server (requires [authentication](Authentication%20Flow.md))
4. Your server sends email to all recipients ([Inter-Server Send Emails Flow](../Server-Only%20Flows/Inter-Server%20Send%20Emails%20Flow.md))

## Typical Implementation
1. Get the proof-of-work burden for sending an email either with a [GetUserPowPolicyRequest](../generated/routes/foreign/get_user_pow_policy/GetUserPowPolicyRequest.md) to the recipient's server, or a [IsWhitelistedRequest](../generated/routes/native/is_whitelisted/IsWhitelistedRequest.md) to your server (as the recipients server will only reveal whitelist information to an authorised source - your email server). Note that the [IsWhitelistedRequest](../generated/routes/native/is_whitelisted/IsWhitelistedRequest.md) also returns the user's POW policy, if you are not whitelisted.
2. Create the [SendEmailPackage](../generated/email/SendEmailPackage.md) and solve the POW requirement for all recipients for it.
3. Make a [SendEmailRequest](../generated/routes/native/send_email/SendEmailRequest.md) which requires a solved POW token for each recipient
4. The server then sends the email on your behalf to each recipient in the `to` and `cc` fields in the [SendEmailRequest](../generated/routes/native/send_email/SendEmailRequest.md), as well as to any recipients in the [SendEmailPackage](../generated/email/SendEmailPackage.md)'s `bcc` field

Replies work by specifying the `parent` field in [SendEmailPackage](../generated/email/SendEmailPackage.md) to be the hash of the email you are replying to (same as the one used for POW).

