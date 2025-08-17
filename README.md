# H-Mail

A replacement for email using proof-or-work to protect against spam.

## Docs
[docs](docs/README.md)

## Server
Run the server found in `server` to host a server. Its IP must be registered to your domain using `SPF`.

## Interface
Use this directly to create your own Rust client implementation. 

Use the feature flag `client_interface` to enable parts of the interface used by the reference implementation of a client and server. Use the feature flag `client_implementation` to enable further utilities used by the reference client and server.

The documentation for this interface (allowing you to use it from other languages) can be found in [docs](docs/README.md).

## Client
Use this to create your own Rust client UI using this reference implementation for communication with the server.

## Client UI
This is a UI for the client library allowing you to test/send emails through servers create using Tauri.

## Client TUI
Currently discontinued terminal UI for the client.

## Gen Docs
Used to generate som of the `docs` pages. It is an internal tool used for one-off documentation generation, with features added as needed.
