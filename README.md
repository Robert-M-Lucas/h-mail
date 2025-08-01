# H-Mail

A replacement for email using POW to protect against spam.

## Server
Run the server found in `server` to host a server. Its IP must be registered to your domain using `SPF`.

## Interface
Use this to create your own Rust client implementation. [Serde's](https://crates.io/crates/serde) serialisation and deserialisation of the types found in `interface/src/interface` are the current reference for the server API.

The documentation for this interface can be found in `docs`. A markdown reader such as Obsidian is recommended.

## Client
Use this to create your own Rust client UI using this reference implementation for communication with the server.

## Client UI
This is a UI for the client library allowing you to test/send emails through servers.
