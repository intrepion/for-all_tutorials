---
partial_kind: core-instructions
project: tracking-inventory
---

# Core Instructions

Project-specific core instruction fragment for the `tracking-inventory` core repo.

### 1. Red: Write The First Failing Test

Start with value validation:

```text
given a numeric value string
when validate_inventory_value is called
then null is returned
```

and

```text
given a non-numeric value string
when validate_inventory_value is called
then The value must be numeric. is returned
```

At this point, `validate_inventory_value` may not exist yet. That is fine. The first run should fail either at compile time or at test execution time.

Suggested generic test names:

```text
validate_inventory_value_accepts_numeric_values
validate_inventory_value_rejects_non_numeric_values
```

### 2. Green: Make The First Tests Pass

Create the smallest production code that makes the first tests pass.

At this stage, it is acceptable if the code only handles validation. Do not jump ahead and implement record building or export formatting before those tests exist.

### 3. Refactor

Do the smallest cleanup that keeps the first tests green.

### 4. Red: Add Record Building And Storage Parsing

Write the next failing tests:

```text
given a canonical name, serial number, and value text
when build_inventory_record is called
then the returned record preserves the exact text fields and canonical cents value
```

and

```text
given the canonical stored inventory data
when parse_inventory_storage is called
then the returned records preserve the canonical order and values
```

Suggested generic test names:

```text
build_inventory_record_parses_the_canonical_value_into_cents
parse_inventory_storage_preserves_the_canonical_records_in_order
```

### 5. Green: Make The Record Tests Pass

Make the smallest safe change that gets those tests green.

### 6. Red: Add HTML And CSV Formatting

Write the next failing tests:

```text
given the canonical inventory records
when format_inventory_html_report is called
then the returned lines are the exact canonical HTML table
```

and

```text
given the canonical inventory records
when format_inventory_csv_report is called
then the returned lines are the exact canonical CSV output
```

Suggested generic test names:

```text
format_inventory_html_report_returns_the_canonical_html_table
format_inventory_csv_report_returns_the_canonical_csv_lines
```

### 7. Green: Make The Export Tests Pass

Make the smallest safe change that gets the full suite green.

### 8. Refactor The Core Logic

Now that the full behavior is covered, refactor toward a small, clear core abstraction.

The exact shape can vary by stack:

- five functions in one module
- one service object with validate, build, parse, and export methods
- one module plus a small immutable inventory-record type

The important thing is that validation, record parsing, and export formatting stay small, explicit, and directly testable.

