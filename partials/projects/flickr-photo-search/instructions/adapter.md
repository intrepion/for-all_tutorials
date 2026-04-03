---
partial_kind: adapter-instructions
project: flickr-photo-search
---

# Adapter Instructions

Project-specific adapter instruction fragment for the `flickr-photo-search` adapter repo.

### 1. Red: Add An Adapter-Level Test

In a separate adapter repo, add a failing test that proves the chosen graphical adapter collects the search string, performs the HTTP GET request, delegates response parsing and display-item building to the core logic, and renders the returned photographs in order.

### 2. Green: Add The Thin Surface Adapter

Add the thinnest possible adapter for the graphical surface you are building:

- collect one search string through the chosen graphical interface
- make an HTTP GET request to `https://www.flickr.com/services/feeds/photos_public.gne`
- pass the search string with the `tags` query parameter
- request JSON feed output with `format=json`
- pass the response body to `parse_flickr_public_feed_response`
- pass the parsed results to `build_photo_display_items`
- render the returned display items in source order with visible photographs
- keep HTTP transport and graphical rendering code out of the core parsing and display-item logic

### 3. Refactor

Clean up any remaining duplication while keeping the full suite green.

