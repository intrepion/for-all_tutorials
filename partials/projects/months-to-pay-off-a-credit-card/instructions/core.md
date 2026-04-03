---
partial_kind: core-instructions
project: months-to-pay-off-a-credit-card
---

# Core Instructions

Project-specific core instruction fragment for the `months-to-pay-off-a-credit-card` core repo.

### 1. Red: Write The First Failing Test

Start with the canonical payoff example:

```text
given balance_dollars 5000
and apr_tenths_percent 120
and monthly_payment_dollars 100
when calculate_credit_card_payoff_months is called
then months is 70
```

At this point, `calculate_credit_card_payoff_months` may not exist yet. That is fine. The first run should fail either at compile time or at test execution time.

Suggested generic test name:

```text
calculate_credit_card_payoff_months_returns_the_canonical_month_count
```

### 2. Green: Make The First Test Pass

Create the smallest production code that makes the first test pass.

At this stage, it is acceptable if the code only handles this first case. Do not jump ahead and implement formatter behavior before the next tests exist.

### 3. Refactor

Do the smallest cleanup that keeps the first test green.

### 4. Red: Add A Second APR-Sensitive Example

Write the next failing test:

```text
given balance_dollars 1200
and apr_tenths_percent 180
and monthly_payment_dollars 100
when calculate_credit_card_payoff_months is called
then months is 14
```

Suggested generic test name:

```text
calculate_credit_card_payoff_months_uses_the_entered_apr_value
```

### 5. Green: Make The APR Test Pass

Make the smallest safe change that gets both tests green.

### 6. Refactor

Clean up the implementation while keeping both tests green.

### 7. Red: Add The Ceiling-Rounding Case

Write the next failing test:

```text
given balance_dollars 1000
and apr_tenths_percent 120
and monthly_payment_dollars 100
when calculate_credit_card_payoff_months is called
then months is 11
```

Suggested generic test name:

```text
calculate_credit_card_payoff_months_rounds_up_to_the_next_whole_month
```

### 8. Green: Make The Rounding Test Pass

Make it pass while keeping the earlier tests green.

### 9. Refactor

Clean up the implementation while keeping all payoff-calculation tests green.

### 10. Red: Add The Report Formatter

Write the next failing test:

```text
given months 70
when format_credit_card_payoff_report is called
then the returned message is:
It will take you 70 months to pay off this card.
```

Suggested generic test name:

```text
format_credit_card_payoff_report_returns_the_canonical_sentence
```

### 11. Green: Make The Formatter Test Pass

Make it pass.

### 12. Refactor The Core Logic

Now that the full behavior is covered, refactor toward a small, clear core abstraction.

The exact shape can vary by stack:

- two functions in one module
- one service object with one calculation method and one formatter method
- one module plus a small payoff result record or struct

The important thing is that payoff calculation, whole-month ceiling rounding, and report formatting stay small, explicit, and directly testable.

