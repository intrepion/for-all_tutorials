---
partial_kind: core-instructions
project: grabbing-the-weather
---

# Core Instructions

Project-specific core instruction fragment for the `grabbing-the-weather` core repo.

### 1. Red: Write The First Failing Test

Start with parsing the canonical sample response:

```text
given the canonical sample weather response json
when parse_current_weather_response is called
then the returned report preserves city_name
and returns the rounded whole-degree Fahrenheit temperature
```

At this point, `parse_current_weather_response` may not exist yet. That is fine. The first run should fail either at compile time or at test execution time.

Suggested generic test name:

```text
parse_current_weather_response_preserves_the_canonical_city_and_temperature
```

### 2. Green: Make The First Test Pass

Create the smallest production code that makes the first test pass.

At this stage, it is acceptable if the code only handles the canonical sample parsing case. Do not jump ahead and implement formatter behavior before those tests exist.

### 3. Refactor

Do the smallest cleanup that keeps the first test green.

### 4. Red: Add The Half-Up Rounding Case

Write the next failing test:

```text
given a response with city_name Chicago and main.temp 65.5
when parse_current_weather_response is called
then the returned temperature_fahrenheit is 66
```

Suggested generic test name:

```text
parse_current_weather_response_rounds_half_up
```

### 5. Green: Make The Rounding Test Pass

Make the smallest safe change that gets both tests green.

### 6. Refactor

Clean up the implementation while keeping both tests green.

### 7. Red: Add The Weather Formatter

Write the next failing test:

```text
given city_name Chicago and temperature_fahrenheit 65
when format_current_weather_report is called
then the returned lines are:
Chicago weather:
65 degrees Fahrenheit
```

Suggested generic test name:

```text
format_current_weather_report_returns_the_canonical_two_line_output
```

### 8. Green: Make The Formatter Test Pass

Make it pass while keeping the earlier tests green.

### 9. Refactor The Core Logic

Now that the full behavior is covered, refactor toward a small, clear core abstraction.

The exact shape can vary by stack:

- two functions in one module
- one service object with parser and formatter methods
- one module plus a small immutable report type

The important thing is that weather-response parsing and report formatting stay small, explicit, and directly testable.

