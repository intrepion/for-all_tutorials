---
partial_kind: project-spec
project: pushing-notes-to-firebase
---

# Spec

Canonical project contract for the `pushing-notes-to-firebase` project.

## Goal

Build a small project app that introduces:

- handling two command-line commands
- saving a note through the Firebase Realtime Database REST API
- reading a collection of saved notes from Firebase
- parsing keyed JSON note records into structured note values
- ordering notes by descending calendar date
- formatting a notes report
- test-first parsing, ordering, and formatting logic in small core functions
- thin command-line adapters

## Canonical Sample API Response

For deterministic tests and examples, use a response equivalent to this JSON document:

```json
{
  "-Nk1": {
    "created_on": "2050-12-31",
    "text": "Learn how to invert binary trees"
  },
  "-Nk2": {
    "created_on": "2050-12-30",
    "text": "Notetaking on the command line is cool."
  }
}
```

## Core Logic Contract

The shared core contracts are:

```text
build_note_record(note_text: string, created_on: string) -> note_record
parse_notes_collection_response(json_text: string) -> note_record[]
format_note_saved_message() -> string
format_notes_report(note_record[]) -> string[]
```

Where each `note_record` contains:

```text
created_on
text
```

Canonical behavior:

- `build_note_record`:
  - preserves the exact `note_text`
  - preserves the exact `created_on`
  - returns a note record with those two fields
- `parse_notes_collection_response`:
  - parses the keyed JSON object
  - reads each `created_on` value into `created_on`
  - reads each `text` value into `text`
  - ignores the Firebase-generated keys after successful parsing
  - returns note records ordered by descending `created_on`
- `format_note_saved_message` returns:
  - `Your note was saved.`
- `format_notes_report` returns one line per note in this exact format:
  - `<created_on> - <text>`
  - preserving the input order of the provided `note_record[]`

Examples:

- `build_note_record("Learn how to invert binary trees", "2050-12-31")` returns:
  - `{ created_on: "2050-12-31", text: "Learn how to invert binary trees" }`
- `parse_notes_collection_response(canonical_json_text)` returns note records in this order:
  - `{ created_on: "2050-12-31", text: "Learn how to invert binary trees" }`
  - `{ created_on: "2050-12-30", text: "Notetaking on the command line is cool." }`
- `format_note_saved_message()` returns `Your note was saved.`
- `format_notes_report(parse_notes_collection_response(canonical_json_text))` returns:
  - `2050-12-31 - Learn how to invert binary trees`
  - `2050-12-30 - Notetaking on the command line is cool.`

## Non-Goals

This project does not include:

- a graphical interface
- editing or deleting notes
- tagging or searching notes
- offline storage
- client-library SDK usage
- conflict resolution
- realtime streaming updates
- custom date formatting

## Surface Expectations

This spec follows the shared surface and setup-path rules in [Projects](../../README.md#shared-spec-conventions).

For this project, every tutorial run should use a command-line adapter and adapt the same note-record, parsing, ordering, and formatting behavior instead of redefining it.

Adapters should:

- use a command-line surface
- support these commands:
  - `mynotes new <note text>`
  - `mynotes show`
- for `mynotes new <note text>`:
  - determine the current calendar date in `YYYY-MM-DD` format
  - pass the note text and date to `build_note_record`
  - write the resulting note record to Firebase Realtime Database using its REST API
  - use HTTPS Firebase Realtime Database endpoints ending in `.json`
  - render `format_note_saved_message()`
- for `mynotes show`:
  - read the notes collection from Firebase Realtime Database using its REST API
  - pass the response body to `parse_notes_collection_response`
  - pass the parsed notes to `format_notes_report`
  - render the returned lines in order
- use direct REST requests instead of a premade Firebase client library

## Output Repository Expectations

This project follows the shared output model in [Projects](../../README.md#shared-output-model).

For this project, the core repo owns `build_note_record`, `parse_notes_collection_response`, `format_note_saved_message`, and `format_notes_report`, and adapter repos must not reimplement them.

## Testing And Coverage Contract

This spec follows the shared coverage policy in [Projects](../../README.md#shared-coverage-policy).

This project should be built in a spec-driven and test-driven way.

Minimum test expectations:

- a test that `build_note_record` preserves the exact note text and date
- a test that `parse_notes_collection_response` preserves the exact canonical note texts and dates
- a test that `parse_notes_collection_response` returns the canonical notes in descending date order
- a test that `format_note_saved_message` preserves the exact canonical saved message
- a test that `format_notes_report` preserves the exact canonical report lines
- tests for every adapter built in the chosen run that prove it handles the `new` and `show` commands, uses direct Firebase REST requests, delegates note building/parsing/formatting to the core logic, and renders the canonical saved message and notes report correctly

