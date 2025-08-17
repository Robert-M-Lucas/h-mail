*Does not need to be implemented to create a h-mail compatible server*

The authentication flow is largely based on (although is not) OAuth.

## General Idea
The general idea is as follows:
1. Trade username and password for a *refresh token*. This is stored securely on disk (out of memory while the program is running, being only read as needed) and has a long expiry.
2. Trade refresh tokens for short-lived *access tokens*. These are what is actually needed to authorise requests and aren't stored to disk.
3. Any routes requiring authentication will return the wrapper type [Authorized](../generated/auth/Authorized.md)<*Actual Data*> (e.g. [SendHmailResponse](../generated/routes/native/send_hmail/SendHmailResponse.md)). These expect the header `Authorization: Bearer [access token]`.

> [!WARNING]
> Do not store the username or password on disk


> [!WARNING]
> Do not store the username or password in memory longer than absolutely necessary


> [!WARNING]
> Store the refresh token securely on disk (ideally an OS-provided key storage)


> [!WARNING]
> Do not store the refresh token in memory longer than absolutely necessary

## Typical Implementation
### First Run Authentication
1. Ask the user for their username and password. Send an [AuthenticateRequest](../generated/routes/auth/authenticate/AuthenticateRequest.md) with `https` to `/auth/authenticate` and retrieve the refresh token from the [AuthenticateResponse](../generated/routes/auth/authenticate/AuthenticateResponse.md)
2. Use the refresh token to get an access token by sending a [RefreshAccessRequest](../generated/routes/auth/refresh_access/RefreshAccessRequest.md) to `/auth/refresh_access` and retrieve the access token from the [RefreshAccessResponse](../generated/routes/auth/refresh_access/RefreshAccessResponse.md)
3. Store the refresh token to disk securely (ideally an OS-provided key storage solution) and drop it from memory

### Later Runs Authentication
1. Read the refresh token from disk and get an access token by sending a [RefreshAccessRequest](../generated/routes/auth/refresh_access/RefreshAccessRequest.md) to `/auth/refresh_access` and retrieve the access token from the [RefreshAccessResponse](../generated/routes/auth/refresh_access/RefreshAccessResponse.md)
	- (Should this fail, perform the same steps as in the first run)
2. Drop the refresh token from memory

### Authenticating Other Requests
Any routes requiring authentication will return the wrapper type [Authorized](../generated/auth/Authorized.md)<*Actual Data*> (e.g. [SendHmailResponse](../generated/routes/native/send_hmail/SendHmailResponse.md)). These expect the header `Authorization: Bearer [access token]`.

Typically, refreshing the access token is done behind the scenes so if authorization fails on a request, the client refreshes the access token and retries the same request. A refresh token should be checked at the start of a user's session (by getting an initial access token) and should outlast any reasonable session length, unless it is revoked by the server e.g. if the user changes their password.
