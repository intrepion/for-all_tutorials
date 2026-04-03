<!-- breadcrumbs:start -->
[for-all_tutorials](../../../README.md) / [Projects](../../README.md) / [Text Sharing](../README.md) / Spec
<!-- breadcrumbs:end -->

# Spec

Canonical project contract for the `text-sharing` project.

## Goal

Build a small project app that introduces:

- accepting one text snippet from a web form
- building a snippet record from an adapter-chosen snippet ID
- generating a local share path for the saved snippet
- generating a local edit path for that snippet
- loading and displaying a saved snippet by URL
- pre-filling the new-snippet interface with existing text for editing
- test-first snippet and path logic in small core functions
- thin full-stack web adapters

## Canonical Sample Snippet Record

For deterministic tests and examples, use a snippet record equivalent to:

```text
snippet_id = "abc1234"
snippet_text = "Learn how to invert binary trees"
```

## Core Logic Contract

The shared core contracts are:

```text
build_text_snippet_record(snippet_id: string, snippet_text: string) -> text_snippet_record
format_text_snippet_path(snippet_id: string) -> string
format_text_snippet_edit_path(snippet_id: string) -> string
format_text_snippet_view(text_snippet_record) -> string[]
build_text_snippet_edit_seed(text_snippet_record) -> string
```

Where `text_snippet_record` contains:

```text
snippet_id
snippet_text
```

Canonical behavior:

- `build_text_snippet_record`:
  - preserves the exact `snippet_id`
  - preserves the exact `snippet_text`
- `format_text_snippet_path` returns:
  - `/<snippet_id>`
- `format_text_snippet_edit_path` returns:
  - `/<snippet_id>/edit`
- `format_text_snippet_view` returns these lines in this exact order:
  - `Text: <snippet_text>`
  - `Edit URL: <format_text_snippet_edit_path(snippet_id)>`
- `build_text_snippet_edit_seed`:
  - returns the exact `snippet_text`

Examples:

- `build_text_snippet_record("abc1234", "Learn how to invert binary trees")` returns:
  - `{ snippet_id: "abc1234", snippet_text: "Learn how to invert binary trees" }`
- `format_text_snippet_path("abc1234")` returns:
  - `/abc1234`
- `format_text_snippet_edit_path("abc1234")` returns:
  - `/abc1234/edit`
- `format_text_snippet_view({ snippet_id: "abc1234", snippet_text: "Learn how to invert binary trees" })` returns:
  - `Text: Learn how to invert binary trees`
  - `Edit URL: /abc1234/edit`
- `build_text_snippet_edit_seed({ snippet_id: "abc1234", snippet_text: "Learn how to invert binary trees" })` returns:
  - `Learn how to invert binary trees`

## Non-Goals

This project does not include:

- user accounts
- authentication
- snippet deletion
- snippet search
- syntax highlighting
- snippet version history
- in-place mutation of an existing saved snippet
- access counters or analytics

## Surface Expectations

This spec follows the shared surface and setup-path rules in [Projects](../../README.md#shared-spec-conventions).

For this project, every tutorial run should use a full-stack web adapter and adapt the same snippet, share-path, edit-path, view, and edit-seed behavior instead of redefining it.

Adapters should:

- use a web full-stack surface
- render a form with a text area for entering one text snippet
- generate a unique snippet ID for a new record
- pass the snippet ID and entered text to `build_text_snippet_record`
- persist the resulting record in a permanent data store
- expose the share URL using `format_text_snippet_path`
- when `/<snippet_id>` is visited:
  - load the matching snippet record from storage
  - pass it to `format_text_snippet_view`
  - render the returned content and an invitation to edit
- when `/<snippet_id>/edit` is visited:
  - load the matching snippet record from storage
  - pass it to `build_text_snippet_edit_seed`
  - pre-fill the same create-snippet interface with the returned text

Saving from the edit form may create a new snippet record with a new snippet ID rather than mutating the original saved record.

## Output Repository Expectations

This project follows the shared output model in [Projects](../../README.md#shared-output-model).

For this project, the core repo owns `build_text_snippet_record`, `format_text_snippet_path`, `format_text_snippet_edit_path`, `format_text_snippet_view`, and `build_text_snippet_edit_seed`, and adapter repos must not reimplement them.

## Testing And Coverage Contract

This spec follows the shared coverage policy in [Projects](../../README.md#shared-coverage-policy).

This project should be built in a spec-driven and test-driven way.

Minimum test expectations:

- a test that `build_text_snippet_record` preserves the exact snippet ID and text
- a test that `format_text_snippet_path("abc1234")` returns `/abc1234`
- a test that `format_text_snippet_edit_path("abc1234")` returns `/abc1234/edit`
- a test that `format_text_snippet_view` preserves the exact canonical two-line output
- a test that `build_text_snippet_edit_seed` returns the exact canonical snippet text
- tests for every adapter built in the chosen run that prove it accepts text from the form, persists the snippet record, exposes the share URL, loads and displays the snippet, and pre-fills the create form with existing text on the edit route
