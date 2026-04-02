<!-- breadcrumbs:start -->
[for-all_tutorials](../README.md) / Projects
<!-- breadcrumbs:end -->

# Projects

Project-specific materials, with each project keeping its own overview, spec, and tutorial documents together.

## Contents

- [Counting the Number of Characters](count-characters/README.md)
- [Mad Lib](mad-lib/README.md)
- [Printing Quotes](printing-quotes/README.md)
- [Retirement Calculator](retirement-calculator/README.md)
- [Saying Hello](saying-hello/README.md)
- [Simple Math](simple-math/README.md)

## Shared Workflow

Every project in this folder follows the same high-level pattern:

1. Use the project `spec/README.md` as the source of truth.
2. Use the project `tutorial/README.md`, `tutorial/core.md`, and `tutorial/adapter.md` as the teaching path.
3. Use `setups/` for stack-specific implementation details.
4. Build the core logic first in a separate core library repo.
5. Add one or more separate adapter repos only after the core logic is tested.

## Shared Output Model

The code produced from these tutorials should live outside this curriculum repo.

The default model is:

- one core library repo owns the project rules
- one adapter repo owns each chosen surface and target
- adapter repos depend on the core library instead of copying its logic

## Shared Spec Conventions

Project specs in this folder share these rules:

- a single run only needs one chosen surface and target path
- the currently supported setup paths belong in `setups/`, not in each spec
- adapters must not redefine the core project rules

## Shared Coverage Policy

Unless a local spec explicitly declares an exception, projects in this folder use this coverage policy:

- baseline coverage is `90%` code and `85%` branch
- core validation and service logic should target `100%` code and `100%` branch

Local specs should only restate coverage numbers when a project has a special-case requirement that differs from this shared policy.

## Shared Tutorial Workflow

Project tutorial indexes in this folder share this flow:

1. read the local spec
2. choose one setup path
3. create a separate core library repo
4. use the setup guides for that core repo
5. follow the local core walkthrough for the core repo
6. create a separate adapter repo for the chosen surface and target
7. use the setup guides for that adapter repo
8. follow the local adapter walkthrough for the adapter repo
9. verify that the finished repos still match the local spec

## Shared Finish Checklist

A project run is complete when:

- the behavior matches the local spec
- the project was built in a real red, green, refactor sequence
- the core logic lives in its own thoroughly tested repo
- the chosen adapter is thin and does not redefine the project rules
- the coverage goals are met
