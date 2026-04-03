<!-- breadcrumbs:start -->
[for-all_tutorials](../../../README.md) / [Projects](../../README.md) / [Word Frequency Finder](../README.md) / Spec
<!-- breadcrumbs:end -->

# Spec

Canonical project contract for the `word-frequency-finder` project.

## Goal

Build a small project app that introduces:

- reading a text file
- splitting file contents into words
- counting exact word frequencies
- preserving first-appearance order for the histogram
- formatting the histogram as canonical output lines
- test-first tokenization, counting, and formatting logic in small core functions
- thin surface adapters

## Canonical Input And Output Example

Every tutorial run for this project should preserve this canonical behavior:

Input text:

```text
badger badger badger badger mushroom mushroom
snake badger badger badger
```

Output lines:

```text
badger: *******
mushroom: **
snake: *
```

## Core Logic Contract

The shared core contracts are:

```text
split_text_into_words(text: string) -> string[]
count_word_frequencies(words: string[]) -> word_frequency_entries
format_word_frequency_histogram(word_frequency_entries) -> string[]
```

Where each `word_frequency_entry` contains:

```text
word
count
```

Canonical behavior:

- `split_text_into_words`:
  - treats one or more ASCII whitespace characters as separators
  - preserves each token exactly as it appears between separators
  - does not trim, normalize, or case-fold word text
  - returns words in source order
- `count_word_frequencies`:
  - counts exact word matches
  - preserves the order of first appearance for the resulting entries
  - returns one entry per distinct exact word
- `format_word_frequency_histogram` returns one line per entry in this exact format:
  - `<word>: <asterisks>`
  - where the number of `*` characters exactly equals `count`

Examples:

- `split_text_into_words("badger badger\nsnake")` returns `["badger", "badger", "snake"]`
- `count_word_frequencies(["badger", "badger", "mushroom", "snake", "badger"])` returns entries in this order:
  - `{ word: "badger", count: 3 }`
  - `{ word: "mushroom", count: 1 }`
  - `{ word: "snake", count: 1 }`
- `format_word_frequency_histogram([{ word: "badger", count: 3 }, { word: "snake", count: 1 }])` returns:
  - `badger: ***`
  - `snake: *`
- `format_word_frequency_histogram(count_word_frequencies(split_text_into_words("badger badger badger badger mushroom mushroom\nsnake badger badger badger")))` returns:
  - `badger: *******`
  - `mushroom: **`
  - `snake: *`

## Non-Goals

This project does not include:

- localization
- authentication
- network access
- configuration files
- case-insensitive counting
- punctuation stripping
- alphabetical sorting of histogram lines
- stemming or singular/plural normalization
- writing the histogram to an output file

## Surface Expectations

This spec follows the shared surface and setup-path rules in [Projects](../../README.md#shared-spec-conventions).

For this project, every tutorial run should adapt the same tokenization, exact-match frequency counting, first-appearance ordering, and histogram formatting behavior instead of redefining it.

Adapters should:

- read one source text file
- pass the full file contents to `split_text_into_words`
- pass the returned words to `count_word_frequencies`
- pass the resulting entries to `format_word_frequency_histogram`
- render the returned lines in order

## Output Repository Expectations

This project follows the shared output model in [Projects](../../README.md#shared-output-model).

For this project, the core repo owns `split_text_into_words`, `count_word_frequencies`, and `format_word_frequency_histogram`, and adapter repos must not reimplement them.

## Testing And Coverage Contract

This spec follows the shared coverage policy in [Projects](../../README.md#shared-coverage-policy).

This project should be built in a spec-driven and test-driven way.

Minimum test expectations:

- a test that `split_text_into_words` handles spaces and newlines in the canonical example
- a test that `count_word_frequencies` preserves first-appearance order for distinct words
- a test that `count_word_frequencies` counts repeated exact words correctly
- a test that `format_word_frequency_histogram` preserves the exact `word: stars` line format
- a test that the canonical input example produces the canonical histogram output
- tests for every adapter built in the chosen run that prove it reads the source file, delegates tokenization, counting, and formatting to the core logic, and renders the histogram lines correctly
