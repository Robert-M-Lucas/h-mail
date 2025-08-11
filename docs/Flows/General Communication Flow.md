All communications are made through `https` with `GET` requests expecting query parameters and all others expecting JSON. Headers are used for [authentication](Authentication%20Flow.md).

Paths for requests fall into four categories:
- `/auth/-` for all authentication-related requests
- `/native/-` for all requests from users of a server e.g. [GetEmailsRequest](../generated/routes/native/get_emails/GetEmailsRequest.md)
- `/foreign/-` for all requests from other servers or users of other servers e.g. [GetUserPowPolicyRequest](../generated/routes/foreign/get_user_pow_policy/GetUserPowPolicyRequest.md)
- `/*other*` other requests e.g. [GetPowTokenRequest](../generated/routes/foreign/get_pow_token/GetPowTokenRequest.md)
