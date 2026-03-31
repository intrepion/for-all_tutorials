<!-- breadcrumbs:start -->
[for-all_tutorials](../README.md) / [Docs](README.md) / architecture.md
<!-- breadcrumbs:end -->

# Repository Architecture

Structure, naming conventions, and workflow guidance for organizing comparable tutorials as a long-form curriculum across platforms and frameworks.

## Recommendation

Use a curriculum structure with an embedded platform layer inside each project:

- Keep canonical app definitions in `specs/`.
- Group tutorial folders by `level -> project -> platform -> framework`.
- Use zero-padded level slugs such as `level-001-foundations`.
- Keep benchmark and security expectations inside each spec until there are real shared assets that deserve their own home.

That gives you the best of both worlds:

- Project-first organization still makes cross-framework comparison easy.
- Curriculum-first levels make the repo read like a learning path instead of a dump of app ideas.
- The platform layer keeps the repo ready for non-full-stack stacks.
- The top level stays lean while the repo is still primarily a documentation catalog.

## Why Curriculum Levels Matter

This repo is no longer organized like a simple benchmark queue. It behaves more like a course catalog.

That means a level should represent teaching sequence and prerequisite order, not just rough app size. More levels with fewer projects per level create:

- smaller jumps in complexity
- clearer prerequisites
- room to insert future tutorials between existing ones
- stable alphabetical ordering as the catalog grows past 100 levels

That is why the recommended level folders use three digits from the start, such as `level-001`, `level-002`, and `level-103`.

## Current Curriculum Map

- `level-001-foundations`: Guestbook / Comment Wall, Todo List, Poll / Voting App, Text Sharing
- `level-002-crud-and-routing`: Habit Tracker, Bookmark Manager / Read-Later App, URL Shortener, Personal Link-in-Bio + Analytics
- `level-003-state-and-views`: Trivia App, Leaderboard App, Weather Dashboard, Notification Center
- `level-004-users-and-auth`: Auth + Profile Portal, Expense Tracker, Markdown Blog Engine, Survey Builder
- `level-005-uploads-and-media`: Image Gallery with Uploads, File Upload Vault, Image Upload & Processing Service
- `level-006-business-workflows`: Inventory Tracker, Wiki / Knowledge Base, Support Ticket System, Admin Dashboard
- `level-007-collaboration`: Forum / Discussion Board, Issue Tracker, Calendar / Appointment Scheduler, Kanban Board / Project Board
- `level-008-realtime-and-commerce`: Chat / Messaging App, Mini E-Commerce App
- `level-009-systems`: API Gateway / Auth Proxy, Collaborative Docs, Log Ingestor

## Why The Platform Layer Still Matters

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

If the repo skips that layer inside `tutorials/` now, you would have to reorganize the whole tree later. Keeping it in the tutorial path makes the structure stable without needing a separate top-level `frameworks/` folder yet.

## Recommended Tree

```text
for-all_tutorials/
  docs/
    architecture.md

  specs/
    level-001-foundations/
      guestbook-comment-wall/
      todo-list/
    level-002-crud-and-routing/
    level-003-state-and-views/
    ...
    level-009-systems/

  tutorials/
    level-001-foundations/
      todo-list/
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
    level-002-crud-and-routing/
    level-003-state-and-views/
    ...
    level-009-systems/
```

## Path Shape

Recommended tutorial path:

```text
tutorials/<level>/<project>/<platform>/<framework-id>/
```

Current example:

```text
tutorials/level-001-foundations/todo-list/full-stack/elixir-phoenix-liveview/
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

If two tutorial builds behave differently, the spec is where you resolve it.

### `tutorials/`

This is where the comparable tutorial writing lives.

The project folder groups by platform before framework so you can compare:

- the same project tutorial across frameworks within one platform
- the same project tutorial across different platform categories

The level folder above that project tells readers where the tutorial fits in the curriculum.

## Naming Conventions

Use stable slugs so the matrix stays sortable and scriptable:

- levels: `level-001-foundations`, `level-002-crud-and-routing`, `level-003-state-and-views`
- projects: `guestbook-comment-wall`, `todo-list`, `poll-voting-app`
- platforms: `full-stack`, later `back-end`, `front-end`, `mobile`, and others
- frameworks: `rust-leptos-ssr`, `elixir-phoenix-liveview`, `ruby-rails-hotwire`

Avoid display names in folder paths.

## Suggested Authoring Workflow

1. Add or refine the app contract in `specs/<level>/<project>/`.
2. Choose or add the platform group in `tutorials/<level>/<project>/<platform>/`.
3. Write or refine the stack-specific tutorial in `tutorials/<level>/<project>/<platform>/<framework-id>/`.
4. If a tutorial's teaching scope changes, move it to the level that best matches its prerequisites.
5. If real cross-project framework guidance appears later, introduce a dedicated `frameworks/` folder only when it contains genuinely reusable material.

## Real-World Example

For `Todo List`, the curriculum question is not just "is this easy?" It is "is this the right first durable full-stack exercise after the absolute basics?"

That is why it now lives under:

```text
specs/level-001-foundations/todo-list/
tutorials/level-001-foundations/todo-list/full-stack/
```

It teaches forms, validation, CRUD, filtering, and persistence with low cognitive overhead, which makes it a much better foundation-level tutorial than something like `Chat / Messaging App` or `API Gateway / Auth Proxy`.
