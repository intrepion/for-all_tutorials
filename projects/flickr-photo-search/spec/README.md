<!-- breadcrumbs:start -->
[for-all_tutorials](../../../README.md) / [Projects](../../README.md) / [Flickr Photo Search](../README.md) / Spec
<!-- breadcrumbs:end -->

# Spec

Canonical project contract for the `flickr-photo-search` project.

## Goal

Build a small project app that introduces:

- collecting one search string
- fetching Flickr public photo feed results for that search
- parsing feed items into structured photo records
- preserving the source order of returned photos
- building deterministic photo display items from the parsed response
- test-first parsing and display-item logic in small core functions
- thin graphical surface adapters

## Canonical Sample API Response

For deterministic tests and examples, use a response equivalent to this JSON document:

```json
{
  "title": "Recent Uploads tagged kittens",
  "items": [
    {
      "title": "Tabby On A Blanket",
      "link": "https://www.flickr.com/photos/example-user/1/",
      "media": {
        "m": "https://live.staticflickr.com/example/tabby-on-a-blanket.jpg"
      }
    },
    {
      "title": "Sleeping Kitten",
      "link": "https://www.flickr.com/photos/example-user/2/",
      "media": {
        "m": "https://live.staticflickr.com/example/sleeping-kitten.jpg"
      }
    }
  ]
}
```

## Core Logic Contract

The shared core contracts are:

```text
parse_flickr_public_feed_response(json_text: string) -> flickr_photo_results
build_photo_display_items(flickr_photo_results) -> photo_display_item[]
```

Where `flickr_photo_results` contains:

```text
photos
```

and each parsed photo contains:

```text
title
photo_page_url
image_url
```

and each `photo_display_item` contains:

```text
title
photo_page_url
image_url
```

Canonical behavior:

- `parse_flickr_public_feed_response`:
  - parses the JSON document
  - preserves the source order of the `items` array
  - reads each item `title` into `title`
  - reads each item `link` into `photo_page_url`
  - reads each item `media.m` into `image_url`
  - ignores other top-level and item fields after successful parsing
- `build_photo_display_items`:
  - returns one display item for each parsed photo
  - preserves the source order of the parsed photos
  - preserves the exact `title`, `photo_page_url`, and `image_url` values from the parsed photos

Examples:

- `parse_flickr_public_feed_response(canonical_json_text)` returns photos in this order:
  - `{ title: "Tabby On A Blanket", photo_page_url: "https://www.flickr.com/photos/example-user/1/", image_url: "https://live.staticflickr.com/example/tabby-on-a-blanket.jpg" }`
  - `{ title: "Sleeping Kitten", photo_page_url: "https://www.flickr.com/photos/example-user/2/", image_url: "https://live.staticflickr.com/example/sleeping-kitten.jpg" }`
- `build_photo_display_items(parse_flickr_public_feed_response(canonical_json_text))` returns display items in this order:
  - `{ title: "Tabby On A Blanket", photo_page_url: "https://www.flickr.com/photos/example-user/1/", image_url: "https://live.staticflickr.com/example/tabby-on-a-blanket.jpg" }`
  - `{ title: "Sleeping Kitten", photo_page_url: "https://www.flickr.com/photos/example-user/2/", image_url: "https://live.staticflickr.com/example/sleeping-kitten.jpg" }`

## Non-Goals

This project does not include:

- authentication
- uploads or editing Flickr content
- pagination or infinite scrolling
- favorites, comments, or lightbox behavior
- caching search results
- sorting or filtering the returned items after the feed response arrives
- rewriting image URLs into alternate sizes
- text-only adapters

## Surface Expectations

This spec follows the shared surface and setup-path rules in [Projects](../../README.md#shared-spec-conventions).

For this project, every tutorial run should adapt the same feed-parsing and display-item rules instead of redefining them.

Adapters should:

- use a graphical-capable surface
- collect one search string through the chosen graphical interface
- make an HTTP GET request to Flickr Public Feed:
  - `https://www.flickr.com/services/feeds/photos_public.gne`
- pass the search string with the `tags` query parameter
- request JSON feed output by passing `format=json`
- pass the response body to `parse_flickr_public_feed_response`
- pass the parsed results to `build_photo_display_items`
- render the returned display items in source order
- visually display the returned photographs instead of showing only raw URLs

A text-only adapter is not sufficient for this project.

## Output Repository Expectations

This project follows the shared output model in [Projects](../../README.md#shared-output-model).

For this project, the core repo owns `parse_flickr_public_feed_response` and `build_photo_display_items`, and adapter repos must not reimplement them.

## Testing And Coverage Contract

This spec follows the shared coverage policy in [Projects](../../README.md#shared-coverage-policy).

This project should be built in a spec-driven and test-driven way.

Minimum test expectations:

- a test that `parse_flickr_public_feed_response` preserves the exact canonical titles, photo-page URLs, and image URLs
- a test that `parse_flickr_public_feed_response` preserves the canonical source order of the returned photos
- a test that `build_photo_display_items` preserves the exact canonical display items in order
- tests for every adapter built in the chosen run that prove it collects the search string, calls the feed with `tags` and `format=json`, delegates parsing and display-item building to the core logic, and renders actual photographs in a graphical surface
