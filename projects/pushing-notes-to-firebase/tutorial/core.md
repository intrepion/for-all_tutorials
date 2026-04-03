<!-- breadcrumbs:start -->
[for-all_tutorials](../../../../README.md) / [Projects](../../../README.md) / [Pushing Notes to Firebase](../../README.md) / [Tutorial](../README.md) / core.md
<!-- breadcrumbs:end -->

# Core Walkthrough

Project-specific red, green, refactor sequence for the `pushing-notes-to-firebase` core repo.

### 1. Red: Write The First Failing Test

Start with building one note record:

```text
given a note text and a created_on date
when build_note_record is called
then the returned note record preserves the exact text and date
```

At this point, `build_note_record` may not exist yet. That is fine. The first run should fail either at compile time or at test execution time.

Suggested generic test name:

```text
build_note_record_preserves_the_exact_text_and_date
```

### 2. Green: Make The First Test Pass

Create the smallest production code that makes the first test pass.

At this stage, it is acceptable if the code only handles this one record-construction case. Do not jump ahead and implement collection parsing or formatting before those tests exist.

### 3. Refactor

Do the smallest cleanup that keeps the first test green.

### 4. Red: Add Collection Parsing

Write the next failing test:

```text
given the canonical keyed Firebase notes response json
when parse_notes_collection_response is called
then the returned notes preserve the exact canonical texts and dates in descending date order
```

Suggested generic test name:

```text
parse_notes_collection_response_preserves_the_canonical_notes_in_descending_date_order
```

### 5. Green: Make The Parsing Test Pass

Make the smallest safe change that gets both tests green.

### 6. Red: Add Message And Report Formatting

Write the next failing tests:

```text
given no input
when format_note_saved_message is called
then Your note was saved. is returned
```

and

```text
given the canonical parsed notes
when format_notes_report is called
then the returned lines are:
2050-12-31 - Learn how to invert binary trees
2050-12-30 - Notetaking on the command line is cool.
```

Suggested generic test names:

```text
format_note_saved_message_returns_the_canonical_saved_message
format_notes_report_returns_the_canonical_report_lines
```

### 7. Green: Make The Formatting Tests Pass

Make the smallest safe change that gets the full suite green.

### 8. Refactor The Core Logic

Now that the full behavior is covered, refactor toward a small, clear core abstraction.

The exact shape can vary by stack:

- four functions in one module
- one service object with build, parse, and format methods
- one module plus a small immutable note-record type

The important thing is that note construction, response parsing, ordering, and formatting stay small, explicit, and directly testable.
