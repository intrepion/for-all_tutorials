<!-- breadcrumbs:start -->
[for-all_tutorials](../README.md) / [Docs](README.md) / architecture.md
<!-- breadcrumbs:end -->

# Repository Architecture

Practical guidance for organizing this repo as a spec-driven, test-driven tutorial curriculum with stable project paths and a separate curriculum map.

## Recommendation

Use a project-first structure with curriculum ordering kept outside the filesystem:

- keep stable project slugs in `projects/`
- keep reusable setup guidance in `setups/`
- keep curriculum order in [docs/curriculum.md](curriculum.md)
- keep each project's overview, spec, and tutorial materials together
- require tests and coverage from the beginning instead of treating them as cleanup

This separates identity from ordering:

- the filesystem answers "what is this project?"
- the curriculum map answers "when should this be learned?"

## Core Principles

- project slugs are stable
- curriculum order is mutable
- every tutorial is still a project app
- each project keeps its own spec and tutorial side by side
- setups capture reusable environment and tooling guidance
- this repo stores tutorials, not the resulting implementation repos
- tests are the executable form of the spec
- code coverage and branch coverage are part of done
- project tutorials should keep adapters thin by building on a well-tested core logic unit first

## Recommended Tree

```text
for-all_tutorials/
  docs/
    architecture.md
    curriculum.md

  setups/
    README.md
    code/
      README.md
      beam/
        README.md
        languages/
          README.md
          elixir.md
          erlang.md
        toolchain/
          README.md
          otp.md
          mix.md
          rebar3.md
        ...
      dotnet/
        README.md
        languages/
          README.md
          csharp.md
          fsharp.md
          visual-basic.md
        toolchain/
          README.md
          sdk.md
          dotnet-cli.md
        adapters/
          README.md
          command-line/
            README.md
            all.md
          web/
            README.md
            full-stack.md
        testing/
          README.md
          xunit.md
          nunit.md
          mstest.md
          tunit.md
        frameworks/
          README.md
          command-line/
            README.md
            no-framework.md
            spectre-console.md
          web/
            README.md
            full-stack/
              README.md
              blazor-server.md
      go/
        ...
      java/
        ...
      javascript/
        ...
      php/
        ...
      python/
        ...
      ruby/
        ...
      rust/
        README.md
        languages/
          README.md
          rust.md
        toolchain/
          README.md
          rustup.md
          cargo.md
        adapters/
          README.md
          command-line/
            README.md
            all.md
          web/
            README.md
            full-stack.md
        testing/
          README.md
          libtest.md
        frameworks/
          README.md
          command-line/
            README.md
            no-framework.md
            clap.md
            ratatui.md
          web/
            README.md
            full-stack/
              README.md
              leptos-ssr.md
    storage/
      README.md

  projects/
    README.md
    saying-hello/
      README.md
      spec/
        README.md
      tutorial/
        README.md
        core.md
        adapter.md
```

## Projects

Each project should keep its own materials together:

```text
projects/<project>/
  README.md
  spec/
    README.md
  tutorial/
    README.md
    core.md
    adapter.md
```

This lets one folder answer three questions cleanly:

- what is this project?
- what is the canonical contract?
- how should it be built test-first?

### `projects/<project>/spec/`

The spec is the canonical contract for every implementation of that project.

At minimum, each spec should define:

- the project goal
- non-goals
- the core logic contract
- examples and edge cases
- user-visible behavior
- testing expectations
- coverage expectations
- benchmark expectations if relevant
- security expectations if relevant

The core logic contract is the shared concept that every project tutorial and setup-guided implementation should realize consistently.

### `projects/<project>/tutorial/`

The tutorial folder should contain:

- a `README.md` that explains the files in the folder
- a `core.md` walkthrough for the core library repo
- an `adapter.md` walkthrough for each adapter repo run

The walkthrough files should explain:

- the red, green, refactor sequence
- the order in which behaviors should be introduced
- the point at which a thin surface adapter is added

The project tutorial should not duplicate:

- language installation guidance
- framework bootstrap guidance
- test-runner template details
- environment-specific commands that are already covered in `setups/`

If a project eventually grows too large for one file, add a small number of sibling markdown files under `projects/<project>/tutorial/` rather than creating language or framework subtrees.

## Setups

Reusable setup guidance belongs in `setups/`, not inside individual project tutorials.

Recommended shape:

```text
setups/code/<ecosystem>/
setups/code/<ecosystem>/languages/README.md
setups/code/<ecosystem>/languages/<language>.md
setups/code/<ecosystem>/toolchain/README.md
setups/code/<ecosystem>/toolchain/<tool>.md
setups/code/<ecosystem>/adapters/README.md
setups/code/<ecosystem>/adapters/<surface>/README.md
setups/code/<ecosystem>/adapters/<surface>/<target>.md
setups/code/<ecosystem>/testing/README.md
setups/code/<ecosystem>/testing/<framework>.md
setups/code/<ecosystem>/frameworks/README.md
setups/code/<ecosystem>/frameworks/<surface>/README.md
setups/code/<ecosystem>/frameworks/<surface>/<framework>.md
setups/code/<ecosystem>/frameworks/<surface>/<target>/README.md
setups/code/<ecosystem>/frameworks/<surface>/<target>/<framework>.md
setups/storage/
```

That means:

