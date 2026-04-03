<!-- breadcrumbs:start -->
[for-all_tutorials](../../../../README.md) / [Projects](../../../README.md) / [Temperature Converter](../../README.md) / [Tutorial](../README.md) / adapter.md
<!-- breadcrumbs:end -->

# Adapter Walkthrough

Project-specific red, green, refactor sequence for the `temperature-converter` adapter repo.

### 1. Red: Add An Adapter-Level Test

In a separate adapter repo, add a failing test that proves the chosen adapter delegates to the core logic correctly.

### 2. Green: Add The Thin Surface Adapter

Add the thinnest possible adapter for the surface you are building:

- collect one conversion-choice input string from that surface
- collect one temperature input string from that surface
- parse the temperature into tenths that match the core contract
- pass the parsed temperature and the exact conversion choice to `convert_temperature_tenths`
- pass the conversion choice and returned converted temperature to `format_temperature_conversion_report`
- return, render, or print the returned message in the form that surface requires
- keep transport, parsing, and input/output code out of the core conversion and formatting logic

For prompt-driven adapters, the prompts should be equivalent to:

```text
Press C to convert from Fahrenheit to Celsius.
Press F to convert from Celsius to Fahrenheit.
Your choice:
```

and then:

```text
Please enter the temperature in Fahrenheit:
```

when the choice is `C`, or:

```text
Please enter the temperature in Celsius:
```

when the choice is `F`.

### 3. Refactor

Clean up any remaining duplication while keeping the full suite green.
