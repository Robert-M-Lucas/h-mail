# H-Mail - Server

A sample server implementation.

## Quickstart

> [!WARNING]
> This configuration disables important security measures

```
cargo run -r -- --no-salt --no-rate-limit --test-user
```

## Prerequisites

H-Mail requires that the server's IP is in an SPF DNS entry under the server's domain to prove that it is authorised to send h-mails on behalf of that domain.

The server also requires that `SECRET_SALT` is set to a base-64 encoded salt. If this is lost or changed after being used to hash a password, password verification will break.

## Flags

- `--test-user` - Creates a test user with username and password `test` on startup if one doesn't exist.
- `--generate-salt` - Generates a salt and exits immediately.

> [!WARNING]
> The following flags important security measures

- `--no-salt` - Removes the requirement of having a salt, using a zeroed one instead.
- `--no-spf` - Removes SPF checks that verify whether a server is authorised by a domain to send h-mails on behalf of it.
- `--no--rate-limit` - Disables rate limiting

## Config

Default Config:
```jsonc
{
  "domain": "example.com", // The domain this server is acting on behalf of
  "port": 8081, // The port to use (to be removed)
  "create_account_pow_burden": 390000, // The POW (proof-of-work) requirement for creating an account
  "pow_token_expiry_ms": 3600000, // The length of time for which a POW token is valid
  "pow_rsa_bits": 2048, // The number of bits used by RSA to generate the POW token
  "refresh_token_expiry_ms": 2592000000, // The expiry time of a refresh token (see Authentication Flow in documentation)
  "access_token_expiry_ms": 3600000, // The expiry time of an access token (see Authentication Flow in documentation)
  "verify_ip_token_expiry_ms": 60000, // The expiry time of an IP verification token (see Inter-Server Send Hmails Flow)
  "default_user_pow_policy": {
    "minimum": 6500,
    "accepted": 65000,
    "personal": 650000
  } // The default user POW policy (POW requirements for different categorisation, with below minimum not being delivered)
}
```