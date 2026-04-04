<!-- breadcrumbs:start -->
[for-all_tutorials](../README.md) / [Docs](README.md) / architecture.md
<!-- breadcrumbs:end -->

# Repository Architecture

Practical guidance for organizing this repo as a spec-driven, test-driven tutorial curriculum with stable project paths, source partials, generated tutorials, and a separate curriculum map.

## Recommendation

Use a project-first structure with curriculum ordering kept outside the filesystem:

- keep stable project slugs in `partials/projects/`
- keep generated learner-facing outputs in `tutorials/`
- keep reusable setup guidance in `setups/` while `partials/setups/` is still being scaled out
- keep curriculum order in [docs/curriculum.md](curriculum.md)
- keep each project's overview, spec, instructions, and manifest together
- require tests and coverage from the beginning instead of treating them as cleanup

This separates identity from ordering:

- the filesystem answers "what is this project?"
- the curriculum map answers "when should this be learned?"

## Core Principles

- project slugs are stable
- curriculum order is mutable
- every tutorial is still a project app
- each project keeps its own spec, instructions, and manifest side by side
- setups capture reusable environment and tooling guidance
- this repo stores tutorial source material and generated tutorials, not the resulting implementation repos
- tests are the executable form of the spec
- code coverage and branch coverage are part of done
- project instructions and compiled tutorials should keep adapters thin by building on a well-tested core logic unit first

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
        storage/
          README.md
          command-line/
            README.md
            database-firebase.md
            local-files-csv.md
            local-files-json.md
            local-files-yaml.md
            local-files-toml.md
            local-files-xml.md
          web/
            README.md
            database-sqlite.md
        adapters/
          README.md
          client/
            README.md
            all.md
          command-line/
            README.md
            all.md
          graphical/
            README.md
            all.md
          web/
            README.md
            full-stack.md
          web-service/
            README.md
            all.md
        testing/
          README.md
          xunit.md
          nunit.md
          mstest.md
          tunit.md
        frameworks/
          README.md
          client/
            README.md
            no-framework.md
          command-line/
            README.md
            no-framework.md
            spectre-console.md
          graphical/
            README.md
            no-framework.md
          web/
            README.md
            full-stack/
              README.md
              blazor-server.md
          web-service/
            README.md
            no-framework.md
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
      no-storage.md
      local-files-csv.md
      local-files-json.md
      local-files-yaml.md
      local-files-toml.md
      local-files-xml.md
      database-firebase.md
      database-sqlite.md
      database-postgres.md
      database-mysql.md

  partials/
    README.md
    projects/
      README.md
      saying-hello/
        README.md
        manifest.yaml
        spec/
          README.md
        instructions/
          README.md
          core.md
          adapter.md

  tutorials/
    saying-hello/
      dotnet/
        csharp/
          xunit/
            core/
              README.md
            adapters/
              no-storage/
                command-line/
                  all/
                    no-framework/
                      README.md
```

## Project Partials

Each project source slice should keep its own materials together:

```text
partials/projects/<project>/
  README.md
  manifest.yaml
  spec/
    README.md
  instructions/
    README.md
    core.md
    adapter.md
```

This lets one folder answer three questions cleanly:

- what is this project?
- what is the canonical contract?
- how should it be assembled into compiled tutorials?

### `partials/projects/<project>/spec/`

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

The core logic contract is the shared concept that every project instruction slice and setup-guided implementation should realize consistently.

### `partials/projects/<project>/instructions/`

The instructions folder should contain:

- a `README.md` source index that explains the files in the folder
- a `core.md` instruction fragment for the core library repo
- an `adapter.md` instruction fragment for each adapter repo run

The instruction files should explain:

- the red, green, refactor sequence
- the order in which behaviors should be introduced
- the point at which a thin surface adapter is added

The project instructions should not duplicate:

- language installation guidance
- framework bootstrap guidance
- test-runner template details
- environment-specific commands that are already covered in `setups/`

If a project eventually grows too large for one file, add a small number of sibling markdown files under `partials/projects/<project>/instructions/` rather than creating language or framework subtrees.

### `partials/projects/<project>/manifest.yaml`

The manifest declares which compiled tutorial outputs are actively supported.

At minimum, each manifest should define:

- the stable project slug
- a manifest version
- the active compiled outputs
- the selected stack for each output
- the source partials that feed each output

The safest rollout pattern is to activate the most complete stack first, prove generation end to end, and then expand adapter variants as more setup partials become available.

## Generated Tutorials

Generated learner-facing tutorials belong under `tutorials/`.

Recommended shape:

```text
tutorials/<project>/<ecosystem>/<language>/<testing>/core/README.md
tutorials/<project>/<ecosystem>/<language>/<testing>/adapters/<storage>/<surface>/<target>/<framework>/README.md
```

These files should:

- be generated from `partials/`
- be safe to delete and regenerate
- never be treated as the canonical authoring source
- match the stack encoded in their path exactly

## Setups

Reusable setup guidance belongs in `setups/`, not inside individual project instruction slices or generated tutorials.

Recommended shape:

```text
setups/code/<ecosystem>/
setups/code/<ecosystem>/languages/README.md
setups/code/<ecosystem>/languages/<language>.md
setups/code/<ecosystem>/toolchain/README.md
setups/code/<ecosystem>/toolchain/<tool>.md
setups/code/<ecosystem>/storage/README.md
setups/code/<ecosystem>/storage/<surface>/README.md
setups/code/<ecosystem>/storage/<surface>/<storage>.md
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
- use ecosystem `storage/` guides for stack-specific persistence implementation details layered on top of the shared storage taxonomy
- put surface and target adapter setup under `setups/code/<ecosystem>/adapters/`
- put xUnit, NUnit, MSTest, TUnit, or similar variants under `setups/code/<ecosystem>/testing/`
- put framework bootstrap guidance under `setups/code/<ecosystem>/frameworks/<surface>/` when the framework is tied to a specific surface
- add a target layer under `frameworks/` when the framework only applies to one target such as `web/full-stack`
- keep shared storage taxonomy and cross-ecosystem storage meanings under `setups/storage/`
- keep ecosystem-and-surface-specific storage implementation guidance under `setups/code/<ecosystem>/storage/`
- keep project instructions and generated tutorials focused on the project contract and project flow
- keep environment, template, runner, and coverage-command details in setup docs

