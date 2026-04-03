<!-- breadcrumbs:start -->
[for-all_tutorials](../../../../README.md) / [Projects](../../../README.md) / [Movie Recommendations](../../README.md) / [Tutorial](../README.md) / adapter.md
<!-- breadcrumbs:end -->

# Adapter Walkthrough

Project-specific red, green, refactor sequence for the `movie-recommendations` adapter repo.

### 1. Red: Add An Adapter-Level Test

In a separate adapter repo, add a failing test that proves the chosen adapter collects the movie query, retrieves one movie-details response from the chosen approved provider, delegates parsing, formatting, and recommendation to the core logic, and renders only the recommendation line that applies to the returned audience score.

### 2. Green: Add The Thin Surface Adapter

Add the thinnest possible adapter for the surface you are building:

- prompt for one movie search query
- use an approved movie-data provider that returns fields equivalent to the canonical movie-details response
- when the chosen implementation has approved Rotten Tomatoes-compatible access, use that provider
- pass the response body to `parse_movie_details_response`
- pass the parsed details to `format_movie_details_report`
- pass `audience_score_percent` to `build_audience_recommendation`
- render the returned report lines in order
- render the recommendation line only when the core returns a non-null recommendation
- keep provider access, authentication, and transport code out of the core parsing, formatting, and recommendation logic

### 3. Refactor

Clean up any remaining duplication while keeping the full suite green.
