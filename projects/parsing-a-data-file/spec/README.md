<!-- breadcrumbs:start -->
[for-all_tutorials](../../../README.md) / [Projects](../../README.md) / [Parsing a Data File](../README.md) / Spec
<!-- breadcrumbs:end -->

# Spec

Canonical project contract for the `parsing-a-data-file` project.

## Goal

Build a small project app that introduces:

- reading a fixed canonical CSV data file
- parsing CSV lines into structured records
- preserving input order while converting the salary field into a whole-number value
- formatting parsed records as a canonical table report
- test-first parsing and formatting logic in small core functions
- thin surface adapters

## Canonical Input File Contents

Every tutorial run for this project should use an input file whose contents are these exact lines in this exact order:

- `Ling,Mai,55900`
- `Johnson,Jim,56500`
- `Jones,Aaron,46000`
- `Jones,Chris,34500`
- `Swift,Geoffrey,14200`
- `Xiong,Fong,65000`
- `Zarnecki,Sabrina,51500`

## Core Logic Contract

The shared core contracts are:

```text
parse_employee_salary_records(lines: string[]) -> employee_salary_records
format_employee_salary_table(employee_salary_records) -> string[]
```

Where each parsed record contains:

```text
last_name
first_name
salary_dollars
```

Canonical behavior:

- `parse_employee_salary_records`:
  - returns one parsed record per input line
  - treats each line as being in the exact format `<last_name>,<first_name>,<salary>`
  - preserves the original input order
  - preserves the exact text of `last_name` and `first_name`
  - parses `salary` into a whole-number `salary_dollars` value
- `format_employee_salary_table` returns these lines in this exact order:
  - `Last     First    Salary`
  - `------------------------`
  - one line per employee salary record
  - each row uses:
    - a left-aligned `last_name` column of width `8`
    - one separating space
    - a left-aligned `first_name` column of width `8`
    - one separating space
    - a final `salary_dollars` field rendered with no commas, no currency symbol, and no trailing end-of-line padding

Examples:

- `parse_employee_salary_records(["Ling,Mai,55900", "Johnson,Jim,56500"])` returns:
  - `{ last_name: "Ling", first_name: "Mai", salary_dollars: 55900 }`
  - `{ last_name: "Johnson", first_name: "Jim", salary_dollars: 56500 }`
- `format_employee_salary_table(parse_employee_salary_records(canonical_input_lines))` returns:
  - `Last     First    Salary`
  - `------------------------`
  - `Ling     Mai      55900`
  - `Johnson  Jim      56500`
  - `Jones    Aaron    46000`
  - `Jones    Chris    34500`
  - `Swift    Geoffrey 14200`
  - `Xiong    Fong     65000`
  - `Zarnecki Sabrina  51500`

## Non-Goals

This project does not include:

- persistence beyond the one source file handled by the adapter
- localization
- authentication
- network access
- configuration files
- user-supplied column counts or alternate delimiters
- sorting records
- filtering records
- salary arithmetic
- currency formatting
- column auto-sizing beyond the canonical fixed-width table layout

## Surface Expectations

This spec follows the shared surface and setup-path rules in [Projects](../../README.md#shared-spec-conventions).

For this project, every tutorial run should adapt the same canonical input lines, CSV parsing rule, and table-formatting behavior instead of redefining them.

Adapters should:

- read one source file whose contents match the canonical input lines
- split the source file into lines
- pass those lines to `parse_employee_salary_records`
- pass the parsed records to `format_employee_salary_table`
- render the returned lines in order

## Output Repository Expectations

This project follows the shared output model in [Projects](../../README.md#shared-output-model).

For this project, the core repo owns `parse_employee_salary_records` and `format_employee_salary_table`, and adapter repos must not reimplement them.

## Testing And Coverage Contract

This spec follows the shared coverage policy in [Projects](../../README.md#shared-coverage-policy).

This project should be built in a spec-driven and test-driven way.

Minimum test expectations:

- a test that `parse_employee_salary_records` preserves the exact canonical input order and parsed values
- a test that `parse_employee_salary_records` converts salary text into the canonical whole-number `salary_dollars` values
- a test that `format_employee_salary_table` preserves the exact header line and separator line
- a test that `format_employee_salary_table` preserves the exact canonical table rows and spacing
- tests for every adapter built in the chosen run that prove it reads the source file, delegates parsing and formatting to the core logic, and renders the canonical table output correctly
