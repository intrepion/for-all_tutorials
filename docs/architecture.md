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
      dotnet/
        README.md
        languages.md
        toolchain.md
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
        tdd.md
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
    tdd.md
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
- a `tdd.md` walkthrough that explains the stack-agnostic red, green, refactor sequence

The TDD walkthrough should explain:

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
setups/code/<ecosystem>/languages.md
setups/code/<ecosystem>/toolchain.md
setups/code/<ecosystem>/adapters/<surface>/<target>.md
setups/code/<ecosystem>/testing/<framework>.md
setups/code/<ecosystem>/frameworks/<framework>.md
setups/storage/
```

That means:

- keep one reusable setup area per code ecosystem
- put language choices in `languages.md`
- standardize core ecosystem guidance in `toolchain.md`
- put surface and target adapter setup under `setups/code/<ecosystem>/adapters/`
- put xUnit, NUnit, MSTest, TUnit, or similar variants under `setups/code/<ecosystem>/testing/`
- put framework bootstrap guidance under `setups/code/<ecosystem>/frameworks/`
- keep storage and database concerns under `setups/storage/`
- keep project tutorials focused on the project contract and project flow
- keep environment, template, runner, and coverage-command details in setup docs

This avoids copying the same environment notes across every project tutorial.

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

That map should carry:

- current stage
- project slug
- status such as `planned`, `in-progress`, or `complete`
- prerequisites
- surface options
- recommended tutorial
- suggested setup paths
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
5. Write the project tutorial index and `projects/<project>/tutorial/tdd.md`.
6. Build the code by combining the project tutorial with one chosen setup path.
7. Keep adapter code thin and keep business rules in the core logic unit.
8. Review code coverage and branch coverage before marking the tutorial complete.

## Real-World Example

For `saying-hello`, the project spec defines a tiny contract such as `greet(name) -> string`.

Then one possible early path is:

1. write [projects/saying-hello/spec/README.md](../projects/saying-hello/spec/README.md)
2. add `saying-hello` to [docs/curriculum.md](curriculum.md)
3. create reusable `.NET` setup docs such as `setups/code/dotnet/toolchain.md`, `setups/code/dotnet/testing/xunit.md`, and adapter guides like `setups/code/dotnet/adapters/command-line/all.md` or `setups/code/dotnet/adapters/web/full-stack.md`
4. write [projects/saying-hello/tutorial/README.md](../projects/saying-hello/tutorial/README.md) and [projects/saying-hello/tutorial/tdd.md](../projects/saying-hello/tutorial/tdd.md)
5. test `greet` to the repo's coverage standard using one chosen setup path
6. add the chosen adapter for that run
7. keep the adapter thin and the greeting rules in the tested core logic
