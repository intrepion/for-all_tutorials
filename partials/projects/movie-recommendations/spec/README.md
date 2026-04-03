---
partial_kind: project-spec
project: movie-recommendations
---

# Spec

Canonical project contract for the `movie-recommendations` project.

## Goal

Build a small project app that introduces:

- collecting one movie search query
- retrieving one movie details record from an approved movie-data provider
- parsing that response into structured movie details
- formatting a multi-line movie report
- generating a recommendation from audience-score thresholds
- test-first parsing, formatting, and recommendation logic in small core functions
- thin surface adapters

## Canonical Sample API Response

For deterministic tests and examples, use a response equivalent to this JSON document:

```json
{
  "title": "Guardians of the Galaxy",
  "year": 2014,
  "mpaa_rating": "PG-13",
  "runtime": 121,
  "synopsis": "From Marvel...",
  "ratings": {
    "audience_score": 92
  }
}
```

## Core Logic Contract

The shared core contracts are:

```text
parse_movie_details_response(json_text: string) -> movie_details
format_movie_details_report(movie_details) -> string[]
build_audience_recommendation(audience_score_percent: integer) -> string | null
```

Where `movie_details` contains:

```text
title
year
rating
running_time_minutes
description
audience_score_percent
```

Canonical behavior:

- `parse_movie_details_response`:
  - parses the JSON document
  - reads `title` into `title`
  - reads `year` into `year`
  - reads `mpaa_rating` into `rating`
  - reads `runtime` into `running_time_minutes`
  - reads `synopsis` into `description`
  - reads `ratings.audience_score` into `audience_score_percent`
- `format_movie_details_report` returns these lines in this exact order:
  - `Title: <title>`
  - `Year: <year>`
  - `Rating: <rating>`
  - `Running Time: <running_time_minutes> minutes`
  - `Description: <description>`
- `build_audience_recommendation`:
  - returns `You should watch this movie right now!` when `audience_score_percent` is greater than `80`
  - returns `You should avoid this movie at all costs.` when `audience_score_percent` is less than `50`
  - returns `null` for scores from `50` through `80`, inclusive

Examples:

- `parse_movie_details_response(canonical_json_text)` returns:
  - `title = "Guardians of the Galaxy"`
  - `year = 2014`
  - `rating = "PG-13"`
  - `running_time_minutes = 121`
  - `description = "From Marvel..."`
  - `audience_score_percent = 92`
- `format_movie_details_report(parse_movie_details_response(canonical_json_text))` returns:
  - `Title: Guardians of the Galaxy`
  - `Year: 2014`
  - `Rating: PG-13`
  - `Running Time: 121 minutes`
  - `Description: From Marvel...`
- `build_audience_recommendation(92)` returns `You should watch this movie right now!`
- `build_audience_recommendation(45)` returns `You should avoid this movie at all costs.`
- `build_audience_recommendation(65)` returns `null`

## Non-Goals

This project does not include:

- scraping the public Rotten Tomatoes website
- user accounts
- saving favorites or watchlists
- multiple-movie result lists in the core
- trailers, cast lists, or review excerpts
- sorting or ranking search results in the core
- caching provider responses
- localization

## Surface Expectations

This spec follows the shared surface and setup-path rules in [Projects](../../README.md#shared-spec-conventions).

For this project, every tutorial run should adapt the same movie-details parsing, report formatting, and audience recommendation behavior instead of redefining them.

Adapters should:

- prompt for one movie search query
- use an approved movie-data provider that returns fields equivalent to the canonical movie-details response
- when approved Rotten Tomatoes-compatible access is available in the chosen implementation, use that provider
- pass the provider response body to `parse_movie_details_response`
- pass the parsed details to `format_movie_details_report`
- pass `audience_score_percent` to `build_audience_recommendation`
- render the report lines in order
- render the recommendation line only when `build_audience_recommendation` returns a non-null value

For prompt-driven adapters, the prompt should be equivalent to:

```text
Enter the name of a movie:
```

## Output Repository Expectations

This project follows the shared output model in [Projects](../../README.md#shared-output-model).

For this project, the core repo owns `parse_movie_details_response`, `format_movie_details_report`, and `build_audience_recommendation`, and adapter repos must not reimplement them.

## Testing And Coverage Contract

This spec follows the shared coverage policy in [Projects](../../README.md#shared-coverage-policy).

This project should be built in a spec-driven and test-driven way.

Minimum test expectations:

- a test that `parse_movie_details_response` preserves the exact canonical title, year, rating, running time, description, and audience score
- a test that `format_movie_details_report` preserves the exact canonical five-line report
- a test that `build_audience_recommendation` returns the watch-now message for a score above `80`
- a test that `build_audience_recommendation` returns the avoid message for a score below `50`
- a test that `build_audience_recommendation` returns `null` for a middle-range score
- tests for every adapter built in the chosen run that prove it collects the movie query, delegates parsing, formatting, and recommendation to the core logic, and renders only the appropriate recommendation line for the returned audience score

