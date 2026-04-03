---
partial_kind: core-instructions
project: product-search
---

# Core Instructions

Project-specific core instruction fragment for the `product-search` core repo.

### 1. Red: Write The First Failing Test

Start with parsing the canonical inventory JSON:

```text
given the canonical inventory json text
when parse_product_catalog is called
then the returned products preserve the exact source order and names
and the prices are converted into whole-number cents
```

At this point, `parse_product_catalog` may not exist yet. That is fine. The first run should fail either at compile time or at test execution time.

Suggested generic test name:

```text
parse_product_catalog_preserves_the_canonical_products_and_price_cents
```

### 2. Green: Make The First Test Pass

Create the smallest production code that makes the first test pass.

At this stage, it is acceptable if the code only handles the canonical parsing case. Do not jump ahead and implement lookup or formatter behavior before those tests exist.

### 3. Refactor

Do the smallest cleanup that keeps the first test green.

### 4. Red: Add The Exact-Name Lookup

Write the next failing test:

```text
given the parsed canonical product catalog
when find_product_by_exact_name is called with Widget
then the returned product is Widget with price_cents 2500 and quantity_on_hand 5
```

Suggested generic test name:

```text
find_product_by_exact_name_returns_the_canonical_widget_record
```

### 5. Green: Make The Lookup Test Pass

Make the smallest safe change that gets both tests green.

### 6. Refactor

Clean up the implementation while keeping both tests green.

### 7. Red: Add The Not-Found Case

Write the next failing test:

```text
given the parsed canonical product catalog
when find_product_by_exact_name is called with iPad
then the result is null
```

Suggested generic test name:

```text
find_product_by_exact_name_returns_null_for_a_missing_product
```

### 8. Green: Make The Not-Found Test Pass

Make it pass while keeping the earlier tests green.

### 9. Refactor

Clean up the implementation while keeping all parsing and lookup tests green.

### 10. Red: Add The Product Formatter

Write the next failing test:

```text
given the canonical Widget product
when format_product_report is called
then the returned lines are:
Name: Widget
Price: $25.00
Quantity on hand: 5
```

Suggested generic test name:

```text
format_product_report_returns_the_canonical_widget_output
```

### 11. Green: Make The Product Formatter Test Pass

Make it pass while keeping the earlier tests green.

### 12. Red: Add The Not-Found Formatter

Write the next failing test:

```text
when format_product_not_found_message is called
then the result is:
Sorry, that product was not found in our inventory.
```

Suggested generic test name:

```text
format_product_not_found_message_returns_the_canonical_sentence
```

### 13. Green: Make The Not-Found Formatter Test Pass

Make it pass while keeping the earlier tests green.

### 14. Refactor The Core Logic

Now that the full behavior is covered, refactor toward a small, clear core abstraction.

The exact shape can vary by stack:

- four functions in one module
- one service object with parser, lookup, and formatter methods
- one module plus a small immutable product type

The important thing is that JSON parsing, exact-name lookup, and the two output paths stay small, explicit, and directly testable.

