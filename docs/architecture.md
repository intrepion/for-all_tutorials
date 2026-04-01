<!-- breadcrumbs:start -->
[for-all_tutorials](../README.md) / [Docs](README.md) / architecture.md
<!-- breadcrumbs:end -->

# Repository Architecture

Practical guidance for organizing this repo as a spec-driven, test-driven tutorial curriculum with stable project paths and a separate curriculum map.

## Recommendation

Use a project-first structure with curriculum ordering kept outside the filesystem:

- keep stable project slugs in `specs/` and `tutorials/`
- keep curriculum order in [docs/curriculum.md](curriculum.md)
- treat each project spec as the canonical contract
- split tutorials into a core-logic `libraries` branch and a surface-adapter `surfaces` branch
- require tests and coverage from the beginning instead of treating them as cleanup

This separates identity from ordering:

- the filesystem answers "what is this project?"
- the curriculum map answers "when should this be learned?"

## Core Principles

- project slugs are stable
- curriculum order is mutable
- every tutorial is still a project app
- specs are the source of truth
- tests are the executable form of the spec
- code coverage and branch coverage are part of done
- surface tutorials should keep adapters thin by building on a language-level core first

## Recommended Tree

```text
for-all_tutorials/
  docs/
    architecture.md
    curriculum.md

  specs/
    saying-hello/
      README.md

  tutorials/
    saying-hello/
      README.md
      libraries/
        README.md
      surfaces/
        README.md
        command-line/
          all/
            README.md
        web/
          back-end/
            README.md
          front-end/
            README.md
          full-stack/
            README.md
        desktop/
          all/
            README.md
          macos/
            README.md
          linux/
            README.md
          windows/
            README.md
        mobile/
          all/
            README.md
          ios/
            README.md
          android/
            README.md
```

## Specs

Each project spec is the canonical contract for every tutorial built from that project.

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

The core logic contract is the shared concept that library tutorials and surface tutorials both implement.

## Tutorials

Each project gets one tutorial root:

```text
tutorials/<project>/
```

Inside that root, tutorials split into two branches:

```text
tutorials/<project>/libraries/
tutorials/<project>/surfaces/
```

### `libraries/`

Library tutorials teach the language-level core logic for the project before any platform adapter is added.

Recommended shape:

```text
tutorials/<project>/libraries/<language>/
```

Each library tutorial should:

- implement the core logic contract from the spec
- focus on domain logic, validation, and service behavior
- establish the strongest tests and coverage first
- be usable as the conceptual base for later surface tutorials in the same language

### `surfaces/`

Surface tutorials adapt the project to a user-facing environment.

Recommended shape:

```text
tutorials/<project>/surfaces/<surface>/<target>/<framework>/
```

Each surface tutorial should:

- point back to the relevant library tutorial when one exists
- keep surface-specific code thin
- avoid moving core rules into the UI or transport layer
- focus on adapting the already-defined project behavior to the selected surface

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

Using explicit `target` folders, including `all`, keeps the tree uniform and scriptable.

## Curriculum Map

Curriculum order belongs in [docs/curriculum.md](curriculum.md), not in folder names.

That map should carry:

- current stage
- project slug
- status such as `planned`, `in-progress`, or `complete`
- prerequisites
- required surfaces
- recommended first library tutorial
- recommended first surface tutorial
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

1. Create or refine `specs/<project>/README.md`.
2. Add or update the project's entry in [docs/curriculum.md](curriculum.md).
3. Write the first language library tutorial in `tutorials/<project>/libraries/<language>/`.
4. Build tests for the core logic before building the surface adapter.
5. Add the first surface tutorial in `tutorials/<project>/surfaces/<surface>/<target>/<framework>/`.
6. Keep adapter code thin and keep business rules in the library-level core.
7. Review code coverage and branch coverage before marking the tutorial complete.

## Real-World Example

For `saying-hello`, the project spec defines a tiny contract such as `greet(name) -> string`.

Then the recommended first path is:

1. write `specs/saying-hello/README.md`
2. add `saying-hello` to [docs/curriculum.md](curriculum.md)
3. create `tutorials/saying-hello/libraries/<language>/` and implement `greet`
4. test that logic to the repo's coverage standard
5. create `tutorials/saying-hello/surfaces/command-line/all/<framework>/`
6. wrap the already-tested `greet` logic in a thin CLI adapter
