<!-- breadcrumbs:start -->
[for-all_tutorials](../../../README.md) / [Projects](../../README.md) / [URL Shortener](../README.md) / Spec
<!-- breadcrumbs:end -->

# Spec

Canonical project contract for the `url-shortener` project.

## Goal

Build a small project app that introduces:

- accepting one long URL from a web form
- turning an adapter-chosen short code into a local short path
- building a short-link record with an initial visit count
- incrementing the visit count when the short path is visited
- formatting a deterministic statistics report for the short link
- test-first short-link and formatting logic in small core functions
- thin full-stack web adapters

## Canonical Sample Short-Link Record

For deterministic tests and examples, use a short-link record equivalent to:

```text
short_code = "abc1234"
long_url = "https://example.com/articles/invert-binary-trees"
visit_count = 2
```

## Core Logic Contract

The shared core contracts are:

```text
build_short_link_record(short_code: string, long_url: string) -> short_link_record
format_short_link_path(short_code: string) -> string
increment_short_link_visit_count(short_link_record) -> short_link_record
format_short_link_stats(short_link_record) -> string[]
```

Where `short_link_record` contains:

```text
short_code
long_url
visit_count
```

Canonical behavior:

- `build_short_link_record`:
  - preserves the exact `short_code`
  - preserves the exact `long_url`
  - sets `visit_count` to `0`
- `format_short_link_path` returns:
  - `/<short_code>`
- `increment_short_link_visit_count`:
  - returns a new short-link record
  - preserves the exact `short_code`
  - preserves the exact `long_url`
  - increases `visit_count` by `1`
- `format_short_link_stats` returns these lines in this exact order:
  - `Short URL: <format_short_link_path(short_code)>`
  - `Long URL: <long_url>`
  - `Visits: <visit_count>`

Examples:

- `build_short_link_record("abc1234", "https://example.com/articles/invert-binary-trees")` returns:
  - `{ short_code: "abc1234", long_url: "https://example.com/articles/invert-binary-trees", visit_count: 0 }`
- `format_short_link_path("abc1234")` returns:
  - `/abc1234`
- `increment_short_link_visit_count({ short_code: "abc1234", long_url: "https://example.com/articles/invert-binary-trees", visit_count: 2 })` returns:
  - `{ short_code: "abc1234", long_url: "https://example.com/articles/invert-binary-trees", visit_count: 3 }`
- `format_short_link_stats({ short_code: "abc1234", long_url: "https://example.com/articles/invert-binary-trees", visit_count: 2 })` returns:
  - `Short URL: /abc1234`
  - `Long URL: https://example.com/articles/invert-binary-trees`
  - `Visits: 2`

## Non-Goals

This project does not include:

- custom domains
- user accounts
- expiring links
- QR codes
- analytics beyond the one visit count
- partial-link search
- editing an existing short link
- distributed uniqueness guarantees across multiple servers

## Surface Expectations

This spec follows the shared surface and setup-path rules in [Projects](../../README.md#shared-spec-conventions).

For this project, every tutorial run should use a full-stack web adapter and adapt the same short-link, visit-count, and stats behavior instead of redefining it.

Adapters should:

- use a web full-stack surface
- render a form that accepts one long URL
- generate a unique short code for a new record
- pass the short code and long URL to `build_short_link_record`
- persist the resulting short-link record in a permanent data store
- render or expose the resulting short path using `format_short_link_path`
- when `/<short_code>` is visited:
  - load the matching short-link record from storage
  - pass it to `increment_short_link_visit_count`
  - persist the updated record
  - redirect the visitor to the stored `long_url`
- when `/<short_code>/stats` is visited:
  - load the matching short-link record from storage
  - pass it to `format_short_link_stats`
  - render the returned lines in the form the web surface requires

## Output Repository Expectations

This project follows the shared output model in [Projects](../../README.md#shared-output-model).

For this project, the core repo owns `build_short_link_record`, `format_short_link_path`, `increment_short_link_visit_count`, and `format_short_link_stats`, and adapter repos must not reimplement them.

## Testing And Coverage Contract

This spec follows the shared coverage policy in [Projects](../../README.md#shared-coverage-policy).

This project should be built in a spec-driven and test-driven way.

Minimum test expectations:

- a test that `build_short_link_record` preserves the exact short code and long URL while initializing `visit_count` to `0`
- a test that `format_short_link_path("abc1234")` returns `/abc1234`
- a test that `increment_short_link_visit_count` preserves the exact short code and long URL while incrementing the count
- a test that `format_short_link_stats` preserves the exact canonical three-line stats output
- tests for every adapter built in the chosen run that prove it accepts the long URL from a form, persists the short-link record, redirects from the short path, increments and persists the visit count on each visit, and renders the stats page correctly
