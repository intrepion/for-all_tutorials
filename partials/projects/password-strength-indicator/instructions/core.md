---
partial_kind: core-instructions
project: password-strength-indicator
---

# Core Instructions

Project-specific core instruction fragment for the `password-strength-indicator` core repo.

### 1. Red: Write The First Failing Test

Start with the digit-scoring rule:

```text
given password "12345"
when calculate_password_strength_score is called
then the result is 1
```

At this point, `calculate_password_strength_score` may not exist yet. That is fine. The first run should fail either at compile time or at test execution time.

Suggested generic test name:

```text
calculate_password_strength_score_adds_one_point_for_digits
```

### 2. Green: Make The First Test Pass

Create the smallest production code that makes the first test pass.

At this stage, it is acceptable if the code only handles this first scoring case. Do not jump ahead and implement the other scoring rules, classification, or formatter behavior before those tests exist.

### 3. Refactor

Do the smallest cleanup that keeps the first test green.

### 4. Red: Add The Lowercase Scoring Rule

Write the next failing test:

```text
given password "abcdef"
when calculate_password_strength_score is called
then the result is 1
```

Suggested generic test name:

```text
calculate_password_strength_score_adds_one_point_for_lowercase_letters
```

### 5. Green: Make The Weak Test Pass

Make the smallest safe change that gets both tests green.

### 6. Refactor

Clean up the implementation while keeping both tests green.

### 7. Red: Add The Length Bonus

Write the next failing test:

```text
given password "abcdefghijklmnop"
when calculate_password_strength_score is called
then the result is 4
```

Suggested generic test name:

```text
calculate_password_strength_score_adds_three_points_for_at_least_sixteen_characters
```

### 8. Green: Make The Length Test Pass

Make it pass while keeping the earlier tests green.

### 9. Refactor

Clean up the implementation while keeping all current scoring tests green.

### 10. Red: Add The Uppercase Scoring Rule

Write the next failing test:

```text
given password "Abcdefghijklmnop"
when calculate_password_strength_score is called
then the result is 5
```

Suggested generic test name:

```text
calculate_password_strength_score_adds_one_point_for_uppercase_letters
```

### 11. Green: Make The Uppercase Test Pass

Make it pass while keeping the earlier tests green.

### 12. Refactor

Clean up the implementation while keeping all scoring tests green.

### 13. Red: Add The Digit Combination Case

Write the next failing test:

```text
given password "Abcdefghijklmno1"
when calculate_password_strength_score is called
then the result is 6
```

Suggested generic test name:

```text
calculate_password_strength_score_adds_one_point_for_digits_in_mixed_passwords
```

### 14. Green: Make The Digit Test Pass

Make it pass while keeping the earlier tests green.

### 15. Refactor

Clean up the implementation while keeping all scoring tests green.

### 16. Red: Add The Special-Character Rule

Write the next failing test:

```text
given password "Abcdefghijklmn1!"
when calculate_password_strength_score is called
then the result is 8
```

Suggested generic test name:

```text
calculate_password_strength_score_adds_two_points_for_special_characters
```

### 17. Green: Make The Special-Character Test Pass

Make it pass while keeping the earlier tests green.

### 18. Refactor

Clean up the implementation while keeping all scoring tests green.

### 19. Red: Add The Classification Boundaries

Write failing tests for the score ranges:

```text
given score 2
when classify_password_strength is called
then the result is very weak
```

```text
given score 3
when classify_password_strength is called
then the result is weak
```

```text
given score 5
when classify_password_strength is called
then the result is strong
```

```text
given score 7
when classify_password_strength is called
then the result is very strong
```

Suggested generic test names:

```text
classify_password_strength_returns_very_weak_for_scores_zero_through_two
classify_password_strength_returns_weak_for_scores_three_through_four
classify_password_strength_returns_strong_for_scores_five_through_six
classify_password_strength_returns_very_strong_for_scores_seven_through_eight
```

### 20. Green: Make The Classification Tests Pass

Make them pass while keeping the earlier tests green.

### 21. Refactor

Clean up the implementation while keeping all scoring and classification tests green.

### 22. Red: Add The Canonical Formatter

Write the next failing test:

```text
given password "Abcdefghijklmn1!"
and strength "very strong"
when format_password_strength_message is called
then the result is:
The password 'Abcdefghijklmn1!' is a very strong password.
```

Suggested generic test name:

```text
format_password_strength_message_returns_the_canonical_known_category_sentence
```

### 23. Green: Make The Formatter Test Pass

Make it pass.

### 24. Refactor The Core Logic

Now that the full behavior is covered, refactor toward a small, clear core abstraction.

The exact shape can vary by stack:

- three functions in one module
- one service object with a scoring method, a classification method, and a formatter method
- one module plus one helper for character-category checks
- one static helper

The important thing is that score calculation, password classification, and message formatting stay separate, small, and directly testable.

