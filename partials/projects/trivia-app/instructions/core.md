---
partial_kind: core-instructions
project: trivia-app
---

# Core Instructions

Project-specific core instruction fragment for the `trivia-app` core repo.

### 1. Red: Write The First Failing Test

Start with parsing the canonical question bank:

```text
given the canonical trivia question bank json
when parse_trivia_question_bank is called
then the returned questions preserve the exact canonical order, prompts, correct answers, and distractors
```

At this point, `parse_trivia_question_bank` may not exist yet. That is fine. The first run should fail either at compile time or at test execution time.

Suggested generic test name:

```text
parse_trivia_question_bank_preserves_the_canonical_questions_in_order
```

### 2. Green: Make The First Test Pass

Create the smallest production code that makes the first test pass.

At this stage, it is acceptable if the code only handles the canonical parsing case. Do not jump ahead and implement choice-pool or answer-checking behavior before those tests exist.

### 3. Refactor

Do the smallest cleanup that keeps the first test green.

### 4. Red: Add Choice Pool And Answer Checking

Write the next failing tests:

```text
given the canonical arithmetic question
when build_trivia_choice_pool is called
then the returned list contains the exact correct answer first followed by the canonical distractors
```

and

```text
given the canonical arithmetic question
when is_correct_trivia_answer is called with 4
then true is returned
```

and

```text
given the canonical arithmetic question
when is_correct_trivia_answer is called with 5
then false is returned
```

Suggested generic test names:

```text
build_trivia_choice_pool_returns_the_correct_answer_and_distractors_in_canonical_order
is_correct_trivia_answer_returns_true_for_the_exact_correct_answer
is_correct_trivia_answer_returns_false_for_a_wrong_answer
```

### 5. Green: Make The Choice And Answer Tests Pass

Make the smallest safe change that gets those tests green.

### 6. Red: Add Final Score Formatting

Write the next failing test:

```text
given a correct answer count of 2
when format_trivia_final_score is called
then the returned sentence is You answered 2 questions correctly.
```

Suggested generic test name:

```text
format_trivia_final_score_returns_the_canonical_sentence
```

### 7. Green: Make The Score Test Pass

Make the smallest safe change that gets the full suite green.

### 8. Refactor The Core Logic

Now that the full behavior is covered, refactor toward a small, clear core abstraction.

The exact shape can vary by stack:

- four functions in one module
- one service object with parse, choice-pool, answer-check, and score-format methods
- one module plus a small immutable trivia-question type

The important thing is that question parsing, choice-pool building, answer checking, and final score formatting stay small, explicit, and directly testable.

