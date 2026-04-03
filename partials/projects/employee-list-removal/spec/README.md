---
partial_kind: project-spec
project: employee-list-removal
---

# Spec

Canonical project contract for the `employee-list-removal` project.

## Goal

Build a small project app that introduces:

- working with a fixed canonical list of employee names
- filtering one exact employee name out of that list
- returning a new list rather than mutating the existing one
- separating canonical-list construction, removal, and output formatting
- test-first list and formatting logic in small core functions
- thin surface adapters

## Core Logic Contract

The shared core contracts are:

```text
default_employee_names() -> string[]
remove_employee_by_exact_name(employee_names: string[], employee_name_to_remove: string) -> string[]
format_employee_list(employee_names: string[]) -> string[]
```

Canonical behavior:

- `default_employee_names` returns this exact list in this exact order:
  - `John Smith`
  - `Jackie Jackson`
  - `Chris Jones`
  - `Amanda Cullen`
  - `Jeremy Goodwin`
- `remove_employee_by_exact_name`:
  - compares names by exact string match
  - does not trim, normalize, or case-fold `employee_name_to_remove`
  - returns a new list
  - preserves the original relative order of the remaining names
  - removes the exact matching employee name when it is present
  - returns the same names in the same order when no exact match is present
- `format_employee_list` returns:
  - a first line in the exact format `There are <count> employees:`
  - one additional line per employee name, in order

Examples:

- `default_employee_names()` returns:
  - `John Smith`
  - `Jackie Jackson`
  - `Chris Jones`
  - `Amanda Cullen`
  - `Jeremy Goodwin`
- `remove_employee_by_exact_name(default_employee_names(), "Chris Jones")` returns:
  - `John Smith`
  - `Jackie Jackson`
  - `Amanda Cullen`
  - `Jeremy Goodwin`
- `remove_employee_by_exact_name(default_employee_names(), "Chris")` returns the unchanged five-name list
- `format_employee_list(["John Smith", "Jackie Jackson", "Amanda Cullen", "Jeremy Goodwin"])` returns:
  - `There are 4 employees:`
  - `John Smith`
  - `Jackie Jackson`
  - `Amanda Cullen`
  - `Jeremy Goodwin`

## Non-Goals

This project does not include:

- persistence
- localization
- authentication
- network access
- configuration files
- partial-name matching
- case-insensitive matching
- duplicate employee names in the canonical list
- editing or adding employee names
- in-place mutation requirements

## Surface Expectations

This spec follows the shared surface and setup-path rules in [Projects](../../README.md#shared-spec-conventions).

For this project, every tutorial run should adapt the same canonical list, exact-match removal rule, and list-formatting behavior instead of redefining it.

Adapters should:

- obtain the canonical employee list from `default_employee_names`
- render the starting list by passing that list to `format_employee_list`
- prompt for one employee name to remove
- pass the canonical list and the entered name to `remove_employee_by_exact_name`
- pass the returned list to `format_employee_list`
- render the resulting list in the form that surface requires

For prompt-driven adapters, the removal prompt should be equivalent to:

```text
Enter an employee name to remove:
```

## Output Repository Expectations

This project follows the shared output model in [Projects](../../README.md#shared-output-model).

For this project, the core repo owns `default_employee_names`, `remove_employee_by_exact_name`, and `format_employee_list`, and adapter repos must not reimplement them.

## Testing And Coverage Contract

This spec follows the shared coverage policy in [Projects](../../README.md#shared-coverage-policy).

This project should be built in a spec-driven and test-driven way.

Minimum test expectations:

- a test that `default_employee_names` returns the exact five canonical names in order
- a test that `remove_employee_by_exact_name` removes `Chris Jones` from the canonical list
- a test that `remove_employee_by_exact_name` preserves the unchanged list for a non-matching name such as `Chris`
- a test that `remove_employee_by_exact_name` returns a new list rather than mutating the original input list
- a test that `format_employee_list` preserves the exact header format and per-name ordering
- tests for every adapter built in the chosen run that prove it renders the starting list, prompts once for the removal name, and delegates to the core logic correctly

