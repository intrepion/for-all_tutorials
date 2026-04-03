<!-- breadcrumbs:start -->
[for-all_tutorials](../../../README.md) / [Projects](../../README.md) / [Computing Statistics](../README.md) / Spec
<!-- breadcrumbs:end -->

# Spec

Canonical project contract for the `computing-statistics` project.

## Goal

Build a small project app that introduces:

- collecting a dynamic list of numeric response times
- ending collection when the adapter receives the sentinel value `done`
- computing mean, minimum, maximum, and population standard deviation
- separating statistical calculations and output formatting from input parsing and control flow
- test-first statistics and formatting logic in small core functions
- thin surface adapters

## Core Logic Contract

The shared core contracts are:

```text
mean_hundredths(response_times_ms: integer[]) -> integer
minimum_response_time(response_times_ms: integer[]) -> integer
maximum_response_time(response_times_ms: integer[]) -> integer
standard_deviation_hundredths(response_times_ms: integer[]) -> integer
format_statistics_report(
  response_times_ms: integer[],
  mean_hundredths: integer,
  minimum_ms: integer,
  maximum_ms: integer,
  standard_deviation_hundredths: integer
) -> string[]
```

Canonical behavior:

- treat `response_times_ms` as a non-empty list of whole-number response times in milliseconds
- preserve the input order of `response_times_ms`
- `mean_hundredths`:
  - returns the arithmetic mean rounded to the nearest hundredth with half-up rounding
  - represents the rounded result as hundredths
- `minimum_response_time` returns the smallest value in the list
- `maximum_response_time` returns the largest value in the list
- `standard_deviation_hundredths`:
  - uses the population-standard-deviation formula described in the prompt:
    - compute the mean
    - compute the squared difference from the mean for each value
    - compute the mean of those squared differences
    - take the square root
  - returns the final value rounded to the nearest hundredth with half-up rounding
  - represents the rounded result as hundredths
- `format_statistics_report` returns these lines in this exact order:
  - `Numbers: <values joined by comma+space in original order>`
  - `The average is <formatted_mean>.`
  - `The minimum is <minimum_ms>.`
  - `The maximum is <maximum_ms>.`
  - `The standard deviation is <formatted_standard_deviation>.`
- when rendering the mean and standard deviation:
  - omit the decimal portion when the rounded value is a whole number
  - otherwise render exactly two decimal places

Examples:

- `mean_hundredths([100, 200, 1000, 300])` returns `40000`
- `minimum_response_time([100, 200, 1000, 300])` returns `100`
- `maximum_response_time([100, 200, 1000, 300])` returns `1000`
- `standard_deviation_hundredths([100, 200, 1000, 300])` returns `35355`
- `format_statistics_report([100, 200, 1000, 300], 40000, 100, 1000, 35355)` returns:
  - `Numbers: 100, 200, 1000, 300`
  - `The average is 400.`
  - `The minimum is 100.`
  - `The maximum is 1000.`
  - `The standard deviation is 353.55.`

## Non-Goals

This project does not include:

- persistence
- localization
- authentication
- network access
- configuration files
- parsing the sentinel value `done` in the core
- sample standard deviation
- empty input lists in the core
- decimal input parsing
- percentile or median calculations

## Surface Expectations

This spec follows the shared surface and setup-path rules in [Projects](../../README.md#shared-spec-conventions).

For this project, every tutorial run should adapt the same statistics and report-formatting behavior instead of redefining it.

Adapters should:

- prompt for response times repeatedly
- stop collecting values when the entered input is exactly `done`
- parse every collected response time into a whole number of milliseconds before calling the core
- preserve the collected order of response times
- not call the core until at least one numeric value has been collected
- call the core statistics functions
- pass the calculated values to `format_statistics_report`
- render the returned lines in order

For prompt-driven adapters, the repeated prompt should be equivalent to:

```text
Enter a number:
```

## Output Repository Expectations

This project follows the shared output model in [Projects](../../README.md#shared-output-model).

For this project, the core repo owns `mean_hundredths`, `minimum_response_time`, `maximum_response_time`, `standard_deviation_hundredths`, and `format_statistics_report`, and adapter repos must not reimplement them.

## Testing And Coverage Contract

This spec follows the shared coverage policy in [Projects](../../README.md#shared-coverage-policy).

This project should be built in a spec-driven and test-driven way.

Minimum test expectations:

- a test that `mean_hundredths([100, 200, 1000, 300])` returns `40000`
- a test that `minimum_response_time([100, 200, 1000, 300])` returns `100`
- a test that `maximum_response_time([100, 200, 1000, 300])` returns `1000`
- a test that `standard_deviation_hundredths([100, 200, 1000, 300])` returns `35355`
- a test that `format_statistics_report` preserves the exact canonical line order and sentences
- tests for every adapter built in the chosen run that prove it stops on the sentinel `done`, preserves entry order, and delegates to the core logic correctly
