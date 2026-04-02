<!-- breadcrumbs:start -->
[for-all_tutorials](../../../README.md) / [Projects](../../README.md) / [Printing Quotes](../README.md) / Spec
<!-- breadcrumbs:end -->

# Spec

Canonical project contract for the `printing-quotes` project.

## Goal

Build a small project app that introduces:

- combining two plain-text inputs into one final sentence
- formatting output with literal double quotation marks
- preserving author and quote text exactly as entered
- test-first formatting logic in one small core function
- thin surface adapters

## Core Logic Contract

The shared core contract is:

```text
format_attributed_quote(author: string, quote: string) -> string
```

Canonical behavior:

- preserve `author` exactly as entered
- preserve `quote` exactly as entered
- return `<author> says, "<quote>"`
- include the literal word `says`, followed by a comma and a space
- wrap `quote` in literal double quotation marks
- do not add, remove, or normalize punctuation inside `quote`

Examples:

- `format_attributed_quote("Obi-Wan Kenobi", "These aren't the droids you're looking for.")` returns `Obi-Wan Kenobi says, "These aren't the droids you're looking for."`
- `format_attributed_quote("Ada", "Hello")` returns `Ada says, "Hello"`
- `format_attributed_quote("Grace Hopper", "It's easier to ask forgiveness than it is to get permission.")` returns `Grace Hopper says, "It's easier to ask forgiveness than it is to get permission."`

## Non-Goals

This project does not include:

- persistence
- localization
- authentication
- network access
- configuration files
- validation for missing author or quote
- escaping or sanitizing embedded double-quote characters inside the user-provided quote
- multiple quotes, history, or editing workflows

## Surface Expectations

This spec follows the shared surface and setup-path rules in [Projects](../../README.md#shared-spec-conventions).

For this project, every tutorial run should adapt the same quote-formatting behavior instead of redefining it.

## Output Repository Expectations

This project follows the shared output model in [Projects](../../README.md#shared-output-model).

For this project, the core repo owns `format_attributed_quote`, and adapter repos must not reimplement it.

## Testing And Coverage Contract

This spec follows the shared coverage policy in [Projects](../../README.md#shared-coverage-policy).

This project should be built in a spec-driven and test-driven way.

Minimum test expectations:

- tests for basic author-plus-quote formatting
- tests that apostrophes and punctuation inside the quote are preserved exactly
- tests that the formatter wraps the quote in literal double quotation marks
- tests that the formatter does not invent punctuation when the quote text does not contain it
- tests for every adapter built in the chosen run that prove it delegates to the core logic correctly
