<!-- breadcrumbs:start -->
[for-all_tutorials](../../../README.md) / [Projects](../../README.md) / [Self-Checkout](../README.md) / Spec
<!-- breadcrumbs:end -->

# Spec

Canonical project contract for the `self-checkout` project.

## Goal

Build a small project app that introduces:

- converting three prompted item prices into exact money values
- converting three prompted quantities into numeric values
- calculating line-item totals, subtotal, tax, and grand total
- applying a fixed tax rate of `5.5%`
- rounding tax to the nearest cent with half-up rounding
- separating checkout calculation from output formatting
- test-first calculation and formatting logic in small core functions
- thin surface adapters

## Core Logic Contract

The shared core contracts are:

```text
calculate_checkout_summary(items: { unit_price_cents, quantity }[]) -> { line_items, subtotal_cents, tax_cents, total_cents }
format_checkout_report(summary) -> string[]
```

Canonical behavior:

- for this project, `items` contains exactly three entries in item order
- each item entry contains:
  - `unit_price_cents` as a positive whole number of cents
  - `quantity` as a positive whole number
- `calculate_checkout_summary` returns:
  - `line_items`, in the same order as the supplied items, where each line item contains:
    - `item_number`
    - `unit_price_cents`
    - `quantity`
    - `line_total_cents = unit_price_cents * quantity`
  - `subtotal_cents = sum of all line_total_cents`
  - `tax_cents = subtotal_cents * 5.5%`, rounded to the nearest cent with half-up rounding
  - `total_cents = subtotal_cents + tax_cents`
- `format_checkout_report` returns six lines in this exact order:
  - `Item 1: 2 x $25.00 = $50.00`
  - `Item 2: 1 x $10.00 = $10.00`
  - `Item 3: 1 x $4.00 = $4.00`
  - `Subtotal: $64.00`
  - `Tax: $3.52`
  - `Total: $67.52`
- currency values must always render with a leading `$` and exactly two decimal places
- the core logic must use exact cent-based values rather than floating-point money values

Examples:

- `calculate_checkout_summary([{ unit_price_cents: 2500, quantity: 2 }, { unit_price_cents: 1000, quantity: 1 }, { unit_price_cents: 400, quantity: 1 }])` returns line items with totals `5000`, `1000`, and `400`, plus `subtotal_cents: 6400`, `tax_cents: 352`, and `total_cents: 6752`
- `format_checkout_report(...)` for that summary returns:
  - `Item 1: 2 x $25.00 = $50.00`
  - `Item 2: 1 x $10.00 = $10.00`
  - `Item 3: 1 x $4.00 = $4.00`
  - `Subtotal: $64.00`
  - `Tax: $3.52`
  - `Total: $67.52`
- `calculate_checkout_summary([{ unit_price_cents: 25, quantity: 1 }, { unit_price_cents: 25, quantity: 1 }, { unit_price_cents: 50, quantity: 1 }])` returns `subtotal_cents: 100`, `tax_cents: 6`, and `total_cents: 106`

## Non-Goals

This project does not include:

- persistence
- localization
- authentication
- network access
- configuration files
- validation for zero or negative prices or quantities
- discounts, coupons, or promotions
- more or fewer than three items
- multiple tax rates or tax jurisdictions
- alternate currencies or currency symbols
- floating-point money calculations in the core logic

## Surface Expectations

This spec follows the shared surface and setup-path rules in [Projects](../../README.md#shared-spec-conventions).

For this project, every tutorial run should adapt the same checkout-calculation and report-formatting behavior instead of redefining it.

Adapters should parse prompted prices into exact cent values before calling the core logic.

## Output Repository Expectations

This project follows the shared output model in [Projects](../../README.md#shared-output-model).

For this project, the core repo owns `calculate_checkout_summary` and `format_checkout_report`, and adapter repos must not reimplement them.

## Testing And Coverage Contract

This spec follows the shared coverage policy in [Projects](../../README.md#shared-coverage-policy).

This project should be built in a spec-driven and test-driven way.

Minimum test expectations:

- tests for `calculate_checkout_summary` on the canonical example
- tests for a tax-rounding case where a `$1.00` subtotal produces `$0.06` tax
- tests that line-item totals preserve item order and quantity multiplication
- tests that `format_checkout_report` preserves the exact line order and wording
- tests that all rendered money values use `$` and exactly two decimal places
- tests for every adapter built in the chosen run that prove it delegates to the core logic correctly
