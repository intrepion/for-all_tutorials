---
partial_kind: project-spec
project: name-sorter
---

# Spec

Canonical project contract for the `name-sorter` project.

## Goal

Build a small project app that introduces:

- reading a fixed canonical list of `Last, First` names from a file
- parsing those lines into structured name records
- sorting parsed name records alphabetically by last name and then first name
- formatting the sorted names as canonical output-file contents
- test-first parsing, sorting, and formatting logic in small core functions
- thin surface adapters

## Canonical Input File Contents

Every tutorial run for this project should use an input file whose contents are these exact lines in this exact order:

- `Ling, Mai`
- `Johnson, Jim`
- `Zarnecki, Sabrina`
- `Jones, Chris`
- `Jones, Aaron`
- `Swift, Geoffrey`
- `Xiong, Fong`

## Core Logic Contract

The shared core contracts are:

```text
parse_name_entries(lines: string[]) -> name_entries
sort_name_entries(name_entries) -> name_entries
format_sorted_name_report(name_entries) -> string[]
```

Where each parsed entry contains:

```text
last_name
first_name
```

Canonical behavior:

- `parse_name_entries`:
  - returns one parsed entry per input line
  - treats each line as being in the exact format `<last_name>, <first_name>`
  - preserves the original input order
  - preserves the exact text of `last_name` and `first_name`
- `sort_name_entries`:
  - returns a new list
  - sorts entries in ascending order by exact `last_name`
  - breaks ties by exact `first_name`
  - preserves stable relative order when two parsed entries have the same `last_name` and `first_name`
  - does not trim, normalize, or case-fold names before sorting
- `format_sorted_name_report` returns these lines in this exact order:
  - `Total of 7 names`
  - `-----------------`
  - one line per name entry in this exact format:
    - `<last_name>, <first_name>`

Examples:

- `parse_name_entries(["Ling, Mai", "Johnson, Jim"])` returns:
  - `{ last_name: "Ling", first_name: "Mai" }`
  - `{ last_name: "Johnson", first_name: "Jim" }`
- `sort_name_entries(parse_name_entries(canonical_input_lines))` returns entries in this order:
  - `Johnson, Jim`
  - `Jones, Aaron`
  - `Jones, Chris`
  - `Ling, Mai`
  - `Swift, Geoffrey`
  - `Xiong, Fong`
  - `Zarnecki, Sabrina`
- `format_sorted_name_report(sort_name_entries(parse_name_entries(canonical_input_lines)))` returns:
  - `Total of 7 names`
  - `-----------------`
  - `Johnson, Jim`
  - `Jones, Aaron`
  - `Jones, Chris`
  - `Ling, Mai`
  - `Swift, Geoffrey`
  - `Xiong, Fong`
  - `Zarnecki, Sabrina`

## Non-Goals

This project does not include:

- persistence beyond the one source file and one output file handled by the adapter
- localization
- authentication
- network access
- configuration files
- user-supplied name formats other than `<last_name>, <first_name>`
- filtering names before sorting
- deduplicating names
- descending sorting
- multi-column output beyond the canonical count line, separator line, and sorted names

## Surface Expectations

This spec follows the shared surface and setup-path rules in [Projects](../../README.md#shared-spec-conventions).

For this project, every tutorial run should adapt the same canonical input lines, parsing rule, sort rule, and output-file formatting behavior instead of redefining them.

Adapters should:

- read one source file whose contents match the canonical input lines
- split the source file into lines
- pass those lines to `parse_name_entries`
- pass the parsed entries to `sort_name_entries`
- pass the sorted entries to `format_sorted_name_report`
- write the returned lines to an output file in order

## Output Repository Expectations

This project follows the shared output model in [Projects](../../README.md#shared-output-model).

For this project, the core repo owns `parse_name_entries`, `sort_name_entries`, and `format_sorted_name_report`, and adapter repos must not reimplement them.

## Testing And Coverage Contract

This spec follows the shared coverage policy in [Projects](../../README.md#shared-coverage-policy).

This project should be built in a spec-driven and test-driven way.

Minimum test expectations:

- a test that `parse_name_entries` preserves the exact canonical input order and parsed values
- a test that `sort_name_entries` returns the canonical alphabetical order by last name and then first name
- a test that `sort_name_entries` returns a new list rather than mutating the original input list
- a test that `format_sorted_name_report` preserves the exact count line and separator line
- a test that `format_sorted_name_report` preserves the exact canonical sorted output lines
- tests for every adapter built in the chosen run that prove it reads the source file, delegates parsing, sorting, and formatting to the core logic, and writes the canonical output-file contents correctly

