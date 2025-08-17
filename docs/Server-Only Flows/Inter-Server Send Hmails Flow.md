*Needs to be implemented to create a h-mail compatible server*

## Implementation
1. The sender server sends a [DeliverHmailRequest](../generated/routes/foreign/deliver_hmail/DeliverHmailRequest.md) to the recipient server (typically all those in the `recipients` and `ccs` fields in [SendHmailPackage](../generated/hmail/SendHmailPackage.md), as well as `bcc` in [SendEmailRequest](../generated/routes/native/send_email/SendEmailRequest.md)) with a proof-of-work attached. The [DeliverHmailRequest](../generated/routes/foreign/deliver_hmail/DeliverHmailRequest.md) also has a [AuthTokenDataField](../generated/fields/auth_token/AuthTokenDataField.md) attached for IP verification.
2. The recipient server checks if the request meets the minimum POW requirement, assuming the domain isn't spoofed at the minute. 
3. The recipient server sends a [VerifyIpRequest](../generated/routes/foreign/verify_ip/VerifyIpRequest.md) with the [AuthTokenField](../generated/fields/auth_token/AuthTokenField.md) to the IP it receives the [DeliverHmailRequest](../generated/routes/foreign/deliver_hmail/DeliverHmailRequest.md) from to ensure the IP isn't spoofed.
4. The sender server validates the [AuthTokenField](../generated/fields/auth_token/AuthTokenField.md) and replies with a [VerifyIpResponse](../generated/routes/foreign/verify_ip/VerifyIpResponse.md).
5. The recipient server makes an SPF DNS check (the same one used by normal e-mail) to verify that the sender IP is authorised to handle h-mails on behalf of the domain.
6. The recipient stores the email and responds with a [DeliverHmailResponse](DeliverHmailResponse).
7. The sender server typically collects the responses from recipients to return in [SendHmailResponse](../generated/routes/native/send_hmail/SendHmailResponse.md) back to a client.