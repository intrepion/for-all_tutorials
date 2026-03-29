# for-all_tutorials

Comparable full-stack tutorials across nine framework and language stacks.

The repo is organized as a comparison matrix instead of a pure framework dump:

- `specs/` defines the canonical requirements for each tutorial app.
- `tutorials/` contains the runnable implementations, grouped by level and project first.
- `frameworks/` holds reusable framework starters and conventions.
- `shared/` keeps cross-framework benchmark, security, fixture, and parity assets.
- `docs/` explains the architecture and growth path.

The key rule is simple: define each app once in `specs/`, then implement it many times in `tutorials/`.

See `docs/architecture.md` for the recommended directory structure and the tradeoffs behind it.
