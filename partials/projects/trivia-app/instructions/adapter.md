---
partial_kind: adapter-instructions
project: trivia-app
---

# Adapter Instructions

Project-specific adapter instruction fragment for the `trivia-app` adapter repo.

### 1. Red: Add An Adapter-Level Test

In a separate adapter repo, add a failing test that proves the chosen adapter reads the question bank from a file, chooses questions at random without repeating them, displays the correct answer and distractors in a random order, tracks the number of correct answers, and ends the game immediately after the first wrong answer.

### 2. Green: Add The Thin Adapter

Add the thinnest possible adapter for the surface you are building:

- read one source file containing the trivia question bank
- pass the file contents to `parse_trivia_question_bank`
- choose questions at random from the parsed list without repeating already-used questions
- for each chosen question:
  - pass it to `build_trivia_choice_pool`
  - randomize the order of the returned choices before displaying them
  - collect one player choice
  - pass the chosen answer to `is_correct_trivia_answer`
  - increment the correct-answer count only after a correct answer
- end the game immediately when the player selects a wrong answer
- when the player answers every available question correctly, end the game after the last question
- render `format_trivia_final_score(correct_answer_count)` when the game ends
- keep file I/O, randomization, and game-loop flow out of the core trivia logic

### 3. Refactor

Clean up any remaining duplication while keeping the full suite green.

