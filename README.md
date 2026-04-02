<!-- breadcrumbs:start -->
for-all_tutorials
<!-- breadcrumbs:end -->

# For All Tutorials

Comparable, spec-driven tutorials organized as a project-first curriculum.

This repository stores the tutorials themselves. The code produced by following a tutorial should live in separate implementation repositories outside this repo.

The repo is organized around stable project slugs instead of level IDs in paths:

- `projects/` keeps each project's overview, spec, and tutorial materials together.
- `setups/` captures reusable code-ecosystem, testing, adapter, framework, and storage setup guidance.
- `docs/` explains the architecture and keeps the mutable curriculum map.

The current setup catalog includes command-line and `web/full-stack` framework paths across multiple ecosystems so the same project can later be built and compared in several stacks.

Curriculum order lives in [docs/curriculum.md](docs/curriculum.md), not in folder names. That keeps project paths stable while the learning sequence evolves.

The default authoring flow is:

1. refine the project spec
2. update the curriculum map
3. add or confirm the relevant setup guides
4. write the project tutorial plus core and adapter walkthroughs
5. translate the core part of that walkthrough into a separate core-library repo
6. translate the adapter part into one or more separate adapter repos
7. review code and branch coverage before calling the tutorial complete

Repo-wide coverage policy:

- baseline: `90%` code coverage and `85%` branch coverage
- validation and service logic: `100%` code coverage and `100%` branch coverage

## Contents

- [Docs](docs/README.md)
- [Projects](projects/README.md)
- [Setups](setups/README.md)
