<!-- breadcrumbs:start -->
[for-all_tutorials](../../README.md) / [Projects](../README.md) / Determining Compound Interest
<!-- breadcrumbs:end -->

# Determining Compound Interest

Project home for the thirteenth project app in the curriculum.

## Contents

- [Spec](spec/README.md)
- [Tutorial](tutorial/README.md)

## Purpose

`determining-compound-interest` exists to teach:

- parsing a whole-number principal amount at the adapter boundary
- parsing an entered interest rate into an exact scaled core value
- computing compound growth with the exponent form of the investment formula
- rounding the final accrued amount to cents with deterministic half-up rounding
- separating compound-interest calculation from output formatting
- keeping adapters thin while the core owns the scaling, exponent-based calculation, and reporting rules

This project follows the shared workflow and output model in [Projects](../README.md#shared-workflow) and [Projects](../README.md#shared-output-model).
