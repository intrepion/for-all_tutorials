<!-- breadcrumbs:start -->
for-all_tutorials
<!-- breadcrumbs:end -->

# For All Tutorials

Comparable, spec-driven tutorials organized as a project-first curriculum.

[![CI](https://github.com/intrepion/for-all_tutorials/actions/workflows/ci.yml/badge.svg)](https://github.com/intrepion/for-all_tutorials/actions/workflows/ci.yml)

This repository stores tutorial source material plus generated tutorial outputs. The code produced by following a tutorial should live in separate implementation repositories outside this repo.

The repo is organized around stable project slugs instead of level IDs in paths:

- `partials/projects/` keeps each project's overview, spec, instructions, and manifest together.
- `setups/` captures the broader reusable code-ecosystem, testing, adapter, framework, and storage setup guidance while `partials/setups/` is still in a pilot phase.
- `tutorials/` is generated output and should be regenerated rather than edited by hand.
- `docs/` explains the architecture and keeps the mutable curriculum map.

The current setup catalog includes command-line and `web/full-stack` framework paths across multiple ecosystems so the same project can later be built and compared in several stacks.

Curriculum order lives in [docs/curriculum.md](docs/curriculum.md), not in folder names. That keeps project paths stable while the learning sequence evolves.

The default authoring flow is:

1. refine the project spec
2. update the curriculum map
3. add or confirm the relevant setup guides
4. write or refine the project instructions and manifest in `partials/projects/`
5. regenerate the compiled tutorials from `partials/`
6. translate the core part of that compiled tutorial into a separate core-library repo
7. translate the adapter part into one or more separate adapter repos
8. review code and branch coverage before calling the tutorial complete

Repo-wide coverage policy:

- baseline: `90%` code coverage and `85%` branch coverage
- validation and service logic: `100%` code coverage and `100%` branch coverage

## Contents

- [Docs](docs/README.md)
- [Partials](partials/README.md)
- [Setups](setups/README.md)
