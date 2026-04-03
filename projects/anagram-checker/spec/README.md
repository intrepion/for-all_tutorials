<!-- breadcrumbs:start -->
[for-all_tutorials](../../../README.md) / [Projects](../../README.md) / [Anagram Checker](../README.md) / Spec
<!-- breadcrumbs:end -->

# Spec

Canonical project contract for the `anagram-checker` project.

## Goal

Build a small project app that introduces:

- comparing two text inputs by exact character content
- separating anagram detection from output formatting
- preserving both original inputs exactly in the displayed result
- test-first comparison and formatting logic in small core functions
- thin surface adapters

## Core Logic Contract

The shared core contracts are:

```text
are_anagrams(first_input: string, second_input: string) -> boolean
format_anagram_result(first_input: string, second_input: string, are_anagrams: boolean) -> string
```

Canonical behavior:

- treat both inputs as plain-text strings
- return `true` only when both inputs contain the same characters with the same counts
- return `false` when the inputs have different lengths
- return `false` when any character count differs
- treat uppercase and lowercase letters as different characters
- treat spaces, punctuation, and other visible ASCII symbols as significant characters
- do not trim, lowercase, sort away, or otherwise normalize either input before comparison
- preserve both original inputs exactly in the formatted output
- return `"<first_input>" and "<second_input>" are anagrams.` when `are_anagrams` is `true`
- return `"<first_input>" and "<second_input>" are not anagrams.` when `are_anagrams` is `false`

Examples:

- `are_anagrams("note", "tone")` returns `true`
- `are_anagrams("aab", "aba")` returns `true`
- `are_anagrams("note", "tones")` returns `false`
- `are_anagrams("note", "tore")` returns `false`
- `are_anagrams("aab", "abb")` returns `false`
- `are_anagrams("Note", "tone")` returns `false`
- `format_anagram_result("note", "tone", true)` returns `"note" and "tone" are anagrams.`
- `format_anagram_result("note", "tore", false)` returns `"note" and "tore" are not anagrams.`

## Non-Goals

This project does not include:

- persistence
- localization
- authentication
- network access
- configuration files
- case-insensitive matching
- whitespace-insensitive matching
- punctuation-insensitive matching
- Unicode normalization or grapheme-cluster comparison

## Surface Expectations

This spec follows the shared surface and setup-path rules in [Projects](../../README.md#shared-spec-conventions).

For this project, every tutorial run should adapt the same anagram-checking and result-formatting behavior instead of redefining it.

Adapters should collect two strings, pass them to `are_anagrams`, and pass the resulting boolean into `format_anagram_result`.

## Output Repository Expectations

This project follows the shared output model in [Projects](../../README.md#shared-output-model).

For this project, the core repo owns `are_anagrams` and `format_anagram_result`, and adapter repos must not reimplement them.

## Testing And Coverage Contract

This spec follows the shared coverage policy in [Projects](../../README.md#shared-coverage-policy).

This project should be built in a spec-driven and test-driven way.

Minimum test expectations:

- tests for a known anagram pair such as `note` and `tone`
- tests for a different-length pair such as `note` and `tones`
- tests for a same-length non-anagram pair such as `note` and `tore`
- tests that prove repeated-character counts matter, such as `aab` and `abb`
- tests that prove case-sensitive comparison, such as `Note` and `tone`
- tests that `format_anagram_result` preserves the exact success sentence
- tests that `format_anagram_result` preserves the exact failure sentence
- tests for every adapter built in the chosen run that prove it delegates to the core logic correctly
