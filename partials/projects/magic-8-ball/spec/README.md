---
partial_kind: project-spec
project: magic-8-ball
---

# Spec

Canonical project contract for the `magic-8-ball` project.

## Goal

Build a small project app that introduces:

- storing a fixed set of four possible answers
- selecting one answer by index after the adapter chooses a random value
- separating response lookup from random number generation and input/output
- test-first response-catalog logic in small core functions
- thin surface adapters

## Core Logic Contract

The shared core contracts are:

```text
magic_8_ball_response_count() -> integer
magic_8_ball_response(choice_number: integer) -> string | null
```

Canonical behavior:

- `magic_8_ball_response_count` returns `4`
- `magic_8_ball_response` returns:
  - `Yes` for `choice_number` `1`
  - `No` for `choice_number` `2`
  - `Maybe` for `choice_number` `3`
  - `Ask again later.` for `choice_number` `4`
  - `null` for any other value
- the core logic does not inspect or interpret the user question
- the core logic does not generate random values

Examples:

- `magic_8_ball_response_count()` returns `4`
- `magic_8_ball_response(1)` returns `Yes`
- `magic_8_ball_response(2)` returns `No`
- `magic_8_ball_response(3)` returns `Maybe`
- `magic_8_ball_response(4)` returns `Ask again later.`
- `magic_8_ball_response(5)` returns `null`

## Non-Goals

This project does not include:

- persistence
- localization
- authentication
- network access
- configuration files
- interpreting the meaning of the user question
- weighted or stateful randomness
- replay flow
- custom response sets
- validation of empty questions in the core

## Surface Expectations

This spec follows the shared surface and setup-path rules in [Projects](../../README.md#shared-spec-conventions).

For this project, every tutorial run should adapt the same response-catalog behavior instead of redefining it.

Adapters should:

- prompt for a question
- ignore the actual question text for response selection beyond collecting it from the user
- call `magic_8_ball_response_count` to determine the upper bound for random selection
- choose a random whole number between `1` and that count, inclusive
- pass the chosen number to `magic_8_ball_response`
- render, return, or print the returned response in the form that surface requires

For prompt-driven adapters, the question prompt should be equivalent to:

```text
What's your question?
```

## Output Repository Expectations

This project follows the shared output model in [Projects](../../README.md#shared-output-model).

For this project, the core repo owns `magic_8_ball_response_count` and `magic_8_ball_response`, and adapter repos must not reimplement them.

## Testing And Coverage Contract

This spec follows the shared coverage policy in [Projects](../../README.md#shared-coverage-policy).

This project should be built in a spec-driven and test-driven way.

Minimum test expectations:

- a test that `magic_8_ball_response_count` returns `4`
- tests that `magic_8_ball_response` returns each of the four canonical responses for indices `1` through `4`
- a test that `magic_8_ball_response` returns `null` for an out-of-range value such as `5`
- tests for every adapter built in the chosen run that prove it uses the response count as the random upper bound and delegates to the core logic correctly

