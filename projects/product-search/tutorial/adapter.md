<!-- breadcrumbs:start -->
[for-all_tutorials](../../../../README.md) / [Projects](../../../README.md) / [Product Search](../../README.md) / [Tutorial](../README.md) / adapter.md
<!-- breadcrumbs:end -->

# Adapter Walkthrough

Project-specific red, green, refactor sequence for the `product-search` adapter repo.

### 1. Red: Add An Adapter-Level Test

In a separate adapter repo, add a failing test that proves the chosen adapter reads the canonical inventory JSON file, delegates parsing and lookup to the core logic, retries after the not-found case, and renders either the canonical not-found sentence or the canonical product report lines in order.

### 2. Green: Add The Thin Surface Adapter

Add the thinnest possible adapter for the surface you are building:

- read the inventory file contents
- pass those contents to `parse_product_catalog`
- render the equivalent of `What is the product name?`
- pass the entered name to `find_product_by_exact_name`
- when the lookup result is missing:
  - render `format_product_not_found_message`
  - prompt again
- when the lookup result is present:
  - pass the product to `format_product_report`
  - render the returned lines in order
- keep file input/output, retry-loop, transport, and input/output code out of the core inventory-search logic

### 3. Refactor

Clean up any remaining duplication while keeping the full suite green.
