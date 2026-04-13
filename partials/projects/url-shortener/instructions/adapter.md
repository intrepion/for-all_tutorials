---
partial_kind: adapter-instructions
project: url-shortener
---

# Adapter Instructions

Project-specific adapter instruction fragment for the `url-shortener` adapter repo.

### 1. Red: Add Adapter-Level Tests

In a separate adapter repo, add failing tests that prove the chosen adapter:

- accepts one long URL through the chosen surface
- generates a unique short code
- delegates short-link creation, path formatting, visit-count updates, and stats formatting to the core logic
- persists short-link records
- redirects from the short path to the stored long URL
- increments and persists the visit count on each short-path visit
- renders or returns the stats output correctly

### 2. Green: Add The Thin Adapter

Add the thinnest possible adapter:

- collect one long URL through the chosen surface
- generate a unique short code for a new record
- pass the short code and long URL to `build_short_link_record`
- persist the resulting record in a permanent data store
- use `format_short_link_path` when showing, linking to, or returning the short path
- when `/<short_code>` is resolved through the chosen surface:
  - load the matching record
  - pass it to `increment_short_link_visit_count`
  - persist the updated record
  - transfer control to the stored `long_url` in the way the chosen surface requires
- when `/<short_code>/stats` is resolved through the chosen surface:
  - load the matching record
  - pass it to `format_short_link_stats`
  - render or return the returned lines in the form the chosen surface requires
- keep routing, persistence, surface behavior, and unique-code generation code out of the core short-link logic

### 3. Refactor

Clean up any remaining duplication while keeping the full suite green.
