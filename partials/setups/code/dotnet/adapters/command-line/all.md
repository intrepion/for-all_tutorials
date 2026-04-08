---
partial_kind: adapter-partial
ecosystem: dotnet
surface: command-line
target: all
---

# All

Reusable `.NET` adapter setup for the `command-line/all` target.

## Goal

Create a thin console adapter that depends on the shared contracts library without moving project rules into the entry point.

## Use This After

- [Toolchain](../../../toolchain/README.md)
- one guide from [Testing](../../../testing/README.md)
- an existing contracts library repo for the project shape

## Scope

This guide covers:

- keeping the adapter thin
- pairing the adapter with a stack-specific storage guide when the project persists local data

This guide does not cover:

- project-specific command parsing
- local-file storage mechanics beyond choosing the appropriate storage guide
- optional command-line framework choices beyond the pilot slice

## Contracts Dependency Rule

Keep the contracts library repo separate from the adapter repo.

For local development, depend on a sibling checkout of the contracts repo with a real project reference.

If you also want a separately tested runtime implementation, keep that in a matching core repo. Let the project-specific adapter tutorial decide when and how to wire that implementation.

If the chosen project persists local data, pair this guide with one storage guide from [Storage](../../storage/command-line/README.md).

## Adapter Rule

The `Program.cs` entry point should stay thin:

- read input from the command line
- call a service that satisfies the contract
- write the output

Do not duplicate validation or service rules in the adapter if those rules belong in the core project logic.

## Ready State

The `.NET` `command-line/all` adapter setup is ready when:

- the console project exists
- the console project is added to the solution
- the console project depends on the contracts library
- the console entry point can delegate through the contract
- any optional command-line framework still delegates through the contract
