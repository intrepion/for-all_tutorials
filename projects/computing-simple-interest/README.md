<!-- breadcrumbs:start -->
[for-all_tutorials](../../README.md) / [Projects](../README.md) / Computing Simple Interest
<!-- breadcrumbs:end -->

# Computing Simple Interest

Project home for the twelfth project app in the curriculum.

## Contents

- [Spec](spec/README.md)
- [Tutorial](tutorial/README.md)

## Purpose

`computing-simple-interest` exists to teach:

- parsing a whole-number principal amount at the adapter boundary
- parsing an entered interest rate into an exact scaled core value
- applying the simple-interest formula in an explicit order of operations
- rounding the accrued amount to cents with deterministic half-up rounding
- separating interest calculation from output formatting
- keeping adapters thin while the core owns the rate-scaling, arithmetic, and reporting rules

This project follows the shared workflow and output model in [Projects](../README.md#shared-workflow) and [Projects](../README.md#shared-output-model).
