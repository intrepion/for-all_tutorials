---
partial_kind: instructions-index
project: product-search
---

# Instructions

Instructions for building `product-search` from the spec using one setup path and separate core and adapter walkthroughs.

## Contents

- [Core Instructions](core.md)
- [Adapter Instructions](adapter.md)

## Project-Specific Flow

Follow the shared repository-creation and instruction workflow in [Projects](../../README.md#shared-repository-creation) and [Projects](../../README.md#shared-instruction-workflow), then use this project's instruction files:

- core repo: [Core Instructions](core.md)
- adapter repo: [Adapter Instructions](adapter.md)

For this project, the core repo should expose `parse_product_catalog`, `find_product_by_exact_name`, `format_product_report`, and `format_product_not_found_message`, and the adapter repo should read the canonical inventory JSON, retry until a product is found, and render the canonical product report by delegating parsing, lookup, and formatting to those core functions.

This file is a source index, not a compiled tutorial. Use the shared finish checklist in [Projects](../../README.md#shared-finish-checklist) together with the project-specific rules in [Spec](../spec/README.md) and the declared outputs in [Manifest](../manifest.yaml).

