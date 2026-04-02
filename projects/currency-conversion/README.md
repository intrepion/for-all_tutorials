<!-- breadcrumbs:start -->
[for-all_tutorials](../../README.md) / [Projects](../README.md) / Currency Conversion
<!-- breadcrumbs:end -->

# Currency Conversion

Project home for the eleventh project app in the curriculum.

## Contents

- [Spec](spec/README.md)
- [Tutorial](tutorial/README.md)

## Purpose

`currency-conversion` exists to teach:

- parsing a whole-number euro amount at the adapter boundary
- parsing an entered exchange rate into an exact scaled core value
- converting euros to U.S. dollars with deterministic cent rounding
- separating conversion calculation from output formatting
- keeping adapters thin while the core owns the scaling, rounding, and reporting rules

This project follows the shared workflow and output model in [Projects](../README.md#shared-workflow) and [Projects](../README.md#shared-output-model).
