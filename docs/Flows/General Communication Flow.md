All communications are made through `https` with `GET/DELETE` requests expecting query parameters `POST` requests expecting JSON. All requests return JSON.

Headers are used for [authentication](Authentication%20Flow.md) between a client and its server.

Paths for requests fall into four categories:
- `/auth/-` for all authentication-related requests
- `/native/-` for all requests from users of a server e.g. [GetEmailsRequest](../generated/routes/native/get_emails/GetEmailsRequest.md)
- `/foreign/-` for all requests from other servers or users of other servers e.g. [GetUserPowPolicyRequest](../generated/routes/foreign/get_user_pow_policy/GetUserPowPolicyRequest.md). Only these need to be implemented to create a h-mail compatible server.
- `/*other*` other requests e.g. [GetPowTokenRequest](../generated/routes/foreign/get_pow_token/GetPowTokenRequest.md)
