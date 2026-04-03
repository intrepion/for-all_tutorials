<!-- breadcrumbs:start -->
[for-all_tutorials](../../../../README.md) / [Projects](../../../README.md) / [Troubleshooting Car Issues](../../README.md) / [Tutorial](../README.md) / core.md
<!-- breadcrumbs:end -->

# Core Walkthrough

Project-specific red, green, refactor sequence for the `troubleshooting-car-issues` core repo.

### 1. Red: Write The First Failing Test

Start with the canonical example path:

```text
given silent_when_turning_key true
and battery_terminals_corroded false
when diagnose_car_issue is called
then the result is replace_cables
```

At this point, `diagnose_car_issue` may not exist yet. That is fine. The first run should fail either at compile time or at test execution time.

Suggested generic test name:

```text
diagnose_car_issue_returns_replace_cables_for_the_canonical_example
```

### 2. Green: Make The First Test Pass

Create the smallest production code that makes the first test pass.

At this stage, it is acceptable if the code only handles this first case. Do not jump ahead and implement other diagnosis paths or formatter behavior before the next tests exist.

### 3. Refactor

Do the smallest cleanup that keeps the first test green.

### 4. Red: Add The Other Diagnosis Paths

Write failing tests for the remaining decision leaves:

```text
silent_when_turning_key true
battery_terminals_corroded true
=> clean_terminals
```

```text
silent_when_turning_key false
clicking_noise true
=> replace_battery
```

```text
silent_when_turning_key false
clicking_noise false
cranks_but_fails_to_start true
=> check_spark_plugs
```

```text
silent_when_turning_key false
clicking_noise false
cranks_but_fails_to_start false
engine_starts_then_dies true
has_fuel_injection true
=> get_service
```

```text
silent_when_turning_key false
clicking_noise false
cranks_but_fails_to_start false
engine_starts_then_dies true
has_fuel_injection false
=> check_choke
```

```text
silent_when_turning_key false
clicking_noise false
cranks_but_fails_to_start false
engine_starts_then_dies false
=> not_possible
```

Suggested generic test names:

```text
diagnose_car_issue_returns_clean_terminals_when_silent_and_corroded
diagnose_car_issue_returns_replace_battery_when_clicking
diagnose_car_issue_returns_check_spark_plugs_when_it_cranks_but_fails_to_start
diagnose_car_issue_returns_get_service_for_fuel_injection_start_then_die
diagnose_car_issue_returns_check_choke_for_non_injection_start_then_die
diagnose_car_issue_returns_not_possible_for_the_uncovered_final_branch
```

### 5. Green: Make The Remaining Diagnosis Tests Pass

Make the smallest safe change that gets all diagnosis tests green.

### 6. Refactor

Clean up the implementation while keeping all diagnosis tests green.

### 7. Red: Add The Canonical Formatter

Write the next failing test:

```text
given diagnosis_code replace_cables
when format_car_diagnosis_message is called
then the returned lines are:
The battery cables may be damaged.
Replace cables and try again.
```

Suggested generic test name:

```text
format_car_diagnosis_message_returns_the_canonical_replace_cables_report
```

### 8. Green: Make The Canonical Formatter Test Pass

Make it pass.

### 9. Refactor

Clean up the implementation while keeping all tests green.

### 10. Red: Add The Remaining Formatters

Write failing tests for the remaining diagnosis messages:

```text
clean_terminals => Clean the terminals and try starting again.
replace_battery => Replace the battery.
check_spark_plugs => Check the spark plug connections.
get_service => Get it in for service.
check_choke => Check to ensure the choke is opening and closing.
not_possible => This should not be possible.
```

Suggested generic test names:

```text
format_car_diagnosis_message_returns_the_clean_terminals_message
format_car_diagnosis_message_returns_the_replace_battery_message
format_car_diagnosis_message_returns_the_check_spark_plugs_message
format_car_diagnosis_message_returns_the_get_service_message
format_car_diagnosis_message_returns_the_check_choke_message
format_car_diagnosis_message_returns_the_not_possible_message
```

### 11. Green: Make The Remaining Formatter Tests Pass

Make them pass while keeping the earlier tests green.

### 12. Refactor The Core Logic

Now that the full behavior is covered, refactor toward a small, clear core abstraction.

The exact shape can vary by stack:

- two functions in one module
- one service object with one diagnosis method and one formatter method
- one module plus a small diagnosis-result record or struct

The important thing is that decision-tree resolution and diagnosis formatting stay small, explicit, and directly testable.
