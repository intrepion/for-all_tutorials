<!-- breadcrumbs:start -->
[for-all_tutorials](../README.md) / [Docs](README.md) / curriculum.md
<!-- breadcrumbs:end -->

# Curriculum Map

Mutable learning order for project tutorials, kept separate from the filesystem so projects can move without being renamed.

## How To Use This Map

This file answers:

- what should be learned first
- what each project depends on
- which surface paths are currently supported or encouraged
- which tutorial path should usually be built first
- what state each project is in

This file should stay incremental:

- add projects when there is real intent to build them
- reorder stages when the curriculum becomes clearer
- avoid turning this into a giant speculative backlog

## Entry Shape

Each project entry should capture:

- `Project`
- `Status`
- `Prerequisites`
- `Surface Options`
- `Recommended Tutorial`
- `Suggested Setup Paths`
- `Suggested Output Repos`
- `Notes`

Project slugs stay stable even if their place in this map changes.

## Stage 0

### saying-hello

- `Project`: `saying-hello`
- `Status`: `in-progress`
- `Prerequisites`: none
- `Surface Options`: `command-line/all`, `web/full-stack`
- `Recommended Tutorial`: [projects/saying-hello/tutorial/tdd.md](../projects/saying-hello/tutorial/tdd.md)
- `Suggested Setup Paths`: for `.NET`, start with [setups/code/dotnet/toolchain.md](../setups/code/dotnet/toolchain.md), choose one testing guide such as [setups/code/dotnet/testing/xunit.md](../setups/code/dotnet/testing/xunit.md), then choose one adapter guide such as [setups/code/dotnet/adapters/command-line/all.md](../setups/code/dotnet/adapters/command-line/all.md) or [setups/code/dotnet/adapters/web/full-stack.md](../setups/code/dotnet/adapters/web/full-stack.md)
- `Suggested Output Repos`: one core library repo for the chosen ecosystem, language, and test framework, then one adapter repo per chosen surface path
- `Notes`: start with a tiny core logic contract such as `greet(name) -> string`, follow the stack-agnostic TDD tutorial, build the core repo first, then add adapter repos that consume that core; a single run only needs one supported surface path, and more can be added later
