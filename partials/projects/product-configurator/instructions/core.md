---
partial_kind: core-instructions
project: product-configurator
---

# Core Instructions

Project-specific core instruction fragment for the `product-configurator` core repo.

### 1. Red: Build The Canonical Variant Catalog

Start with this failing test:

```text
given no user input
when build_variant_catalog is called
then the returned variants are cube, tall-box, and flat-box in order
```

### 2. Green: Make The Catalog Test Pass

Create the smallest production code that returns the canonical catalog.

### 3. Red: Add Exact-Id Lookup

Write failing tests for exact-id success and failure.

### 4. Green: Make The Lookup Tests Pass

Make them pass while keeping the earlier catalog test green.

### 5. Red: Build The Variant Scene

Write a failing test that requires `build_variant_scene` to copy the selected variant's source, orbit, and exposure.

### 6. Green: Make The Scene Test Pass

Make it pass while keeping the earlier tests green.
