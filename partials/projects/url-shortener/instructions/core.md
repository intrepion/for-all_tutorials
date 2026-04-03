---
partial_kind: core-instructions
project: url-shortener
---

# Core Instructions

Project-specific core instruction fragment for the `url-shortener` core repo.

### 1. Red: Write The First Failing Test

Start with building one short-link record:

```text
given a short code and a long URL
when build_short_link_record is called
then the returned short-link record preserves the exact inputs and initializes the visit count to 0
```

At this point, `build_short_link_record` may not exist yet. That is fine. The first run should fail either at compile time or at test execution time.

Suggested generic test name:

```text
build_short_link_record_initializes_a_new_short_link
```

### 2. Green: Make The First Test Pass

Create the smallest production code that makes the first test pass.

At this stage, it is acceptable if the code only handles this one record-building case. Do not jump ahead and implement path formatting, stats, or visit-count updates before those tests exist.

### 3. Refactor

Do the smallest cleanup that keeps the first test green.

### 4. Red: Add Path Formatting And Visit Counting

Write the next failing tests:

```text
given the short code abc1234
when format_short_link_path is called
then /abc1234 is returned
```

and

```text
given a short-link record with visit_count 2
when increment_short_link_visit_count is called
then the returned record preserves the exact short code and long URL and has visit_count 3
```

Suggested generic test names:

```text
format_short_link_path_returns_the_canonical_local_path
increment_short_link_visit_count_returns_a_new_record_with_the_count_incremented
```

### 5. Green: Make The Path And Count Tests Pass

Make the smallest safe change that gets those tests green.

### 6. Red: Add Statistics Formatting

Write the next failing test:

```text
given the canonical short-link record
when format_short_link_stats is called
then the returned lines are:
Short URL: /abc1234
Long URL: https://example.com/articles/invert-binary-trees
Visits: 2
```

Suggested generic test name:

```text
format_short_link_stats_returns_the_canonical_stats_lines
```

### 7. Green: Make The Stats Test Pass

Make the smallest safe change that gets the full suite green.

### 8. Refactor The Core Logic

Now that the full behavior is covered, refactor toward a small, clear core abstraction.

The exact shape can vary by stack:

- four functions in one module
- one service object with build, path, increment, and stats methods
- one module plus a small immutable short-link-record type

The important thing is that short-link construction, path formatting, visit-count updates, and stats formatting stay small, explicit, and directly testable.

