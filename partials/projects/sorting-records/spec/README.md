---
partial_kind: project-spec
project: sorting-records
---

# Spec

Canonical project contract for the `sorting-records` project.

## Goal

Build a small project app that introduces:

- working with a fixed canonical list of employee records
- sorting those records by last name
- preserving stable order when two records have the same last name
- separating canonical-record construction, sorting, and table formatting
- test-first record and formatting logic in small core functions
- thin surface adapters

## Core Logic Contract

The shared core contracts are:

```text
default_employee_records() -> employee_records
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

- the first record in `default_employee_records()` is `John Johnson / Manager / 2016-12-31`
- `sort_employee_records_by_last_name(default_employee_records())` returns records in this order:
  - `Jacquelyn Jackson`
  - `Jake Jacobson`
  - `John Johnson`
  - `Michaela Michaelson`
  - `Sally Weber`
  - `Tou Xiong`
- `format_employee_record_table(sort_employee_records_by_last_name(default_employee_records()))` returns:
  - `Name | Position | Separation Date`
  - `--------------------|-------------------|----------------`
  - `Jacquelyn Jackson | DBA | `
  - `Jake Jacobson | Programmer | `
  - `John Johnson | Manager | 2016-12-31`
  - `Michaela Michaelson | District Manager | 2015-12-19`
  - `Sally Weber | Web Developer | 2015-12-18`
  - `Tou Xiong | Software Engineer | 2016-10-05`

## Non-Goals

This project does not include:

- persistence
- localization
- authentication
- network access
- configuration files
- user-supplied employee records
- descending or multi-column sorting
- filtering records before sorting
- column-width auto-sizing beyond the canonical fixed header and separator lines
- date parsing beyond preserving the canonical text values

## Surface Expectations

This spec follows the shared surface and setup-path rules in [Projects](../../README.md#shared-spec-conventions).

For this project, every tutorial run should adapt the same canonical dataset, last-name sort, and table-formatting behavior instead of redefining it.

Adapters should:

- obtain the canonical employee records from `default_employee_records`
- pass them to `sort_employee_records_by_last_name`
- pass the sorted records to `format_employee_record_table`
- render the returned lines in order

## Output Repository Expectations

This project follows the shared output model in [Projects](../../README.md#shared-output-model).

For this project, the core repo owns `default_employee_records`, `sort_employee_records_by_last_name`, and `format_employee_record_table`, and adapter repos must not reimplement them.

## Testing And Coverage Contract

This spec follows the shared coverage policy in [Projects](../../README.md#shared-coverage-policy).

This project should be built in a spec-driven and test-driven way.

Minimum test expectations:

- a test that `default_employee_records` returns the exact six canonical records in order
- a test that `sort_employee_records_by_last_name` returns the canonical last-name order
- a test that `sort_employee_records_by_last_name` returns a new list rather than mutating the original input list
- a test that `format_employee_record_table` preserves the exact header and separator lines
- a test that `format_employee_record_table` preserves the exact canonical row text, including blank separation-date fields
- tests for every adapter built in the chosen run that prove it delegates to the core logic correctly

