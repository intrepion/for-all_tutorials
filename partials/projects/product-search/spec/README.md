---
partial_kind: project-spec
project: product-search
---

# Spec

Canonical project contract for the `product-search` project.

## Goal

Build a small project app that introduces:

- reading a fixed canonical JSON inventory file
- parsing JSON product objects into structured records
- locating a product by name from the parsed catalog
- formatting either a product report or a not-found message
- retrying in the adapter when a product is not found
- test-first parsing, lookup, and formatting logic in small core functions
- thin surface adapters

## Canonical Input File Contents

Every tutorial run for this project should use an input file whose contents are equivalent to this JSON document:

```json
{
  "products": [
    { "name": "Widget", "price": 25.00, "quantity": 5 },
    { "name": "Thing", "price": 15.00, "quantity": 5 },
    { "name": "Doodad", "price": 5.00, "quantity": 10 }
  ]
}
```

## Core Logic Contract

The shared core contracts are:

```text
parse_product_catalog(json_text: string) -> product_catalog
find_product_by_exact_name(product_catalog, product_name: string) -> product | null
format_product_report(product) -> string[]
format_product_not_found_message() -> string
```

Where each parsed product contains:

```text
name
price_cents
quantity_on_hand
```

Canonical behavior:

- `parse_product_catalog`:
  - parses the canonical JSON document into a list of product records in source order
  - preserves the exact text of each product `name`
  - converts `price` into whole-number `price_cents`
  - converts `quantity` into whole-number `quantity_on_hand`
- `find_product_by_exact_name`:
  - compares `product_name` by exact string match against each product `name`
  - does not trim, normalize, or case-fold `product_name`
  - returns the first exact matching product when it is present
  - returns `null` when no exact matching product is present
- `format_product_report` returns these lines in this exact order:
  - `Name: <name>`
  - `Price: $<price rendered to exactly two decimal places>`
  - `Quantity on hand: <quantity_on_hand>`
- `format_product_not_found_message` returns:
  - `Sorry, that product was not found in our inventory.`

Examples:

- `parse_product_catalog(canonical_json_text)` returns products in this order:
  - `{ name: "Widget", price_cents: 2500, quantity_on_hand: 5 }`
  - `{ name: "Thing", price_cents: 1500, quantity_on_hand: 5 }`
  - `{ name: "Doodad", price_cents: 500, quantity_on_hand: 10 }`
- `find_product_by_exact_name(parse_product_catalog(canonical_json_text), "Widget")` returns `{ name: "Widget", price_cents: 2500, quantity_on_hand: 5 }`
- `find_product_by_exact_name(parse_product_catalog(canonical_json_text), "iPad")` returns `null`
- `format_product_report({ name: "Widget", price_cents: 2500, quantity_on_hand: 5 })` returns:
  - `Name: Widget`
  - `Price: $25.00`
  - `Quantity on hand: 5`
- `format_product_not_found_message()` returns `Sorry, that product was not found in our inventory.`

## Non-Goals

This project does not include:

- persistence beyond reading the one inventory file handled by the adapter
- localization
- authentication
- network access
- configuration files
- partial-name or fuzzy product matching
- case-insensitive matching
- editing or saving inventory data
- multiple-product result lists
- sorting the product catalog

## Surface Expectations

This spec follows the shared surface and setup-path rules in [Projects](../../README.md#shared-spec-conventions).

For this project, every tutorial run should adapt the same JSON parsing rule, exact-name lookup rule, and output formatting instead of redefining them.

Adapters should:

- read one source file whose contents match the canonical inventory JSON document
- pass the source file contents to `parse_product_catalog`
- prompt for one product name
- pass the entered name to `find_product_by_exact_name`
- when no product is found:
  - render `format_product_not_found_message`
  - prompt again
- when a product is found:
  - pass the product to `format_product_report`
  - render the returned lines in order

For prompt-driven adapters, the prompt should be equivalent to:

```text
What is the product name?
```

## Output Repository Expectations

This project follows the shared output model in [Projects](../../README.md#shared-output-model).

For this project, the core repo owns `parse_product_catalog`, `find_product_by_exact_name`, `format_product_report`, and `format_product_not_found_message`, and adapter repos must not reimplement them.

## Testing And Coverage Contract

This spec follows the shared coverage policy in [Projects](../../README.md#shared-coverage-policy).

This project should be built in a spec-driven and test-driven way.

Minimum test expectations:

- a test that `parse_product_catalog` preserves the exact canonical product names and source order
- a test that `parse_product_catalog` converts the canonical prices into `2500`, `1500`, and `500` cents
- a test that `find_product_by_exact_name` finds `Widget`
- a test that `find_product_by_exact_name` returns `null` for `iPad`
- a test that `format_product_report` preserves the exact canonical three-line output
- a test that `format_product_not_found_message` preserves the exact canonical sentence
- tests for every adapter built in the chosen run that prove it reads the JSON inventory file, retries after the not-found case, and delegates parsing, lookup, and formatting to the core logic correctly

