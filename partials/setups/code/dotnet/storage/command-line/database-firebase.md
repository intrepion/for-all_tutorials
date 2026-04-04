---
partial_kind: storage-partial
ecosystem: dotnet
surface: command-line
storage: database-firebase
---

# Database Firebase

Use `database-firebase` for `.NET` command-line adapters that persist data through Firebase Realtime Database over its REST API.

## Use This After

- [Shared Storage Taxonomy](../../../../storage/database-firebase.md)
- [All](../../adapters/command-line/all.md)

## Guidance

- keep the Firebase base URL, auth configuration, and REST request code in the adapter repo
- use direct HTTPS requests to Firebase Realtime Database endpoints ending in `.json`
- avoid premade Firebase client libraries for this path
- map JSON payloads to and from the core library contract at the adapter boundary
- keep canonical parsing, ordering, formatting, and domain rules in the core

## Ready State

The `.NET` `database-firebase` command-line storage path is ready when:

- the adapter can issue direct Firebase REST reads and writes to `.json` endpoints
- the adapter can serialize request payloads and parse response bodies without moving project rules into transport code
- Firebase-specific configuration and HTTP mechanics remain outside the core library
