---
partial_kind: project-spec
project: tracking-inventory
---

# Spec

Canonical project contract for the `tracking-inventory` project.

## Goal

Build a small project app that introduces:

- collecting inventory records with name, serial number, and estimated value
- validating that the estimated value is numeric
- persisting inventory data in one structured local data file
- parsing inventory records from structured stored data
- rendering the same inventory records as both HTML and CSV
- test-first validation, parsing, and formatting logic in small core functions
- thin surface adapters

## Canonical Sample Inventory Records

For deterministic tests and examples, use inventory records equivalent to:

```text
{ name: "Xbox One", serial_number: "AXB124AXY", value_cents: 39900 }
{ name: "Samsung TV", serial_number: "S40AZBDE4", value_cents: 59999 }
```

## Core Logic Contract

The shared core contracts are:

```text
validate_inventory_value(value_text: string) -> error | null
build_inventory_record(name: string, serial_number: string, value_text: string) -> inventory_record
parse_inventory_storage(storage_text: string) -> inventory_record[]
format_inventory_html_report(inventory_records: inventory_record[]) -> string[]
format_inventory_csv_report(inventory_records: inventory_record[]) -> string[]
```

Where each `inventory_record` contains:

```text
name
serial_number
value_cents
```

Canonical behavior:

- `validate_inventory_value` returns:
  - `The value must be numeric.` when `value_text` is empty or contains any non-digit character other than one decimal point
  - `null` otherwise
- `build_inventory_record`:
  - preserves the exact `name`
  - preserves the exact `serial_number`
  - parses `value_text` into whole-number `value_cents`
  - supports whole-dollar or two-decimal-place values such as `399` and `599.99`
- `parse_inventory_storage`:
  - parses one structured local data document containing the inventory records
  - preserves source order
  - preserves the exact `name` and `serial_number` text of each record
  - returns whole-number `value_cents`
- `format_inventory_html_report` returns these exact lines in this exact order for the canonical records:
  - `<table>`
  - `  <thead>`
  - `    <tr><th>Name</th><th>Serial Number</th><th>Value</th></tr>`
  - `  </thead>`
  - `  <tbody>`
  - `    <tr><td>Xbox One</td><td>AXB124AXY</td><td>$399.00</td></tr>`
  - `    <tr><td>Samsung TV</td><td>S40AZBDE4</td><td>$599.99</td></tr>`
  - `  </tbody>`
  - `</table>`
- `format_inventory_csv_report` returns these exact lines in this exact order for the canonical records:
  - `Name,Serial Number,Value`
  - `Xbox One,AXB124AXY,$399.00`
  - `Samsung TV,S40AZBDE4,$599.99`

Examples:

- `validate_inventory_value("599.99")` returns `null`
- `validate_inventory_value("abc")` returns `The value must be numeric.`
- `build_inventory_record("Xbox One", "AXB124AXY", "399.00")` returns:
  - `{ name: "Xbox One", serial_number: "AXB124AXY", value_cents: 39900 }`
- `format_inventory_csv_report(canonical_inventory_records)` returns:
  - `Name,Serial Number,Value`
  - `Xbox One,AXB124AXY,$399.00`
  - `Samsung TV,S40AZBDE4,$599.99`

## Non-Goals

This project does not include:

- network access
- multiple storage files
- inventory search
- inventory deletion
- inventory sorting
- barcode scanning
- localization
- spreadsheet import or export beyond the canonical CSV report

## Surface Expectations

This spec follows the shared surface and setup-path rules in [Projects](../../README.md#shared-spec-conventions).

For this project, every tutorial run should adapt the same validation, record-building, parsing, and report-formatting behavior instead of redefining it.

Adapters should:

- collect `name`, `serial number`, and `estimated value` for each item
- validate the entered value with `validate_inventory_value`
- reject non-numeric value input
- build each accepted record with `build_inventory_record`
- persist the inventory records in one local structured data file using JSON, XML, or YAML
- read the same structured file back through `parse_inventory_storage`
- render both the HTML report and the CSV report from the parsed inventory records

## Output Repository Expectations

This project follows the shared output model in [Projects](../../README.md#shared-output-model).

For this project, the core repo owns `validate_inventory_value`, `build_inventory_record`, `parse_inventory_storage`, `format_inventory_html_report`, and `format_inventory_csv_report`, and adapter repos must not reimplement them.

## Testing And Coverage Contract

This spec follows the shared coverage policy in [Projects](../../README.md#shared-coverage-policy).

This project should be built in a spec-driven and test-driven way.

Minimum test expectations:

- a test that `validate_inventory_value` accepts canonical numeric values and rejects non-numeric values
- a test that `build_inventory_record` preserves the exact canonical name and serial number while parsing cents correctly
- a test that `parse_inventory_storage` preserves the canonical record order and parsed values
- a test that `format_inventory_html_report` preserves the exact canonical HTML table lines
- a test that `format_inventory_csv_report` preserves the exact canonical CSV lines
- tests for every adapter built in the chosen run that prove it persists the inventory in one local structured data file, validates value input, parses the stored records through the core, and renders both export formats correctly

