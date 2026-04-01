<!-- breadcrumbs:start -->
for-all_tutorials
<!-- breadcrumbs:end -->

# For All Tutorials

Comparable, spec-driven tutorials organized as a project-first curriculum.

The repo is organized around stable project slugs instead of level IDs in paths:

- `specs/` defines the canonical contract for each project app.
- `setups/` captures reusable language, tooling, testing, and framework setup guidance.
- `tutorials/` contains per-project, stack-agnostic TDD walkthroughs.
- `docs/` explains the architecture and keeps the mutable curriculum map.

Curriculum order lives in [docs/curriculum.md](docs/curriculum.md), not in folder names. That keeps project paths stable while the learning sequence evolves.

The default authoring flow is:

1. refine the spec
2. update the curriculum map
3. add or confirm the relevant setup guides
4. write the project tutorial as a stack-agnostic TDD walkthrough
5. translate that walkthrough into code using one chosen setup path
6. add a thin surface adapter when the project requires one
7. review code and branch coverage before calling the tutorial complete

Repo-wide coverage policy:

- baseline: `90%` code coverage and `85%` branch coverage
- validation and service logic: `100%` code coverage and `100%` branch coverage

## Contents

- [Docs](docs/README.md)
- [Specs](specs/README.md)
- [Setups](setups/README.md)
- [Tutorials](tutorials/README.md)
