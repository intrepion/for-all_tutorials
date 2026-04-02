<!-- breadcrumbs:start -->
[for-all_tutorials](../../README.md) / [Projects](../README.md) / Self-Checkout
<!-- breadcrumbs:end -->

# Self-Checkout

Project home for the tenth project app in the curriculum.

## Contents

- [Spec](spec/README.md)
- [Tutorial](tutorial/README.md)

## Purpose

`self-checkout` exists to teach:

- parsing three prompted item prices and quantities at the adapter boundary
- representing money as exact cents in the core logic
- calculating line-item totals, subtotal, tax, and total
- applying a fixed `5.5%` tax rate with deterministic cent rounding
- separating checkout calculation from output formatting
- keeping adapters thin while the core owns the money and tax rules

This project follows the shared workflow and output model in [Projects](../README.md#shared-workflow) and [Projects](../README.md#shared-output-model).
