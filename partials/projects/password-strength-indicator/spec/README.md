---
partial_kind: project-spec
project: password-strength-indicator
---

# Spec

Canonical project contract for the `password-strength-indicator` project.

## Goal

Build a small project app that introduces:

- scoring one password candidate by length and character categories
- separating score calculation, strength classification, and output formatting
- making fixed classification rules explicit and testable
- preserving the entered password exactly in the displayed result
- test-first scoring, classification, and formatting logic in small core functions
- thin surface adapters

## Core Logic Contract

The shared core contracts are:

```text
calculate_password_strength_score(password: string) -> integer
classify_password_strength(score: integer) -> strength
format_password_strength_message(password: string, strength: string) -> string
```

Canonical behavior:

- treat `password` as a plain-text ASCII string
- count every character exactly as entered
- do not trim, lowercase, normalize, or otherwise rewrite the password before classification
- treat ASCII letters `A-Z` and `a-z` as letters
- treat ASCII digits `0-9` as numbers
- treat any ASCII character that is neither a letter nor a digit as a special character
- `calculate_password_strength_score` returns the sum of these points:
  - `3` points when the password has at least `16` characters
  - `2` points when the password contains at least one special character
  - `1` point when the password contains at least one uppercase letter
  - `1` point when the password contains at least one lowercase letter
  - `1` point when the password contains at least one digit
- `classify_password_strength` returns:
  - `very weak` when `score` is from `0` through `2`, inclusive
  - `weak` when `score` is from `3` through `4`, inclusive
  - `strong` when `score` is from `5` through `6`, inclusive
  - `very strong` when `score` is from `7` through `8`, inclusive
- these four categories are total and non-overlapping, so every password classifies into exactly one of them
- `format_password_strength_message` returns:
  - `The password '<password>' is a <strength> password.`

Examples:

- `calculate_password_strength_score("12345")` returns `1`
- `classify_password_strength(1)` returns `very weak`
- `calculate_password_strength_score("abcdefghijklmnop")` returns `4`
- `classify_password_strength(4)` returns `weak`
- `calculate_password_strength_score("Abcdefghijklmno1")` returns `6`
- `classify_password_strength(6)` returns `strong`
- `calculate_password_strength_score("Abcdefghijklmn1!")` returns `8`
- `classify_password_strength(8)` returns `very strong`
- `format_password_strength_message("12345", "very weak")` returns `The password '12345' is a very weak password.`
- `format_password_strength_message("abcdefghijklmnop", "weak")` returns `The password 'abcdefghijklmnop' is a weak password.`
- `format_password_strength_message("Abcdefghijklmno1", "strong")` returns `The password 'Abcdefghijklmno1' is a strong password.`
- `format_password_strength_message("Abcdefghijklmn1!", "very strong")` returns `The password 'Abcdefghijklmn1!' is a very strong password.`

## Non-Goals

This project does not include:

- persistence
- localization
- authentication
- network access
- configuration files
- entropy scoring
- password breach or dictionary checks
- real-world secure password handling such as masking, hashing, or avoiding password echo
- Unicode normalization or non-ASCII character-class rules
- password-policy enforcement beyond the fixed exercise categories above
- any additional sub-ranking inside the four canonical categories

## Surface Expectations

This spec follows the shared surface and setup-path rules in [Projects](../../README.md#shared-spec-conventions).

For this project, every tutorial run should adapt the same password-classification and message-formatting behavior instead of redefining it.

Adapters should collect one password string, pass it to `calculate_password_strength_score`, pass the resulting score to `classify_password_strength`, and pass the password and resulting strength into `format_password_strength_message`.

## Output Repository Expectations

This project follows the shared output model in [Projects](../../README.md#shared-output-model).

For this project, the core repo owns `calculate_password_strength_score`, `classify_password_strength`, and `format_password_strength_message`, and adapter repos must not reimplement them.

## Testing And Coverage Contract

This spec follows the shared coverage policy in [Projects](../../README.md#shared-coverage-policy).

This project should be built in a spec-driven and test-driven way.

Minimum test expectations:

- tests that prove each scoring rule contributes the correct number of points
- tests that prove the `16`-character threshold earns the full `3` points
- tests for the score boundaries between `very weak`, `weak`, `strong`, and `very strong`
- tests that `format_password_strength_message` preserves the exact canonical sentence for each category
- tests for every adapter built in the chosen run that prove it delegates to the core logic correctly

