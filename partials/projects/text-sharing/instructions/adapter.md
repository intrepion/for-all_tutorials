---
partial_kind: adapter-instructions
project: text-sharing
---

# Adapter Instructions

Project-specific adapter instruction fragment for the `text-sharing` adapter repo.

### 1. Red: Add Adapter-Level Tests

In a separate adapter repo, add failing tests that prove the chosen full-stack web adapter:

- accepts one text snippet from a text area
- generates a unique snippet ID
- delegates snippet creation, path formatting, view formatting, and edit-seed behavior to the core logic
- persists snippet records
- exposes the share URL
- loads and displays the saved snippet from the share route
- pre-fills the create form with existing text on the edit route

### 2. Green: Add The Thin Full-Stack Adapter

Add the thinnest possible full-stack web adapter:

- render a form with a text area for entering one text snippet
- generate a unique snippet ID for a new record
- pass the snippet ID and text to `build_text_snippet_record`
- persist the resulting record in a permanent data store
- use `format_text_snippet_path` when showing or linking to the share URL
- when `/<snippet_id>` is visited:
  - load the matching record
  - pass it to `format_text_snippet_view`
  - render the returned content and an invitation to edit
- when `/<snippet_id>/edit` is visited:
  - load the matching record
  - pass it to `build_text_snippet_edit_seed`
  - pre-fill the same create form with the returned text
- keep routing, persistence, and form handling code out of the core snippet logic

### 3. Refactor

Clean up any remaining duplication while keeping the full suite green.

