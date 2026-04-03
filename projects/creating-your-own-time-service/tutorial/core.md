<!-- breadcrumbs:start -->
[for-all_tutorials](../../../../README.md) / [Projects](../../../README.md) / [Creating Your Own Time Service](../../README.md) / [Tutorial](../README.md) / core.md
<!-- breadcrumbs:end -->

# Core Walkthrough

Project-specific red, green, refactor sequence for the `creating-your-own-time-service` core repo.

### 1. Red: Write The First Failing Test

Start with building the payload:

```text
given a UTC timestamp string
when build_current_time_payload is called
then the returned payload preserves the exact timestamp
```

At this point, `build_current_time_payload` may not exist yet. That is fine. The first run should fail either at compile time or at test execution time.

Suggested generic test name:

```text
build_current_time_payload_preserves_the_exact_timestamp
```

### 2. Green: Make The First Test Pass

Create the smallest production code that makes the first test pass.

At this stage, it is acceptable if the code only handles this one payload-building case. Do not jump ahead and implement JSON formatting or client-side parsing before those tests exist.

### 3. Refactor

Do the smallest cleanup that keeps the first test green.

### 4. Red: Add Service JSON Formatting

Write the next failing test:

```text
given the canonical payload
when format_current_time_service_response is called
then the returned JSON is equivalent to {"currentTime":"2050-01-04 15:06:26"}
```

Suggested generic test name:

```text
format_current_time_service_response_returns_the_canonical_json_shape
```

### 5. Green: Make The Service Formatter Test Pass

Make the smallest safe change that gets both tests green.

### 6. Red: Add Client Parsing And Display Formatting

Write the next failing tests:

```text
given the canonical service response json
when parse_current_time_service_response is called
then the returned payload preserves the exact canonical timestamp
```

and

```text
given the canonical parsed payload
when format_current_time_display is called
then the returned message is:
The current time is 15:06:26 UTC January 4 2050.
```

Suggested generic test names:

```text
parse_current_time_service_response_preserves_the_canonical_timestamp
format_current_time_display_returns_the_canonical_message
```

### 7. Green: Make The Parsing And Display Tests Pass

Make the smallest safe change that gets the full suite green.

### 8. Refactor The Core Logic

Now that the full behavior is covered, refactor toward a small, clear core abstraction.

The exact shape can vary by stack:

- four functions in one module
- one service object with payload, parse, and format methods
- one module plus a small immutable payload type

The important thing is that payload construction, service-response formatting, response parsing, and display formatting stay small, explicit, and directly testable.
