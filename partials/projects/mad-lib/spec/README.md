---
partial_kind: project-spec
project: mad-lib
---

# Spec

Canonical project contract for the `mad-lib` project.

## Goal

Build a small project app that introduces:

- combining four plain-text inputs into one fixed story template
- preserving user-provided words exactly as entered
- test-first formatting logic in one small core function
- thin surface adapters

## Core Logic Contract

The shared core contract is:

```text
format_mad_lib(noun: string, verb: string, adjective: string, adverb: string) -> string
```

Canonical behavior:

- preserve `noun`, `verb`, `adjective`, and `adverb` exactly as entered
- return `Do you <verb> your <adjective> <noun> <adverb>? That's hilarious!`
- include the literal words `Do you` at the start of the sentence
- include the literal ending `? That's hilarious!`
- keep the story template fixed for every implementation of this project

Examples:

- `format_mad_lib("dog", "walk", "blue", "quickly")` returns `Do you walk your blue dog quickly? That's hilarious!`
- `format_mad_lib("cat", "juggle", "tiny", "slowly")` returns `Do you juggle your tiny cat slowly? That's hilarious!`
- `format_mad_lib("space hamster", "admire", "neon-green", "very carefully")` returns `Do you admire your neon-green space hamster very carefully? That's hilarious!`

## Non-Goals

This project does not include:

- persistence
- localization
- authentication
- network access
- configuration files
- validation for missing words
- multiple story templates
- random story generation
- grammar correction or article selection

## Surface Expectations

This spec follows the shared surface and setup-path rules in [Projects](../../README.md#shared-spec-conventions).

For this project, every tutorial run should adapt the same mad-lib formatting behavior instead of redefining it.

## Output Repository Expectations

This project follows the shared output model in [Projects](../../README.md#shared-output-model).

For this project, the core repo owns `format_mad_lib`, and adapter repos must not reimplement it.

## Testing And Coverage Contract

This spec follows the shared coverage policy in [Projects](../../README.md#shared-coverage-policy).

This project should be built in a spec-driven and test-driven way.

Minimum test expectations:

- tests for the canonical example story
- tests that words are inserted into the correct positions in the template
- tests that multiword and punctuated input is preserved exactly
- tests that the fixed ending `? That's hilarious!` is preserved
- tests for every adapter built in the chosen run that prove it delegates to the core logic correctly

