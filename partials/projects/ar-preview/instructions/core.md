---
partial_kind: core-instructions
project: ar-preview
---

# Core Instructions

Project-specific core instruction fragment for the `ar-preview` core repo.

### 1. Red: Select The Canonical AR Scale Mode

Start with failing tests for `fixed`, `auto`, and unknown mode names.

### 2. Green: Make The Scale Tests Pass

Create the smallest production code that returns the canonical scale mode and fallback.

### 3. Red: Add AR Launch Eligibility

Write failing tests for all combinations of device support and camera permission.

### 4. Green: Make The Eligibility Tests Pass

Make them pass while keeping the scale-mode tests green.

### 5. Red: Add The Unavailable Message

Write a failing test for the canonical unsupported-device message.

### 6. Green: Make The Message Test Pass

Make it pass while keeping the earlier tests green.
