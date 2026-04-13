---
partial_kind: adapter-instructions
project: text-sharing
---

# Adapter Instructions

Project-specific adapter instruction fragment for the `text-sharing` adapter repo.

### 1. Red: Add Adapter-Level Tests

In a separate adapter repo, add failing tests that prove the chosen adapter:

- accepts one text snippet through the chosen surface
- generates a unique snippet ID
- delegates snippet creation, path formatting, view formatting, and edit-seed behavior to the core logic
- persists snippet records
- exposes the share path
- loads and displays the saved snippet from the share interaction
- pre-fills the chosen edit experience with existing text on the edit interaction

### 2. Green: Add The Thin Adapter

Add the thinnest possible adapter:

- collect one text snippet through the chosen surface
- generate a unique snippet ID for a new record
- pass the snippet ID and text to `build_text_snippet_record`
- persist the resulting record in a permanent data store
- use `format_text_snippet_path` when showing, linking to, or returning the share path
- when `/<snippet_id>` is resolved through the chosen surface:
  - load the matching record
  - pass it to `format_text_snippet_view`
  - render or return the returned content and an invitation to edit
- when `/<snippet_id>/edit` is resolved through the chosen surface:
  - load the matching record
  - pass it to `build_text_snippet_edit_seed`
  - pre-fill the chosen edit experience with the returned text
- keep routing, persistence, and surface interaction code out of the core snippet logic

### 3. Refactor

Clean up any remaining duplication while keeping the full suite green.
