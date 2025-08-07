# H-Mail

A replacement for email using proof-or-work to protect against spam.

## Server
Run the server found in `server` to host a server. Its IP must be registered to your domain using `SPF`.

## Interface
Use this directly to create your own Rust client implementation.

The documentation for this interface (allowing you to use it from other languages) can be found in [docs](docs/README.md). A markdown reader such as Obsidian is recommended.

## Client
Use this to create your own Rust client UI using this reference implementation for communication with the server.

## Client UI
This is a UI for the client library allowing you to test/send emails through servers create using Tauri.

## Client TUI
Currently discontinued terminal UI for the client.
