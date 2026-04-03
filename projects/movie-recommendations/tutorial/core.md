<!-- breadcrumbs:start -->
[for-all_tutorials](../../../../README.md) / [Projects](../../../README.md) / [Movie Recommendations](../../README.md) / [Tutorial](../README.md) / core.md
<!-- breadcrumbs:end -->

# Core Walkthrough

Project-specific red, green, refactor sequence for the `movie-recommendations` core repo.

### 1. Red: Write The First Failing Test

Start with parsing the canonical sample movie-details response:

```text
given the canonical movie-details response json
when parse_movie_details_response is called
then the returned movie details preserve the exact canonical title, year, rating, running time, description, and audience score
```

At this point, `parse_movie_details_response` may not exist yet. That is fine. The first run should fail either at compile time or at test execution time.

Suggested generic test name:

```text
parse_movie_details_response_preserves_the_canonical_movie_details
```

### 2. Green: Make The First Test Pass

Create the smallest production code that makes the first test pass.

At this stage, it is acceptable if the code only handles the canonical sample parsing case. Do not jump ahead and implement formatter or recommendation behavior before those tests exist.

### 3. Refactor

Do the smallest cleanup that keeps the first test green.

### 4. Red: Add The Report Formatter

Write the next failing test:

```text
given the canonical parsed movie details
when format_movie_details_report is called
then the returned lines are:
Title: Guardians of the Galaxy
Year: 2014
Rating: PG-13
Running Time: 121 minutes
Description: From Marvel...
```

Suggested generic test name:

```text
format_movie_details_report_returns_the_canonical_report
```

### 5. Green: Make The Formatter Test Pass

Make the smallest safe change that gets both tests green.

### 6. Red: Add The Recommendation Rules

Write the next failing tests:

```text
given an audience score above 80
when build_audience_recommendation is called
then the watch-now message is returned
```

and

```text
given an audience score below 50
when build_audience_recommendation is called
then the avoid message is returned
```

and

```text
given an audience score from 50 through 80
when build_audience_recommendation is called
then null is returned
```

Suggested generic test names:

```text
build_audience_recommendation_returns_watch_now_for_scores_above_80
build_audience_recommendation_returns_avoid_for_scores_below_50
build_audience_recommendation_returns_null_for_middle_scores
```

### 7. Green: Make The Recommendation Tests Pass

Make the smallest safe change that gets the full suite green.

### 8. Refactor The Core Logic

Now that the full behavior is covered, refactor toward a small, clear core abstraction.

The exact shape can vary by stack:

- three functions in one module
- one service object with parser, formatter, and recommendation methods
- one module plus a small immutable movie-details type

The important thing is that response parsing, report formatting, and recommendation rules stay small, explicit, and directly testable.
