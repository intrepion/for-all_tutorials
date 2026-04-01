<!-- breadcrumbs:start -->
for-all_tutorials
<!-- breadcrumbs:end -->

# For All Tutorials

Comparable, spec-driven tutorials organized as a long-form curriculum, with room to grow into other platform categories later.

The repo is organized as a curriculum-style tutorial matrix instead of a pure framework dump:

- `specs/` defines the canonical requirements, testing expectations, benchmark expectations, and security checklist for each tutorial app.
- `tutorials/` contains the per-stack tutorial folders, grouped by level, project, platform, and framework.
- `docs/` explains the architecture and growth path.

The platform layer inside `tutorials/` makes room for future `back-end`, `front-end`, `mobile`, `desktop`, and platform-specific stacks without reshaping the repo again, and the zero-padded `level-001` style folders keep the curriculum sortable as it grows past 100 levels.

The default authoring flow is:

1. refine the spec
2. translate the spec into tests
3. implement against those tests
4. review code and branch coverage before calling the tutorial complete

That means these tutorials should be spec-driven and test-driven from the beginning, with coverage pushed high early instead of being treated as a later cleanup step.

See `docs/architecture.md` for the recommended directory structure and the tradeoffs behind it.

## Contents

- [Docs](docs/README.md)
- [Specs](specs/README.md)
- [Tutorials](tutorials/README.md)
