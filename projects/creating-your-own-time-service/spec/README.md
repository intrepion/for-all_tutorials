<!-- breadcrumbs:start -->
[for-all_tutorials](../../../README.md) / [Projects](../../README.md) / [Creating Your Own Time Service](../README.md) / Spec
<!-- breadcrumbs:end -->

# Spec

Canonical project contract for the `creating-your-own-time-service` project.

## Goal

Build a small project app that introduces:

- exposing the current UTC time from a web service adapter
- returning that time in a fixed JSON shape
- consuming that JSON from a separate client adapter
- parsing the service response into a structured timestamp value
- formatting a human-readable UTC time message
- test-first payload, parsing, and formatting logic in small core functions
- thin service and client adapters

## Canonical Sample Service Response

For deterministic tests and examples, use a response equivalent to this JSON document:

```json
{
  "currentTime": "2050-01-04 15:06:26"
}
```

## Core Logic Contract

The shared core contracts are:

```text
build_current_time_payload(current_time_utc: string) -> current_time_payload
format_current_time_service_response(current_time_payload) -> string
parse_current_time_service_response(json_text: string) -> current_time_payload
format_current_time_display(current_time_payload) -> string
```

Where `current_time_payload` contains:

```text
current_time_utc
```

Canonical behavior:

- `build_current_time_payload`:
  - preserves the exact `current_time_utc`
  - returns a payload with that one value
- `format_current_time_service_response`:
  - returns a JSON string equivalent to:
    - `{"currentTime":"<current_time_utc>"}`
- `parse_current_time_service_response`:
  - parses the JSON document
  - reads `currentTime` into `current_time_utc`
- `format_current_time_display`:
  - converts the canonical `YYYY-MM-DD HH:MM:SS` UTC timestamp into this exact message shape:
    - `The current time is <HH:MM:SS> UTC <Month name> <day number> <year>.`

Examples:

- `build_current_time_payload("2050-01-04 15:06:26")` returns:
  - `{ current_time_utc: "2050-01-04 15:06:26" }`
- `format_current_time_service_response({ current_time_utc: "2050-01-04 15:06:26" })` returns JSON equivalent to:
  - `{"currentTime":"2050-01-04 15:06:26"}`
- `parse_current_time_service_response(canonical_json_text)` returns:
  - `{ current_time_utc: "2050-01-04 15:06:26" }`
- `format_current_time_display({ current_time_utc: "2050-01-04 15:06:26" })` returns:
  - `The current time is 15:06:26 UTC January 4 2050.`

## Non-Goals

This project does not include:

- localization
- time zones other than UTC
- custom time formats
- authentication
- persistence
- polling or live updates
- multiple time endpoints
- HTML rendering in the service response

## Surface Expectations

This spec follows the shared surface and setup-path rules in [Projects](../../README.md#shared-spec-conventions).

For this project, every tutorial run should include both:

- one web service adapter that returns the canonical JSON payload
- one client adapter that consumes that service and renders the canonical display message

Service adapters should:

- use a web service surface
- determine the current UTC timestamp in `YYYY-MM-DD HH:MM:SS` format
- pass that timestamp to `build_current_time_payload`
- pass the payload to `format_current_time_service_response`
- return the resulting JSON with a content type appropriate for JSON responses

Client adapters should:

- make an HTTP GET request to the service adapter endpoint
- pass the response body to `parse_current_time_service_response`
- pass the parsed payload to `format_current_time_display`
- render the returned message exactly

## Output Repository Expectations

This project follows the shared output model in [Projects](../../README.md#shared-output-model).

For this project, a complete run should produce:

- one core library repo
- one web service adapter repo
- one client adapter repo

The core repo owns `build_current_time_payload`, `format_current_time_service_response`, `parse_current_time_service_response`, and `format_current_time_display`, and neither adapter repo should reimplement them.

## Testing And Coverage Contract

This spec follows the shared coverage policy in [Projects](../../README.md#shared-coverage-policy).

This project should be built in a spec-driven and test-driven way.

Minimum test expectations:

- a test that `build_current_time_payload` preserves the exact canonical timestamp
- a test that `format_current_time_service_response` preserves the exact canonical `currentTime` JSON shape
- a test that `parse_current_time_service_response` preserves the exact canonical timestamp
- a test that `format_current_time_display` preserves the exact canonical display sentence
- tests for every service adapter built in the chosen run that prove it gets the current UTC time, delegates payload building and JSON formatting to the core logic, and returns the canonical JSON shape
- tests for every client adapter built in the chosen run that prove it fetches the service response, delegates response parsing and display formatting to the core logic, and renders the canonical message correctly
