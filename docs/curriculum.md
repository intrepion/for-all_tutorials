<!-- breadcrumbs:start -->
[for-all_tutorials](../README.md) / [Docs](README.md) / curriculum.md
<!-- breadcrumbs:end -->

# Curriculum Map

Mutable learning order for project tutorials, kept separate from the filesystem so projects can move without being renamed.

## How To Use This Map

This file answers:

- what should be learned first
- what each project depends on
- which surfaces are required
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
- `Required Surfaces`
- `Recommended Tutorial`
- `Suggested Setup Path`
- `Notes`

Project slugs stay stable even if their place in this map changes.

## Stage 0

### saying-hello

- `Project`: `saying-hello`
- `Status`: `in-progress`
- `Prerequisites`: none
- `Required Surfaces`: `command-line/all`
- `Recommended Tutorial`: [tutorials/saying-hello/README.md](../tutorials/saying-hello/README.md)
- `Suggested Setup Path`: start with [setups/csharp/sdk/README.md](../setups/csharp/sdk/README.md) and one C# testing setup such as [setups/csharp/testing/xunit/README.md](../setups/csharp/testing/xunit/README.md)
- `Notes`: start with a tiny core logic contract such as `greet(name) -> string`, follow the stack-agnostic TDD tutorial, then realize it through one chosen setup path and wrap it in a thin adapter; expansion targets may later include `web/back-end`, `web/front-end`, `web/full-stack`, `desktop/all`, `desktop/macos`, `desktop/linux`, `desktop/windows`, `mobile/all`, `mobile/ios`, and `mobile/android`
