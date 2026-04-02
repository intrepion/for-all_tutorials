<!-- breadcrumbs:start -->
[for-all_tutorials](../README.md) / [Docs](README.md) / curriculum.md
<!-- breadcrumbs:end -->

# Curriculum Map

Mutable learning order for project tutorials, kept separate from the filesystem so projects can move without being renamed.

## How To Use This Map

This file answers:

- what should be learned first
- what each project depends on
- which ecosystems, languages, framework guides, testing options, surfaces, and targets are currently documented
- which tutorial path should usually be built first
- what state each project is in

This file should stay incremental:

- add projects when there is real intent to build them
- reorder stages when the curriculum becomes clearer
- avoid turning this into a giant speculative backlog

## Option Catalogs

Keep shared option lists here so project entries can stay small.

### Ecosystems

- `dotnet`
- `rust`

### Languages By Ecosystem

#### `dotnet`

- `C#`
- `F#`
- `Visual Basic`

#### `rust`

- `Rust`

### Framework Options By Ecosystem

#### `dotnet`

- no framework-specific guide has been written yet

#### `rust`

- no framework-specific guide has been written yet

### Testing Options By Ecosystem

#### `dotnet`

- `xunit`
- `nunit`
- `mstest`
- `tunit`

#### `rust`

- `builtin`

### Surface Options By Ecosystem

#### `dotnet`

- `command-line`
- `web`

#### `rust`

- `command-line`

### Target Options By Surface

- `command-line`: `all`
- `web`: `back-end`, `front-end`, `full-stack`

## Entry Shape

Each project entry should capture:

- `Project`
- `Status`
- `Prerequisites`
- `Recommended Tutorial`
- `Suggested Output Repos`
- `Notes`

Project slugs stay stable even if their place in this map changes.

## Stage 0

### saying-hello

- `Project`: `saying-hello`
- `Status`: `in-progress`
- `Prerequisites`: none
- `Recommended Tutorial`: [projects/saying-hello/tutorial/README.md](../projects/saying-hello/tutorial/README.md)
- `Suggested Output Repos`: one core library repo for the chosen ecosystem, language, and test framework, then one adapter repo for the chosen surface implementation
- `Notes`: start with a tiny core logic contract such as `greet(name) -> string`, follow the project tutorial starting with the core walkthrough, build the core repo first, then add adapter repos that consume that core; choose the exact surface and target later from the shared option catalogs and setup docs

## Stage 1

### count-characters

- `Project`: `count-characters`
- `Status`: `planned`
- `Prerequisites`: `saying-hello`
- `Recommended Tutorial`: [projects/count-characters/tutorial/README.md](../projects/count-characters/tutorial/README.md)
- `Suggested Output Repos`: one core library repo for the chosen ecosystem, language, and test framework, then one adapter repo for the chosen surface implementation
- `Notes`: start with small core logic contracts such as `count_characters(input) -> integer` and `format_character_count_message(input, count) -> string`, build the core repo first, then add adapter repos that consume that core; this project builds on `saying-hello` by introducing counting, exact input preservation, and zero/one/many message formatting
