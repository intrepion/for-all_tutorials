<!-- breadcrumbs:start -->
[for-all_tutorials](../../../README.md) / [specs](../../README.md) / [level-1](../README.md) / todo-list
<!-- breadcrumbs:end -->

# Todo List

Canonical spec for this Level 1 comparable tutorial app.

## Goal

Build a small server-backed todo app that exercises the baseline full-stack behaviors you want to compare later:

- forms
- CRUD
- routing
- filtering
- validation
- persistence

This spec should stay intentionally small. It is the reference pattern for future Level 1 specs, not a feature-rich task manager.

## Non-Goals

The canonical version of this app does not include:

- authentication or user accounts
- multi-user collaboration
- due dates or reminders
- tags, labels, or projects
- drag and drop ordering
- file uploads
- realtime sync
- offline sync
- notifications
- analytics beyond basic app logging

## Core Features

The canonical app must support:

- creating a todo
- listing all todos
- editing an existing todo title
- toggling a todo between active and completed
- deleting a todo
- filtering the list by `all`, `active`, and `completed`
- preserving todos across page reloads and app restarts
- showing validation feedback for invalid input

## Routes And Page States

Required user-facing routes:

- `GET /` shows the todo list with the default `all` filter
- `GET /?filter=all`
- `GET /?filter=active`
- `GET /?filter=completed`

Required page states:

- empty state with no todos
- populated state with one or more todos
- filtered-empty state when todos exist but none match the selected filter
- validation-error state on invalid create or edit

Mutation endpoints may vary by framework. Standard form posts, server actions, LiveView events, SignalR actions, or equivalent mechanisms are all acceptable as long as the user-visible behavior matches this spec.

## Data Model

Required logical fields:

| Field | Type | Required | Notes |
| --- | --- | --- | --- |
| `id` | unique identifier | yes | Server-generated. Exact type is tutorial-specific. |
| `title` | string | yes | Human-readable todo text. |
| `completed` | boolean | yes | `false` by default. |
| `created_at` | timestamp | yes | Used for stable ordering. |
| `updated_at` | timestamp | yes | Updated on edit and toggle. |

Persistence requirements:

- data must be stored server-side in a durable datastore
- data must survive a browser refresh
- data must survive an application restart in the normal local-development setup
- SQLite is a good default, but the exact datastore may vary by framework

## Validation Rules

Validation must be enforced server-side on both create and edit:

- `title` is required
- `title` is trimmed before validation and persistence
- `title` length after trimming must be between `1` and `120` characters
- whitespace-only titles are invalid
- duplicate titles are allowed

Invalid submissions must:

- not create or update a record
- preserve the user-visible validation error
- leave the rest of the list unchanged

## Behavior Rules

Canonical behavior rules:

- todos are displayed in ascending `created_at` order
- a newly created todo appears at the end of the list
- toggling completion updates only that todo’s completion state
- editing changes only the title of the targeted todo
- deleting removes only the targeted todo
- the selected filter controls which todos are visible
- if a todo stops matching the active filter after an update, it should disappear from the current filtered view immediately after the action completes

Examples:

- if the current filter is `active` and the user marks a todo completed, that todo should no longer appear in the visible list
- if the current filter is `completed` and the user marks a todo active, that todo should no longer appear in the visible list

UI details may vary, but the canonical behavior may not.

## Acceptance Criteria

The app is considered spec-complete when all of the following are true:

1. A user can create a todo with a valid title.
2. A blank or whitespace-only title is rejected.
3. A title longer than `120` characters is rejected.
4. A created todo remains visible after a full page reload.
5. A created todo remains visible after restarting the app in the normal local setup.
6. A user can edit an existing todo title with a valid value.
7. An invalid edit is rejected without corrupting existing data.
8. A user can toggle a todo between active and completed.
9. The `all`, `active`, and `completed` filters each show the correct subset.
10. A user can delete a todo.
11. Todo titles are safely escaped when rendered in HTML.
12. All state-changing requests are protected by the framework’s normal anti-forgery mechanism or an equivalent server-side safeguard.

## Benchmark Contract

This app should later be benchmarked in a repeatable, framework-neutral way.

Each tutorial build must eventually support:

- a repeatable seed or reset mechanism for local benchmarking
- a stable dataset shape for benchmark runs
- the canonical routes and user-visible behaviors defined above

Recommended benchmark dataset:

- `100` total todos
- `50` active todos
- `50` completed todos
- titles with realistic short text lengths

Recommended benchmark scenarios:

- first render of `GET /`
- first render of `GET /?filter=active`
- create one valid todo
- toggle one active todo to completed
- edit one todo title
- delete one todo

The seed or reset mechanism may be implemented in a framework-specific way, but it must not change the canonical app behavior and must be disabled or protected outside benchmark or test workflows.

## Security Checklist

Every tutorial should pass this baseline checklist:

- escape todo titles in rendered HTML to prevent XSS
- validate all create and edit input on the server
- use parameterized queries, ORM protections, or equivalent safe persistence patterns
- protect state-changing requests against CSRF or the framework-equivalent browser attack surface
- return user-friendly validation errors without leaking stack traces to end users
- avoid exposing benchmark, seed, or reset helpers in normal production mode
- avoid embedding secrets or environment credentials in rendered output

## Parity Notes

To keep comparisons fair:

- styling is not part of the spec
- custom JavaScript is optional, not required
- websocket-based or reactive tutorial builds are allowed
- traditional form-post tutorial builds are allowed
- the visible behavior, validation, routes, and persistence guarantees must remain equivalent
- extra features are allowed only if they are clearly documented and disabled in the canonical comparison mode
