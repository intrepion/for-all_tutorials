---
partial_kind: adapter-instructions
project: pushing-notes-to-firebase
---

# Adapter Instructions

Project-specific adapter instruction fragment for the `pushing-notes-to-firebase` adapter repo.

### 1. Red: Add Adapter-Level Tests

In a separate adapter repo, add failing tests that prove the chosen adapter:

- handles note-creation and note-listing interactions equivalent to `mynotes new <note text>` and `mynotes show`
- uses direct Firebase REST requests instead of a premade client library
- delegates note building, parsing, and formatting to the core logic
- renders the canonical saved message and notes report correctly

### 2. Green: Add The Thin Adapter

Add the thinnest possible adapter for the chosen surface:

- handle a note-creation interaction equivalent to `mynotes new <note text>`
- determine the current calendar date in `YYYY-MM-DD` format
- pass the note text and date to `build_note_record`
- write the resulting record to a Firebase Realtime Database HTTPS endpoint ending in `.json`
- render or return `format_note_saved_message()`
- handle a note-listing interaction equivalent to `mynotes show`
- read the notes collection from a Firebase Realtime Database HTTPS endpoint ending in `.json`
- pass the response body to `parse_notes_collection_response`
- pass the parsed notes to `format_notes_report`
- render or return the returned lines in the form the chosen surface requires
- keep surface interaction, clock access, and HTTP transport code out of the core note logic

### 3. Refactor

Clean up any remaining duplication while keeping the full suite green.
