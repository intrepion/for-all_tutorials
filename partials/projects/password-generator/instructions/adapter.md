---
partial_kind: adapter-instructions
project: password-generator
---

# Adapter Instructions

Project-specific adapter instruction fragment for the `password-generator` adapter repo.

### 1. Red: Add An Adapter-Level Test

In a separate adapter repo, add a failing test that proves the chosen adapter parses the three requirement inputs, uses the core generation plan and character pools, randomly chooses the correct number of each character type, shuffles the final characters, and delegates final formatting to the core logic correctly.

### 2. Green: Add The Thin Surface Adapter

Add the thinnest possible adapter for the surface you are building:

- render the equivalents of:
  - `What's the minimum length?`
  - `How many special characters?`
  - `How many numbers?`
- parse the three entered values into whole numbers
- call `password_generation_plan`
- call `letter_character_pool`
- call `digit_character_pool`
- call `special_character_pool`
- randomly choose:
  - `letter_count` characters from the letter pool
  - `special_character_count` characters from the special-character pool
  - `number_count` characters from the digit pool
- combine those chosen characters
- randomly shuffle the combined character list
- join the shuffled characters into one password string
- pass the password string to `format_generated_password`
- render the returned lines in order
- keep random selection, shuffling, parsing, transport, and input/output code out of the core password-generation logic

### 3. Refactor

Clean up any remaining duplication while keeping the full suite green.

