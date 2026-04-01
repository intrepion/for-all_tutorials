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
- `Recommended First Library Tutorial`
- `Recommended First Surface Tutorial`
- `Notes`

Project slugs stay stable even if their place in this map changes.

## Stage 0

### saying-hello

- `Project`: `saying-hello`
- `Status`: `in-progress`
- `Prerequisites`: none
- `Required Surfaces`: `command-line/all`
- `Recommended First Library Tutorial`: [tutorials/saying-hello/libraries/csharp/README.md](../tutorials/saying-hello/libraries/csharp/README.md)
- `Recommended First Surface Tutorial`: `command-line/all` in C# after the library tutorial is complete
- `Notes`: start with a tiny core logic contract such as `greet(name) -> string`, test it thoroughly, then wrap it in a thin adapter; expansion targets may later include `web/back-end`, `web/front-end`, `web/full-stack`, `desktop/all`, `desktop/macos`, `desktop/linux`, `desktop/windows`, `mobile/all`, `mobile/ios`, and `mobile/android`
