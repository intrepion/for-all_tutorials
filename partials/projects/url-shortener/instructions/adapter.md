---
partial_kind: adapter-instructions
project: url-shortener
---

# Adapter Instructions

Project-specific adapter instruction fragment for the `url-shortener` adapter repo.

### 1. Red: Add Adapter-Level Tests

In a separate adapter repo, add failing tests that prove the chosen full-stack web adapter:

- accepts one long URL from a form
- generates a unique short code
- delegates short-link creation, path formatting, visit-count updates, and stats formatting to the core logic
- persists short-link records
- redirects from the short path to the stored long URL
- increments and persists the visit count on each short-path visit
- renders the stats page correctly

### 2. Green: Add The Thin Full-Stack Adapter

Add the thinnest possible full-stack web adapter:

- render a form that accepts one long URL
- generate a unique short code for a new record
- pass the short code and long URL to `build_short_link_record`
- persist the resulting record in a permanent data store
- use `format_short_link_path` when showing or linking to the short URL
- when `/<short_code>` is visited:
  - load the matching record
  - pass it to `increment_short_link_visit_count`
  - persist the updated record
  - redirect to the stored `long_url`
- when `/<short_code>/stats` is visited:
  - load the matching record
  - pass it to `format_short_link_stats`
  - render the returned lines in the form the web surface requires
- keep routing, persistence, redirect, and unique-code generation code out of the core short-link logic

### 3. Refactor

Clean up any remaining duplication while keeping the full suite green.

