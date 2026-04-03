<!-- breadcrumbs:start -->
[for-all_tutorials](../../../../README.md) / [Projects](../../../README.md) / [Text Sharing](../../README.md) / [Tutorial](../README.md) / core.md
<!-- breadcrumbs:end -->

# Core Walkthrough

Project-specific red, green, refactor sequence for the `text-sharing` core repo.

### 1. Red: Write The First Failing Test

Start with building one snippet record:

```text
given a snippet ID and snippet text
when build_text_snippet_record is called
then the returned snippet record preserves the exact ID and text
```

At this point, `build_text_snippet_record` may not exist yet. That is fine. The first run should fail either at compile time or at test execution time.

Suggested generic test name:

```text
build_text_snippet_record_preserves_the_exact_id_and_text
```

### 2. Green: Make The First Test Pass

Create the smallest production code that makes the first test pass.

At this stage, it is acceptable if the code only handles this one record-building case. Do not jump ahead and implement path formatting, view formatting, or edit prefill behavior before those tests exist.

### 3. Refactor

Do the smallest cleanup that keeps the first test green.

### 4. Red: Add Share And Edit Paths

Write the next failing tests:

```text
given the snippet ID abc1234
when format_text_snippet_path is called
then /abc1234 is returned
```

and

```text
given the snippet ID abc1234
when format_text_snippet_edit_path is called
then /abc1234/edit is returned
```

Suggested generic test names:

```text
format_text_snippet_path_returns_the_canonical_share_path
format_text_snippet_edit_path_returns_the_canonical_edit_path
```

### 5. Green: Make The Path Tests Pass

Make the smallest safe change that gets those tests green.

### 6. Red: Add View Formatting And Edit Seed

Write the next failing tests:

```text
given the canonical snippet record
when format_text_snippet_view is called
then the returned lines are:
Text: Learn how to invert binary trees
Edit URL: /abc1234/edit
```

and

```text
given the canonical snippet record
when build_text_snippet_edit_seed is called
then the exact canonical snippet text is returned
```

Suggested generic test names:

```text
format_text_snippet_view_returns_the_canonical_view_lines
build_text_snippet_edit_seed_returns_the_exact_snippet_text
```

### 7. Green: Make The View And Edit Tests Pass

Make the smallest safe change that gets the full suite green.

### 8. Refactor The Core Logic

Now that the full behavior is covered, refactor toward a small, clear core abstraction.

The exact shape can vary by stack:

- five functions in one module
- one service object with build, path, view, and edit-seed methods
- one module plus a small immutable snippet-record type

The important thing is that snippet construction, path formatting, view formatting, and edit-prefill logic stay small, explicit, and directly testable.
