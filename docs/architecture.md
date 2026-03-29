# Repository Architecture

## Recommendation

Use a hybrid structure:

- Group runnable apps by `level -> project -> framework`.
- Keep canonical app definitions in `specs/`.
- Keep reusable framework setup in `frameworks/`.

That gives you the best of both worlds:

- Project-first organization makes cross-framework comparison easy.
- Framework-specific starter code stays reusable instead of being copied into every app.
- Future benchmarks and security checks can target the same app contract across all stacks.

## Why Not Pure Framework-First?

If you group everything under a framework, this is what happens later:

- Comparing `Guestbook` across nine stacks becomes tedious.
- It is harder to verify feature parity before benchmarking.
- Readers who want to learn one app pattern across languages have to jump around the repo.

Framework-first is still useful, but only for shared starter templates, conventions, and scripts.

## Why Not Pure Project-First?

Pure project-first also has a weakness:

- You end up repeating framework bootstrapping and setup notes over and over.

That is why this repo should keep a separate `frameworks/` area for reusable setup.

## Recommended Tree

```text
for-all_tutorials/
  docs/
    architecture.md

  specs/
    level-1/
      guestbook/
        scope.md
        routes.md
        data-model.md
        acceptance.md
        benchmark-contract.md
        security-checklist.md
    level-2/
    level-3/
    level-4/
    level-5/

  tutorials/
    level-1/
      guestbook/
        rust-leptos-ssr/
        elixir-phoenix-liveview/
        ruby-rails-hotwire/
        java-quarkus-qute/
        csharp-blazor-server/
        php-laravel-octane-livewire/
        typescript-sveltekit-bun/
        python-fastapi-jinja-htmx/
        go-echo-templates-htmx/
      todo-list/
      poll-voting-app/
    level-2/
    level-3/
    level-4/
    level-5/

  frameworks/
    rust-leptos-ssr/
      starter/
      conventions.md
      scripts/
    elixir-phoenix-liveview/
    ruby-rails-hotwire/
    java-quarkus-qute/
    csharp-blazor-server/
    php-laravel-octane-livewire/
    typescript-sveltekit-bun/
    python-fastapi-jinja-htmx/
    go-echo-templates-htmx/

  shared/
    benchmarking/
    security/
    fixtures/
    contracts/
```

## How To Think About Each Folder

### `specs/`

This is the source of truth for parity.

Each project spec should define:

- required features
- excluded features
- routes and page states
- data model
- validation rules
- auth rules if applicable
- benchmark contract
- security checklist

If two implementations behave differently, the spec is where you resolve it.

### `tutorials/`

This is where the real comparable apps live.

Recommended path shape:

```text
tutorials/<level>/<project>/<framework-id>/
```

Example:

```text
tutorials/level-1/guestbook/rust-leptos-ssr/
tutorials/level-1/guestbook/elixir-phoenix-liveview/
```

This makes it easy to answer questions like:

- How do all nine stacks implement the same guestbook?
- Which level 2 apps are already complete?
- Which framework is missing a version of the issue tracker?

## Naming Conventions

Use stable slugs so the matrix stays sortable and scriptable:

- `level-1`, `level-2`, `level-3`, `level-4`, `level-5`
- `guestbook`, `todo-list`, `poll-voting-app`
- `rust-leptos-ssr`, `elixir-phoenix-liveview`, `ruby-rails-hotwire`

Avoid display names in folder paths.

## Suggested Authoring Workflow

1. Add or refine the app contract in `specs/level-x/<project>/`.
2. Reuse or improve the framework starter in `frameworks/<framework-id>/`.
3. Build the comparable app in `tutorials/level-x/<project>/<framework-id>/`.
4. Attach shared checks from `shared/` before calling the tutorial complete.

## Real-World Example

For `Guestbook`, the fair comparison question is:

"How do nine stacks implement the same tiny SSR CRUD app with the same validation and the same benchmark contract?"

That question is easiest to answer when all guestbook implementations are neighbors under one folder, while the framework setup lives elsewhere for reuse.

If you want a single rule to anchor the repo around, use this one:

Group implementations by project, not by framework. Keep framework reuse in a separate support layer.
