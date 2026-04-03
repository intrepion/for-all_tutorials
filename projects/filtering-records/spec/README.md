<!-- breadcrumbs:start -->
[for-all_tutorials](../../../README.md) / [Projects](../../README.md) / [Filtering Records](../README.md) / Spec
<!-- breadcrumbs:end -->

# Spec

Canonical project contract for the `filtering-records` project.

## Goal

Build a small project app that introduces:

- working with a fixed canonical list of employee records
- filtering those records by a search string against first and last name fields
- preserving source order during filtering and then sorting the filtered records by last name
- separating canonical-record construction, filtering, sorting, and table formatting
- test-first record and formatting logic in small core functions
- thin surface adapters

## Core Logic Contract

The shared core contracts are:

```text
default_employee_records() -> employee_records
filter_employee_records_by_search_string(employee_records, search_string: string) -> employee_records
sort_employee_records_by_last_name(employee_records) -> employee_records
format_employee_record_table(employee_records) -> string[]
```

Where each record contains:

```text
first_name
last_name
position
separation_date
```

Canonical behavior:

- `default_employee_records` returns these exact records in this exact order:
  - `{ first_name: "John", last_name: "Johnson", position: "Manager", separation_date: "2016-12-31" }`
  - `{ first_name: "Tou", last_name: "Xiong", position: "Software Engineer", separation_date: "2016-10-05" }`
  - `{ first_name: "Michaela", last_name: "Michaelson", position: "District Manager", separation_date: "2015-12-19" }`
  - `{ first_name: "Jake", last_name: "Jacobson", position: "Programmer", separation_date: "" }`
  - `{ first_name: "Jacquelyn", last_name: "Jackson", position: "DBA", separation_date: "" }`
  - `{ first_name: "Sally", last_name: "Weber", position: "Web Developer", separation_date: "2015-12-18" }`
- `filter_employee_records_by_search_string`:
  - returns a new list
  - performs a case-insensitive substring comparison
  - compares `search_string` against `first_name` and `last_name`
  - keeps a record when the search string appears in either field
  - preserves the original relative order of all kept records
  - does not trim `search_string` before comparing
- `sort_employee_records_by_last_name`:
  - returns a new list
  - sorts records in ascending order by exact `last_name`
  - preserves stable relative order when two records share the same `last_name`
  - does not trim, normalize, or case-fold names before sorting
- `format_employee_record_table` returns these lines in this exact order:
  - `Name | Position | Separation Date`
  - `--------------------|-------------------|----------------`
  - one line per employee record in this exact format:
    - `<first_name> <last_name> | <position> | <separation_date>`
  - when `separation_date` is empty, the row still includes the trailing separator and empty final field

Examples:

- `filter_employee_records_by_search_string(default_employee_records(), "Jac")` returns records in this order:
  - `Jake Jacobson`
  - `Jacquelyn Jackson`
- `filter_employee_records_by_search_string(default_employee_records(), "jac")` returns the same two records in the same order
- `sort_employee_records_by_last_name(filter_employee_records_by_search_string(default_employee_records(), "Jac"))` returns records in this order:
  - `Jacquelyn Jackson`
  - `Jake Jacobson`
- `format_employee_record_table(sort_employee_records_by_last_name(filter_employee_records_by_search_string(default_employee_records(), "Jac")))` returns:
  - `Name | Position | Separation Date`
  - `--------------------|-------------------|----------------`
  - `Jacquelyn Jackson | DBA | `
  - `Jake Jacobson | Programmer | `

## Non-Goals

This project does not include:

- persistence
- localization
- authentication
- network access
- configuration files
- user-supplied employee records
- fuzzy matching beyond case-insensitive substring matching
- regular-expression searches
- descending or multi-column sorting
- filtering on `position` or `separation_date`
- column-width auto-sizing beyond the canonical fixed header and separator lines
- date parsing beyond preserving the canonical text values

## Surface Expectations

This spec follows the shared surface and setup-path rules in [Projects](../../README.md#shared-spec-conventions).

For this project, every tutorial run should adapt the same canonical dataset, search rule, last-name sort, and table-formatting behavior instead of redefining it.

Adapters should:

- obtain the canonical employee records from `default_employee_records`
- prompt for one search string
- pass the canonical dataset and the entered search string to `filter_employee_records_by_search_string`
- pass the filtered records to `sort_employee_records_by_last_name`
- render a results heading equivalent to `Results:`
- pass the sorted filtered records to `format_employee_record_table`
- render the returned lines in order

For prompt-driven adapters, the prompt should be equivalent to:

```text
Enter a search string:
```

## Output Repository Expectations

This project follows the shared output model in [Projects](../../README.md#shared-output-model).

For this project, the core repo owns `default_employee_records`, `filter_employee_records_by_search_string`, `sort_employee_records_by_last_name`, and `format_employee_record_table`, and adapter repos must not reimplement them.

## Testing And Coverage Contract

This spec follows the shared coverage policy in [Projects](../../README.md#shared-coverage-policy).

This project should be built in a spec-driven and test-driven way.

Minimum test expectations:

- a test that `default_employee_records` returns the exact six canonical records in order
- a test that `filter_employee_records_by_search_string(default_employee_records(), "Jac")` returns `Jake Jacobson` and `Jacquelyn Jackson` in source order
- a test that `filter_employee_records_by_search_string` matches case-insensitively for a value such as `jac`
- a test that `filter_employee_records_by_search_string` returns a new list rather than mutating the original input list
- a test that `sort_employee_records_by_last_name` returns the canonical filtered order for the `Jac` match set
- a test that `format_employee_record_table` preserves the exact header, separator, and canonical filtered row text, including blank separation-date fields
- tests for every adapter built in the chosen run that prove it prompts for the search string, renders the results heading, and delegates filtering, sorting, and formatting to the core logic correctly
