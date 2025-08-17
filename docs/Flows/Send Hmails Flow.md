*Only the [Inter-Server Send Hmails Flow](../Server-Only%20Flows/Inter-Server%20Send%20Hmails%20Flow.md) needs to be implemented to create a h-mail compatible server*
## General Idea
Emails are sent with very similar fields to normal emails - see [SendHmailPackage](../generated/hmail/SendHmailPackage.md).

1. Obtain the proof-of-work burden (or whether you are whitelisted) for all recipients
2. Solve POW for all recipients
3. Send email to your server (requires [authentication](Authentication%20Flow.md))
4. Your server sends h-mail to all recipients ([Inter-Server Send Hmails Flow](../Server-Only%20Flows/Inter-Server%20Send%20Hmails%20Flow.md))

## Typical Implementation
1. Get the proof-of-work burden for sending an email either with a [GetAnonymousUserPowPolicyRequest](../generated/routes/foreign/get_anonymous_user_pow_policy/GetAnonymousUserPowPolicyRequest.md) to the recipient's server, or a [GetUserPowPolicyRequest](../generated/routes/native/get_user_pow_policy/GetUserPowPolicyRequest.md) to your server (as the recipients server will only reveal whitelist information to an authorised source - your email server).
2. Create the [SendHmailPackage](../generated/hmail/SendHmailPackage.md) and solve the POW requirement for all recipients for it.
3. Make a [SendHmailRequest](../generated/routes/native/send_hmail/SendHmailRequest.md) which requires a solved POW token for each recipient
4. The server then sends the email on your behalf to each recipient in the `to` and `cc` fields in the [SendHmailPackage](../generated/hmail/SendHmailPackage.md), as well as to any recipients in the [SendHmailRequest](../generated/routes/native/send_hmail/SendHmailRequest.md)'s `bcc` field

Replies work by specifying the `parent` field in [SendHmailPackage](../generated/hmail/SendHmailPackage.md) to be the hash of the email you are replying to (same as the one used for POW).

