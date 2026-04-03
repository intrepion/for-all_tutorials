<!-- breadcrumbs:start -->
[for-all_tutorials](../../../README.md) / [Projects](../../README.md) / [Trivia App](../README.md) / Spec
<!-- breadcrumbs:end -->

# Spec

Canonical project contract for the `trivia-app` project.

## Goal

Build a small project app that introduces:

- reading trivia questions, correct answers, and distractors from a local file
- parsing a structured question bank into trivia-question records
- building a deterministic answer-choice pool for each question
- checking whether a selected answer is correct
- formatting a final score message
- keeping random question choice and random answer ordering in the adapter
- test-first parsing, answer-checking, and score-formatting logic in small core functions
- thin surface adapters

## Canonical Question Bank File Contents

Every tutorial run for this project should use one source file whose contents are equivalent to this JSON document:

```json
{
  "questions": [
    {
      "prompt": "What is the capital of France?",
      "correct_answer": "Paris",
      "distractors": ["Berlin", "Madrid", "Rome"]
    },
    {
      "prompt": "What is 2 + 2?",
      "correct_answer": "4",
      "distractors": ["3", "5", "22"]
    },
    {
      "prompt": "Which planet is known as the Red Planet?",
      "correct_answer": "Mars",
      "distractors": ["Venus", "Jupiter", "Saturn"]
    }
  ]
}
```

## Core Logic Contract

The shared core contracts are:

```text
parse_trivia_question_bank(json_text: string) -> trivia_question[]
build_trivia_choice_pool(trivia_question) -> string[]
is_correct_trivia_answer(trivia_question, selected_answer: string) -> boolean
format_trivia_final_score(correct_answer_count: integer) -> string
```

Where each `trivia_question` contains:

```text
prompt
correct_answer
distractors
```

Canonical behavior:

- `parse_trivia_question_bank`:
  - parses the JSON document
  - preserves source order of the `questions` array
  - preserves the exact text of each `prompt`
  - preserves the exact text of each `correct_answer`
  - preserves the exact source order and text of each `distractors` list
- `build_trivia_choice_pool`:
  - returns a four-item list
  - puts the exact `correct_answer` first
  - then returns the three distractors in their original source order
  - does not shuffle or sort the choices
- `is_correct_trivia_answer`:
  - compares `selected_answer` by exact string match against `correct_answer`
  - does not trim, normalize, or case-fold `selected_answer`
  - returns `true` for the exact correct answer
  - returns `false` otherwise
- `format_trivia_final_score` returns:
  - `You answered <correct_answer_count> questions correctly.`

Examples:

- `parse_trivia_question_bank(canonical_json_text)` returns questions in this order:
  - `{ prompt: "What is the capital of France?", correct_answer: "Paris", distractors: ["Berlin", "Madrid", "Rome"] }`
  - `{ prompt: "What is 2 + 2?", correct_answer: "4", distractors: ["3", "5", "22"] }`
  - `{ prompt: "Which planet is known as the Red Planet?", correct_answer: "Mars", distractors: ["Venus", "Jupiter", "Saturn"] }`
- `build_trivia_choice_pool({ prompt: "What is 2 + 2?", correct_answer: "4", distractors: ["3", "5", "22"] })` returns:
  - `4`
  - `3`
  - `5`
  - `22`
- `is_correct_trivia_answer({ prompt: "What is 2 + 2?", correct_answer: "4", distractors: ["3", "5", "22"] }, "4")` returns `true`
- `is_correct_trivia_answer({ prompt: "What is 2 + 2?", correct_answer: "4", distractors: ["3", "5", "22"] }, "5")` returns `false`
- `format_trivia_final_score(2)` returns `You answered 2 questions correctly.`

## Non-Goals

This project does not include:

- persistence of player progress
- network access
- localization
- authentication
- weighted question selection
- randomization in the core
- timed questions
- scoreboards
- question authoring in the application itself

## Surface Expectations

This spec follows the shared surface and setup-path rules in [Projects](../../README.md#shared-spec-conventions).

For this project, every tutorial run should adapt the same question-bank parsing, choice-pool construction, answer checking, and final-score behavior instead of redefining it.

Adapters should:

- read one source file whose contents match the canonical question-bank JSON shape
- pass the source file contents to `parse_trivia_question_bank`
- choose questions at random from the parsed list
- avoid repeating already-used questions during one game
- for each chosen question:
  - pass it to `build_trivia_choice_pool`
  - display the returned correct answer and distractors in a random order
  - collect one player choice
  - pass the selected answer to `is_correct_trivia_answer`
  - increment the correct-answer count only after a correct answer
- end the game immediately when the player selects a wrong answer
- when the player answers every available question correctly, end the game after the last question
- render `format_trivia_final_score(correct_answer_count)` when the game ends

## Output Repository Expectations

This project follows the shared output model in [Projects](../../README.md#shared-output-model).

For this project, the core repo owns `parse_trivia_question_bank`, `build_trivia_choice_pool`, `is_correct_trivia_answer`, and `format_trivia_final_score`, and adapter repos must not reimplement them.

## Testing And Coverage Contract

This spec follows the shared coverage policy in [Projects](../../README.md#shared-coverage-policy).

This project should be built in a spec-driven and test-driven way.

Minimum test expectations:

- a test that `parse_trivia_question_bank` preserves the exact canonical question order and values
- a test that `build_trivia_choice_pool` returns the correct answer followed by the canonical distractors in source order
- a test that `is_correct_trivia_answer` returns `true` for an exact correct answer
- a test that `is_correct_trivia_answer` returns `false` for a wrong answer
- a test that `format_trivia_final_score` preserves the exact canonical sentence
- tests for every adapter built in the chosen run that prove it reads the question bank from a file, randomizes question order, randomizes displayed answer order, tracks the number of correct answers, and ends the game immediately after the first wrong answer