- keep one reusable setup area per code ecosystem
- use `languages/` as a folder with an index plus one markdown file per language
- use `toolchain/` as a folder with an index plus one markdown file per SDK, runtime, CLI, package manager, or equivalent tool
- put surface and target adapter setup under `setups/code/<ecosystem>/adapters/`
- put xUnit, NUnit, MSTest, TUnit, or similar variants under `setups/code/<ecosystem>/testing/`
- put framework bootstrap guidance under `setups/code/<ecosystem>/frameworks/<surface>/` when the framework is tied to a specific surface
- add a target layer under `frameworks/` when the framework only applies to one target such as `web/full-stack`
- keep storage and database concerns under `setups/storage/`
- keep project tutorials focused on the project contract and project flow
- keep environment, template, runner, and coverage-command details in setup docs

This avoids copying the same environment notes across every project tutorial.

## Output Repositories

Following a tutorial should create code in separate repositories outside this curriculum repo.

Recommended model:

- one core library repo owns the shared project rules for a chosen ecosystem, language, and test framework
- one adapter repo owns each chosen surface and target
- adapter repos should consume the core library instead of copying its logic
- adding a new adapter should repeat only the adapter-level TDD work, not the core TDD work

The tutorial repo stays stable while the output repos can evolve independently.

Adapter repo names should always include a storage slot, even when the adapter does not use storage.

Use `no-storage` for runs that do not persist data or depend on a storage service.

### Recommended Output Shape

For a small project such as `saying-hello`, a typical run can produce:

```text
<project>-<ecosystem>-<language>-<test-framework>-core/
  src/
    <core-library>/
  tests/
    <core-library>.Tests/

<project>-<ecosystem>-<language>-<test-framework>-<storage>-<surface>-<target>-<framework>/
  src/
    <adapter>/
  tests/
    <adapter>.Tests/
```

The exact dependency mechanism can vary by ecosystem, but the adapter repo should depend on the core library as a real dependency rather than by duplicating the code.

For example, a minimal console adapter might be named:

```text
saying-hello-dotnet-csharp-xunit-no-storage-command-line-all-no-framework
```

## Allowed Surfaces And Targets

Current surface taxonomy:

- `command-line`
- `web`
- `desktop`
- `mobile`

Current target taxonomy:

- `command-line/all`
- `web/back-end`
- `web/front-end`
- `web/full-stack`
- `desktop/all`
- `desktop/macos`
- `desktop/linux`
- `desktop/windows`
- `mobile/all`
- `mobile/ios`
- `mobile/android`

Using explicit target names, including `all`, keeps the platform vocabulary consistent across specs, curriculum entries, and setup guides.

## Curriculum Map

Curriculum order belongs in [docs/curriculum.md](curriculum.md), not in folder names.

Keep shared option catalogs near the top of that file, such as:

- `code/`
- one section per ecosystem
- `frameworks/` grouped by surface, then target
- `languages/`
- `testing/`
- `toolchain/`
- `storage/`

That map should carry:

- current stage
- project slug
- prerequisites
- unlocks
- recommended tutorial
- notes

Avoid planning dozens of speculative projects far in advance. Add projects when there is genuine intent to build them, and reorder the map as the curriculum becomes clearer.

## Testing And Coverage Policy

This repo is meant to be spec-driven and test-driven from the beginning.

The normal sequence should be:

1. refine the spec
2. turn the spec into tests
3. implement the smallest slice that makes those tests pass
4. refactor
5. review coverage before calling the slice done

Repo-wide baseline:

- `90%` code coverage
- `85%` branch coverage

Stricter rule for validation and service logic:

- `100%` code coverage
- `100%` branch coverage

If a stack cannot produce a trustworthy branch-coverage metric with the approved toolchain, that limitation must be documented explicitly in the spec and the tutorial. That is a tooling exception, not a testing exception.

## Suggested Authoring Workflow

1. Create or refine `projects/<project>/README.md`.
2. Add or update the project's entry in [docs/curriculum.md](curriculum.md).
3. Create or refine the relevant setup docs in `setups/code/<ecosystem>/` or `setups/storage/`.
4. Create or refine `projects/<project>/spec/README.md`.
5. Write the project tutorial index plus `projects/<project>/tutorial/core.md` and `projects/<project>/tutorial/adapter.md`.
6. Define the expected output repos for a normal tutorial run.
7. Build the core library repo by combining the project tutorial with one chosen setup path.
8. Build one or more adapter repos that consume that core library.
9. Keep adapter code thin and keep business rules in the core logic unit.
10. Review code coverage and branch coverage before marking the tutorial complete.

## Real-World Example

For `saying-hello`, the project spec defines a tiny contract such as `greet(name) -> string`.

Then one possible early path is:

1. write [projects/saying-hello/spec/README.md](../projects/saying-hello/spec/README.md)
2. add `saying-hello` to [docs/curriculum.md](curriculum.md)
3. create reusable `.NET` setup docs such as `setups/code/dotnet/languages/csharp.md`, `setups/code/dotnet/toolchain/sdk.md`, `setups/code/dotnet/toolchain/dotnet-cli.md`, `setups/code/dotnet/testing/xunit.md`, adapter guides like `setups/code/dotnet/adapters/command-line/all.md` or `setups/code/dotnet/adapters/web/full-stack.md`, and framework guides like `setups/code/dotnet/frameworks/command-line/spectre-console.md` or `setups/code/dotnet/frameworks/web/full-stack/blazor-server.md`
4. write [projects/saying-hello/tutorial/README.md](../projects/saying-hello/tutorial/README.md), [projects/saying-hello/tutorial/core.md](../projects/saying-hello/tutorial/core.md), and [projects/saying-hello/tutorial/adapter.md](../projects/saying-hello/tutorial/adapter.md)
5. build a separate core library repo and test `greet` to the repo's coverage standard
6. build a separate adapter repo for the chosen surface path
7. keep the adapter thin and the greeting rules in the tested core logic repo
