# H-Mail - Server

A sample server implementation.

## Prerequisites

- The server's IP is in an SPF DNS entry under the server's domain to prove that it is authorised to send h-mails on behalf of that domain.
- `SECRET_SALT` is set to a base-64 encoded salt. If this is lost or changed after being used to hash a password, password verification will break.
- `DATABASE_URL` is set to the URL of a Postgres database e.g. `postgres://test@localhost:5432/db`
  - This database must be set up using the [diesel CLI](https://diesel.rs/guides/getting-started#installing-diesel-cli) by running `diesel database setup --database-url [DATABASE_URL]`. This command requires the `migration` folder.

## Running

```bash
cargo run -r
```
With arguments:
```bash
cargo run -r -- [ARGS]
```
Or build with `cargo build -r` and run the executable:
```bash
server-binary [ARGS]
```

## Flags

- `--test-user` - Creates a test user with username and password `test` on startup if one doesn't exist.
- `--generate-salt` - Generates a salt and exits immediately.
- `--simulate-latency [latency in ms]` - Makes the server take at least the specified time to respond to requests.

> [!WARNING]
> The following flags disable important security measures

- `--no-salt` - Removes the requirement of having a salt, using a zeroed one instead.
- `--no-spf` - Removes SPF checks that verify whether a server is authorised by a domain to send h-mails on behalf of it.
- `--no--rate-limit` - Disables rate limiting

## Config

The config can be found in `config.json`, a default version of which is generated on first run.

Default Config:
```jsonc
{
  // The domain this server is acting on behalf of
  "domain": "example.com",
  // The port to use (to be removed)
  "port": 8081,
  // The POW (proof-of-work) requirement for creating an account
  "create_account_pow_burden": 390000,
  // The length of time for which a POW token is valid
  "pow_token_expiry_ms": 3600000,
  // The number of bits used by RSA to generate the POW token
  "pow_rsa_bits": 2048,
  // The expiry time of a refresh token (see Authentication Flow in documentation)
  "refresh_token_expiry_ms": 2592000000,
  // The expiry time of an access token (see Authentication Flow in documentation)
  "access_token_expiry_ms": 3600000,
  // The expiry time of an IP verification token (see Inter-Server Send Hmails Flow)
  "verify_ip_token_expiry_ms": 60000,
  // The default user POW policy (POW requirements for different categorisation, with below minimum not being delivered)
  "default_user_pow_policy": {
    "minimum": 6500,
    "accepted": 65000,
    "personal": 650000
  },
  // Regex for user passwords
  "password_regex": "^.{8,}$",
  // Reason given for password rejection
  "password_requirement_text": "Password must have at least 8 characters.",
  // How many requests a client can send before being rate limited
  "rate_limit_burst_size": 100,
  // Time between users request allowance being restored by 1
  "rate_limit_refresh_ms": 100 
}
```