<!-- breadcrumbs:start -->
[for-all_tutorials](../../../../../README.md) / [Setups](../../../../README.md) / [Code](../../../README.md) / [dotnet](../../README.md) / [Frameworks](../README.md) / [Command Line](README.md) / spectre-console.md
<!-- breadcrumbs:end -->

# Spectre Console

Reusable `.NET` command-line framework guidance for adapters that use Spectre Console for prompts, command wiring, or richer terminal output.

## Use This After

- [Toolchain](../../toolchain.md)
- one guide from [Testing](../../testing/README.md)
- [All](../../adapters/command-line/all.md)

## Scope

This guide covers:

- adding Spectre Console to a command-line adapter repo
- keeping Spectre-specific prompting and presentation inside the adapter
- mapping framework input and output cleanly to the core library contract

This guide does not cover:

- project-specific business rules
- project-specific prompt wording
- moving validation or service logic out of the core library

## Suggested Setup

Add Spectre Console to the adapter project:

```bash
dotnet add <adapter-project-path> package Spectre.Console
```

If the adapter grows into explicit commands, prompts, or styled console output, keep those concerns in the adapter project and continue treating the core library as the owner of the project rules.

## Adapter Rule

A Spectre-based adapter should stay thin:

- collect input with Spectre prompts or commands
- call the core library with normalized adapter input only when the project spec allows it
- render the returned result with Spectre formatting

Do not move project rules into Spectre validators, prompt handlers, or command classes if those rules belong in the core project logic.

## Ready State

The Spectre Console setup is ready when:

- the adapter repo already follows the generic `command-line/all` setup
- Spectre Console is added to the adapter project
- Spectre-specific prompt and presentation code delegates to the core library
