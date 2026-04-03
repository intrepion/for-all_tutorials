<!-- breadcrumbs:start -->
[for-all_tutorials](../../../../README.md) / [Projects](../../../README.md) / [Temperature Converter](../../README.md) / [Tutorial](../README.md) / core.md
<!-- breadcrumbs:end -->

# Core Walkthrough

Project-specific red, green, refactor sequence for the `temperature-converter` core repo.

### 1. Red: Write The First Failing Test

Start with the canonical Fahrenheit-to-Celsius example:

```text
given starting_temperature_tenths 320
and conversion_choice C
when convert_temperature_tenths is called
then the result is 0
```

At this point, `convert_temperature_tenths` may not exist yet. That is fine. The first run should fail either at compile time or at test execution time.

Suggested generic test name:

```text
convert_temperature_tenths_converts_freezing_fahrenheit_to_zero_celsius
```

### 2. Green: Make The First Test Pass

Create the smallest production code that makes the first test pass.

At this stage, it is acceptable if the code only handles this first case. Do not jump ahead and implement formatter behavior before the next tests exist.

### 3. Refactor

Do the smallest cleanup that keeps the first test green.

### 4. Red: Add A Non-Whole Celsius Result

Write the next failing test:

```text
given starting_temperature_tenths 1000
and conversion_choice C
when convert_temperature_tenths is called
then the result is 378
```

Suggested generic test name:

```text
convert_temperature_tenths_rounds_fahrenheit_to_celsius_to_the_nearest_tenth
```

### 5. Green: Make The Non-Whole Test Pass

Make the smallest safe change that gets both tests green.

### 6. Refactor

Clean up the implementation while keeping both tests green.

### 7. Red: Add A Celsius-To-Fahrenheit Case

Write the next failing test:

```text
given starting_temperature_tenths 0
and conversion_choice F
when convert_temperature_tenths is called
then the result is 320
```

Suggested generic test name:

```text
convert_temperature_tenths_converts_zero_celsius_to_thirty_two_fahrenheit
```

### 8. Green: Make The Fahrenheit Test Pass

Make it pass while keeping the earlier tests green.

### 9. Refactor

Clean up the implementation while keeping all conversion tests green.

### 10. Red: Add The Celsius Report Formatter

Write the next failing test:

```text
given conversion_choice C
and converted_temperature_tenths 0
when format_temperature_conversion_report is called
then the returned message is:
The temperature in Celsius is 0.
```

Suggested generic test name:

```text
format_temperature_conversion_report_returns_the_canonical_celsius_sentence
```

### 11. Green: Make The Celsius Formatter Test Pass

Make it pass.

### 12. Refactor

Clean up the implementation while keeping all tests green.

### 13. Red: Add The Fahrenheit Report Formatter

Write the next failing test:

```text
given conversion_choice F
and converted_temperature_tenths 320
when format_temperature_conversion_report is called
then the returned message is:
The temperature in Fahrenheit is 32.
```

Suggested generic test name:

```text
format_temperature_conversion_report_returns_the_canonical_fahrenheit_sentence
```

### 14. Green: Make The Fahrenheit Formatter Test Pass

Make it pass while keeping the earlier tests green.

### 15. Red: Prove Non-Whole Results Keep One Decimal Place

Write the next failing test:

```text
given conversion_choice C
and converted_temperature_tenths 378
when format_temperature_conversion_report is called
then the returned message is:
The temperature in Celsius is 37.8.
```

Suggested generic test name:

```text
format_temperature_conversion_report_keeps_one_decimal_place_for_non_whole_results
```

### 16. Green: Make The Decimal-Formatting Test Pass

Make it pass while keeping the earlier tests green.

### 17. Refactor The Core Logic

Now that the full behavior is covered, refactor toward a small, clear core abstraction.

The exact shape can vary by stack:

- two functions in one module
- one service object with one conversion method and one formatter method
- one module plus a small conversion-result record or struct

The important thing is that branching, temperature conversion, rounding, and report formatting stay small, explicit, and directly testable.
