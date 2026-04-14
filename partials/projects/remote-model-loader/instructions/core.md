---
partial_kind: core-instructions
project: remote-model-loader
---

# Core Instructions

Project-specific core instruction fragment for the `remote-model-loader` core repo.

### 1. Red: Calculate The First Progress Percentage

Start with this failing test:

```text
given 25 loaded bytes and 100 total bytes
when calculate_load_percentage is called
then the result is 25
```

### 2. Green: Make The First Progress Test Pass

Create the smallest production code that returns the canonical percentage.

### 3. Red: Add The Loading Message

Write a failing test that requires `format_loading_message(42)` to return `Loading model... 42%`.

### 4. Green: Make The Loading Message Test Pass

Make it pass while keeping the earlier progress test green.

### 5. Red: Add The Error Message And Interaction Rule

Write failing tests for:

```text
format_model_load_error("sample-box")
can_interact_with_model(false, false)
can_interact_with_model(true, false)
```

### 6. Green: Make Those Tests Pass

Make them pass while keeping the earlier tests green.
