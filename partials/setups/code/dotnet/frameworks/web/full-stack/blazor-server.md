---
partial_kind: framework-partial
ecosystem: dotnet
surface: web
target: full-stack
framework: blazor-server
---

# Blazor Server

Reusable `.NET` `web/full-stack` framework guidance for adapters that use Blazor Server for server-rendered, interactive web user interfaces.

## Use This After

- [Toolchain](../../../toolchain/README.md)
- one guide from [Testing](../../../testing/README.md)
- [Full Stack](../../../adapters/web/full-stack.md)

## Scope

This guide covers:

- using Blazor Server as the full-stack web host for an adapter repo
- keeping framework-specific component, circuit, and rendering concerns inside the adapter
- mapping user input and rendered output cleanly to the core library contract

This guide does not cover:

- project-specific business rules
- project-specific pages, persistence, auth, or deployment
- moving validation or service logic out of the core library

## Adapter Rule

A Blazor Server adapter should stay thin:

- translate UI events and form input into calls to the core library
- translate core results into rendered component state
- keep validation and service decisions in the core project logic

## Ready State

The Blazor Server setup is ready when:

- the adapter repo already follows the generic `web/full-stack` setup
- Blazor-specific component and host code delegates to the core library
- project rules remain in the separately tested core library repo
