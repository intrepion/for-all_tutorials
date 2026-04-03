---
partial_kind: core-instructions
project: flickr-photo-search
---

# Core Instructions

Project-specific core instruction fragment for the `flickr-photo-search` core repo.

### 1. Red: Write The First Failing Test

Start with parsing the canonical sample feed response:

```text
given the canonical sample Flickr public feed response json
when parse_flickr_public_feed_response is called
then the returned photo results preserve the exact ordered titles, photo-page URLs, and image URLs
```

At this point, `parse_flickr_public_feed_response` may not exist yet. That is fine. The first run should fail either at compile time or at test execution time.

Suggested generic test name:

```text
parse_flickr_public_feed_response_preserves_the_canonical_photo_results
```

### 2. Green: Make The First Test Pass

Create the smallest production code that makes the first test pass.

At this stage, it is acceptable if the code only handles the canonical sample parsing case. Do not jump ahead and implement display-item behavior before those tests exist.

### 3. Refactor

Do the smallest cleanup that keeps the first test green.

### 4. Red: Add Display-Item Construction

Write the next failing test:

```text
given the canonical parsed Flickr photo results
when build_photo_display_items is called
then the returned display items preserve the exact canonical titles, photo-page URLs, and image URLs in source order
```

Suggested generic test name:

```text
build_photo_display_items_preserves_the_canonical_photo_display_items
```

### 5. Green: Make The Display Test Pass

Make the smallest safe change that gets both tests green.

### 6. Refactor The Core Logic

Now that the full behavior is covered, refactor toward a small, clear core abstraction.

The exact shape can vary by stack:

- two functions in one module
- one service object with parser and display-builder methods
- one module plus small immutable result and display-item types

The important thing is that response parsing and display-item construction stay small, explicit, and directly testable.