This avoids copying the same environment notes across every project instruction slice and generated tutorial.

## Output Repositories

Following a tutorial should create code in separate repositories outside this curriculum repo.

Recommended model:

- one core library repo owns the shared project rules for a chosen ecosystem, language, and test framework
- one adapter repo owns each chosen surface and target
- adapter repos should consume the core library instead of copying its logic
- adding a new adapter should repeat only the adapter-level TDD work, not the core TDD work

The tutorial repo stays stable while the output repos can evolve independently.

Adapter repo names should always include a storage slot, even when the adapter does not use storage.

Current storage taxonomy:

- `no-storage`
- `local-files-csv`
- `local-files-json`
- `local-files-yaml`
- `local-files-toml`
- `local-files-xml`
- `database-firebase`
- `database-sqlite`
- `database-postgres`
- `database-mysql`

Use `no-storage` for runs that do not persist data or depend on a storage service.

By default, `local-files-*` slots are for command-line, desktop, and other local single-user adapters. Do not combine `local-files-*` with web adapters unless a project explicitly requires that shape.

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

- `client`
- `command-line`
- `graphical`
- `web`
- `web-service`
- `desktop`
- `mobile`

Current target taxonomy:

- `client/all`
- `command-line/all`
- `graphical/all`
- `web/back-end`
- `web/front-end`
- `web/full-stack`
- `web-service/all`
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
- recommended instructions
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

1. Create or refine `partials/projects/<project>/README.md`.
2. Add or update the project's entry in [docs/curriculum.md](curriculum.md).
3. Create or refine the relevant setup docs in `setups/code/<ecosystem>/` or `setups/storage/`.
4. Create or refine `partials/projects/<project>/spec/README.md`.
5. Write the project instruction index plus `partials/projects/<project>/instructions/core.md` and `partials/projects/<project>/instructions/adapter.md`.
6. Define the active compiled outputs in `partials/projects/<project>/manifest.yaml`.
7. Regenerate `tutorials/` from `partials/`.
8. Build the core library repo by combining the compiled tutorial with one chosen setup path.
9. Build one or more adapter repos that consume that core library.
10. Keep adapter code thin and keep business rules in the core logic unit.
11. Review code coverage and branch coverage before marking the tutorial complete.

## Real-World Example

For `saying-hello`, the project spec defines a tiny contract such as `greet(name) -> string`.

Then one possible early path is:

1. write [partials/projects/saying-hello/spec/README.md](../partials/projects/saying-hello/spec/README.md)
2. add `saying-hello` to [docs/curriculum.md](curriculum.md)
3. create reusable `.NET` setup docs such as `setups/code/dotnet/languages/csharp.md`, `setups/code/dotnet/toolchain/sdk.md`, `setups/code/dotnet/toolchain/dotnet-cli.md`, `setups/code/dotnet/testing/xunit.md`, adapter guides like `setups/code/dotnet/adapters/command-line/all.md` or `setups/code/dotnet/adapters/web/full-stack.md`, and framework guides like `setups/code/dotnet/frameworks/command-line/spectre-console.md` or `setups/code/dotnet/frameworks/web/full-stack/blazor-server.md`
4. write [partials/projects/saying-hello/instructions/README.md](../partials/projects/saying-hello/instructions/README.md), [partials/projects/saying-hello/instructions/core.md](../partials/projects/saying-hello/instructions/core.md), and [partials/projects/saying-hello/instructions/adapter.md](../partials/projects/saying-hello/instructions/adapter.md)
5. declare the active outputs in [partials/projects/saying-hello/manifest.yaml](../partials/projects/saying-hello/manifest.yaml)
6. generate [tutorials/saying-hello/dotnet/csharp/xunit/core/README.md](../tutorials/saying-hello/dotnet/csharp/xunit/core/README.md) and [tutorials/saying-hello/dotnet/csharp/xunit/adapters/no-storage/command-line/all/no-framework/README.md](../tutorials/saying-hello/dotnet/csharp/xunit/adapters/no-storage/command-line/all/no-framework/README.md)
7. build a separate core library repo and test `greet` to the repo's coverage standard
8. build a separate adapter repo for the chosen surface path
9. keep the adapter thin and the greeting rules in the tested core logic repo
