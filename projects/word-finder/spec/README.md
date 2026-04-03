<!-- breadcrumbs:start -->
[for-all_tutorials](../../../README.md) / [Projects](../../README.md) / [Word Finder](../README.md) / Spec
<!-- breadcrumbs:end -->

# Spec

Canonical project contract for the `word-finder` project.

## Goal

Build a small project app that introduces:

- reading a source text file
- finding every exact lowercase whole-word occurrence of `utilize`
- replacing each of those occurrences with `use`
- preserving all other characters exactly
- writing the transformed contents to a new file
- test-first text-transformation logic in a small core function
- thin surface adapters

## Canonical Input And Output Example

Every tutorial run for this project should preserve this canonical replacement behavior:

Input text:

```text
One should never utilize the word "utilize" in
writing. Use "use" instead.
```

Output text:

```text
One should never use the word "use" in
writing. Use "use" instead.
```

## Core Logic Contract

The shared core contract is:

```text
replace_exact_word_utilize_with_use(text: string) -> string
```

Canonical behavior:

- treat `text` as plain text
- replace every exact lowercase whole-word occurrence of `utilize` with `use`
- preserve all other characters exactly, including spaces, punctuation, quotation marks, and line breaks
- treat `utilize` as a whole word when it is bounded on both sides by:
  - the start or end of the text, or
  - a non-letter character
- do not replace uppercase or mixed-case variants such as `Utilize` or `UTILIZE`
- do not replace `utilize` when it appears inside a longer word such as `utilized` or `disutilize`

Examples:

- `replace_exact_word_utilize_with_use("utilize")` returns `use`
- `replace_exact_word_utilize_with_use("\"utilize\"")` returns `"use"`
- `replace_exact_word_utilize_with_use("Use utilize instead.")` returns `Use use instead.`
- `replace_exact_word_utilize_with_use("Utilize utilize utilized")` returns `Utilize use utilized`
- `replace_exact_word_utilize_with_use("One should never utilize the word \"utilize\" in\nwriting. Use \"use\" instead.")` returns:
  - `One should never use the word "use" in`
  - `writing. Use "use" instead.`

## Non-Goals

This project does not include:

- localization
- authentication
- network access
- configuration files
- case-insensitive replacement
- replacing other words besides `utilize`
- grammar correction beyond the exact replacement rule
- choosing a canonical output file path
- replacing text in place in the source file

## Surface Expectations

This spec follows the shared surface and setup-path rules in [Projects](../../README.md#shared-spec-conventions).

For this project, every tutorial run should adapt the same exact replacement rule instead of redefining it.

Adapters should:

- read one source text file
- pass the full file contents to `replace_exact_word_utilize_with_use`
- write the returned transformed contents to a new output file

The exact source-file and output-file path selection is adapter-defined for this project because the exercise requires a new file but does not define canonical file names.

## Output Repository Expectations

This project follows the shared output model in [Projects](../../README.md#shared-output-model).

For this project, the core repo owns `replace_exact_word_utilize_with_use`, and adapter repos must not reimplement it.

## Testing And Coverage Contract

This spec follows the shared coverage policy in [Projects](../../README.md#shared-coverage-policy).

This project should be built in a spec-driven and test-driven way.

Minimum test expectations:

- a test that the canonical two-line example input transforms into the canonical two-line example output
- a test that quoted `utilize` becomes `"use"`
- a test that `Utilize` is not replaced
- a test that `utilized` or `disutilize` is not replaced
- tests for every adapter built in the chosen run that prove it reads a source file, delegates the transformation to the core logic, and writes the transformed contents to a new output file
