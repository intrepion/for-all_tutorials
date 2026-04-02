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

- `beam`
- `dotnet`
- `go`
- `java`
- `javascript`
- `php`
- `python`
- `ruby`
- `rust`

### Languages By Ecosystem

#### `beam`

- `Elixir`
- `Erlang`

#### `dotnet`

- `C#`
- `F#`
- `Visual Basic`

#### `go`

- `Go`

#### `java`

- `Java`

#### `javascript`

- `JavaScript`
- `TypeScript`

#### `php`

- `PHP`

#### `python`

- `Python`

#### `ruby`

- `Ruby`

#### `rust`

- `Rust`

### Framework Options By Ecosystem And Surface Or Target

#### `beam`

- `command-line`: `builtin`
- `web/full-stack`: `phoenix-liveview`

#### `dotnet`

- `command-line`: `builtin`, `spectre-console`
- `web/full-stack`: `blazor-server`

#### `go`

- `command-line`: `builtin`
- `web/full-stack`: `echo-templates-htmx`

#### `java`

- `command-line`: `builtin`
- `web/full-stack`: `quarkus-native-qute`

#### `javascript`

- `command-line`: `builtin`, `commander`
- `web/full-stack`: `sveltekit-bun-adapter`

#### `php`

- `command-line`: `builtin`, `symfony-console`
- `web/full-stack`: `laravel-octane-livewire`

#### `python`

- `command-line`: `builtin`, `click`, `typer`
- `web/full-stack`: `fastapi-jinja2-htmx`

#### `ruby`

- `command-line`: `builtin`
- `web/full-stack`: `rails-hotwire`

#### `rust`

- `command-line`: `builtin`, `clap`, `ratatui`
- `web/full-stack`: `leptos-ssr`

### Testing Options By Ecosystem

#### `beam`

- no testing-specific guide has been written yet

#### `dotnet`

- `xunit`
- `nunit`
- `mstest`
- `tunit`

#### `go`

- no testing-specific guide has been written yet

#### `java`

- no testing-specific guide has been written yet

#### `javascript`

- no testing-specific guide has been written yet

#### `php`

- no testing-specific guide has been written yet

#### `python`

- no testing-specific guide has been written yet

#### `ruby`

- no testing-specific guide has been written yet

#### `rust`

- `libtest`

### Surface Options By Ecosystem

#### `beam`

- `command-line`
- `web`

#### `dotnet`

- `command-line`
- `web`

#### `go`

- `command-line`
- `web`

#### `java`

- `command-line`
- `web`

#### `javascript`

- `command-line`
- `web`

#### `php`

- `command-line`
- `web`

#### `python`

- `command-line`
- `web`

#### `ruby`

- `command-line`
- `web`

#### `rust`

- `command-line`
- `web`

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

## Stage 2

### printing-quotes

- `Project`: `printing-quotes`
- `Status`: `planned`
- `Prerequisites`: `count-characters`
- `Recommended Tutorial`: [projects/printing-quotes/tutorial/README.md](../projects/printing-quotes/tutorial/README.md)
- `Suggested Output Repos`: one core library repo for the chosen ecosystem, language, and test framework, then one adapter repo for the chosen surface implementation
- `Notes`: start with a small formatting contract such as `format_attributed_quote(author, quote) -> string`, build the core repo first, then add adapter repos that consume that core; this project builds on the earlier string exercises by combining two inputs into one formatted sentence and surrounding the quote text with literal double quotation marks

## Stage 3

### mad-lib

- `Project`: `mad-lib`
- `Status`: `planned`
- `Prerequisites`: `printing-quotes`
- `Recommended Tutorial`: [projects/mad-lib/tutorial/README.md](../projects/mad-lib/tutorial/README.md)
- `Suggested Output Repos`: one core library repo for the chosen ecosystem, language, and test framework, then one adapter repo for the chosen surface implementation
- `Notes`: start with a small formatting contract such as `format_mad_lib(noun, verb, adjective, adverb) -> string`, build the core repo first, then add adapter repos that consume that core; this project builds on the earlier formatting exercises by combining four named inputs into one fixed story template
