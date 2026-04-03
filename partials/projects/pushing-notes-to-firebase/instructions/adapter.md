---
partial_kind: adapter-instructions
project: pushing-notes-to-firebase
---

# Adapter Instructions

Project-specific adapter instruction fragment for the `pushing-notes-to-firebase` adapter repo.

### 1. Red: Add Adapter-Level Tests

In a separate adapter repo, add failing tests that prove the chosen command-line adapter:

- handles `mynotes new <note text>`
- handles `mynotes show`
- uses direct Firebase REST requests instead of a premade client library
- delegates note building, parsing, and formatting to the core logic
- renders the canonical saved message and notes report correctly

### 2. Green: Add The Thin Command-Line Adapter

Add the thinnest possible adapter for the command-line surface you are building:

- parse `mynotes new <note text>`
- determine the current calendar date in `YYYY-MM-DD` format
- pass the note text and date to `build_note_record`
- write the resulting record to a Firebase Realtime Database HTTPS endpoint ending in `.json`
- render `format_note_saved_message()`
- parse `mynotes show`
- read the notes collection from a Firebase Realtime Database HTTPS endpoint ending in `.json`
- pass the response body to `parse_notes_collection_response`
- pass the parsed notes to `format_notes_report`
- render the returned lines in order
- keep command parsing, clock access, and HTTP transport code out of the core note logic

### 3. Refactor

Clean up any remaining duplication while keeping the full suite green.

