<!-- breadcrumbs:start -->
[for-all_tutorials](../README.md) / [docs](README.md) / architecture.md
<!-- breadcrumbs:end -->

# Repository Architecture

Structure, naming conventions, and workflow guidance for organizing comparable tutorials across platforms and frameworks.

## Recommendation

Use a hybrid structure with an explicit platform layer:

- Keep canonical app definitions in `specs/`.
- Group tutorial folders by `level -> project -> platform -> framework`.
- Group reusable framework setup by `platform -> framework`.
- Keep benchmark and security expectations inside each spec until truly reusable cross-tutorial assets appear.

That gives you the best of both worlds:

- Project-first organization still makes cross-framework comparison easy.
- The new platform layer keeps the repo ready for non-full-stack stacks.
- Framework-specific starter code stays reusable instead of being copied into every app.
- Benchmark and security rules stay attached to each tutorial contract instead of being split into an empty cross-tutorial layer too early.

## Why The Platform Layer Matters

Today, every tutorial in the repo is full-stack.

Later, you want room for categories such as:

- `back-end`
- `front-end`
- `full-stack`
- `android`
- `ios`
- `mobile`
- `windows`
- `macos`
- `watchos`
- `tvos`
- `linux`
- `desktop`

If the repo skips that layer now, you would have to reorganize the whole tree later. Adding it early keeps the structure stable.

## Recommended Tree

```text
for-all_tutorials/
  docs/
    architecture.md

  specs/
    level-1/
      guestbook-comment-wall/
      todo-list/
    level-2/
    level-3/
    level-4/
    level-5/

  tutorials/
    level-1/
      guestbook-comment-wall/
        full-stack/
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
        full-stack/
    level-2/
    level-3/
    level-4/
    level-5/

  frameworks/
    full-stack/
      rust-leptos-ssr/
        starter/
        scripts/
      elixir-phoenix-liveview/
      ruby-rails-hotwire/
      java-quarkus-qute/
      csharp-blazor-server/
      php-laravel-octane-livewire/
      typescript-sveltekit-bun/
      python-fastapi-jinja-htmx/
      go-echo-templates-htmx/
```

## Path Shapes

Recommended tutorial path:

```text
tutorials/<level>/<project>/<platform>/<framework-id>/
```

Recommended reusable framework path:

```text
frameworks/<platform>/<framework-id>/
```

Current example:

```text
tutorials/level-1/guestbook-comment-wall/full-stack/rust-leptos-ssr/
frameworks/full-stack/rust-leptos-ssr/
```

## How To Think About Each Folder

### `specs/`

This is still the source of truth for parity.

Each project spec should define:

- required features
- excluded features
- routes and page states
- data model
- validation rules
- auth rules if applicable
- benchmark contract
- security checklist

If two tutorial builds behave differently, the spec is where you resolve it.
Benchmark and security expectations should also live here unless multiple tutorials truly reuse the same supporting assets.

### `tutorials/`

This is where the real comparable tutorials live.

The project folder now groups by platform before framework so you can later compare:

- the same project tutorial across frameworks within one platform
- the same project tutorial across different platform categories

### `frameworks/`

This is the reusable support layer for future tutorial writing.

The platform folder sits above the framework folder so future framework families can live beside each other without another repo rewrite.

## Naming Conventions

Use stable slugs so the matrix stays sortable and scriptable:

- levels: `level-1`, `level-2`, `level-3`, `level-4`, `level-5`
- projects: `guestbook-comment-wall`, `todo-list`, `poll-voting-app`
- platforms: `full-stack`, later `back-end`, `front-end`, `mobile`, and others
- frameworks: `rust-leptos-ssr`, `elixir-phoenix-liveview`, `ruby-rails-hotwire`

Avoid display names in folder paths.

## Suggested Authoring Workflow

1. Add or refine the app contract in `specs/level-x/<project>/`.
2. Choose or add the platform group in `tutorials/level-x/<project>/<platform>/`.
3. Reuse or improve the framework starter in `frameworks/<platform>/<framework-id>/`.
4. Write the comparable tutorial in `tutorials/level-x/<project>/<platform>/<framework-id>/`.

## Real-World Example

For `Guestbook / Comment Wall`, the comparison question today is:

"How do the full-stack frameworks teach the same tiny SSR CRUD app with the same validation and the same benchmark contract?"

That is why the current tutorial folders live under:

```text
tutorials/level-1/guestbook-comment-wall/full-stack/
```

Later, if you add a `back-end` or `mobile` version of the same project, it can live beside `full-stack` instead of forcing another top-level reorganization.
