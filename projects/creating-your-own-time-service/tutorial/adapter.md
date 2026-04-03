<!-- breadcrumbs:start -->
[for-all_tutorials](../../../../README.md) / [Projects](../../../README.md) / [Creating Your Own Time Service](../../README.md) / [Tutorial](../README.md) / adapter.md
<!-- breadcrumbs:end -->

# Adapter Walkthrough

Project-specific red, green, refactor sequence for the `creating-your-own-time-service` adapter repos.

### 1. Red: Add A Service-Adapter Test

In a separate service adapter repo, add a failing test that proves the chosen web service adapter gets the current UTC time, delegates payload building and JSON formatting to the core logic, and returns the canonical `currentTime` JSON shape.

### 2. Green: Add The Thin Service Adapter

Add the thinnest possible web service adapter:

- determine the current UTC timestamp in `YYYY-MM-DD HH:MM:SS` format
- pass it to `build_current_time_payload`
- pass the payload to `format_current_time_service_response`
- return the resulting JSON from an HTTP endpoint
- keep clock access and HTTP transport code out of the core payload and formatting logic

### 3. Refactor The Service Adapter

Clean up any remaining duplication while keeping the service tests green.

### 4. Red: Add A Client-Adapter Test

In a separate client adapter repo, add a failing test that proves the chosen client adapter fetches the service response, delegates response parsing and display formatting to the core logic, and renders the canonical message exactly.

### 5. Green: Add The Thin Client Adapter

Add the thinnest possible client adapter:

- make an HTTP GET request to the service adapter endpoint
- pass the response body to `parse_current_time_service_response`
- pass the parsed payload to `format_current_time_display`
- render the returned message exactly
- keep HTTP transport and UI code out of the core parsing and display logic

### 6. Refactor The Client Adapter

Clean up any remaining duplication while keeping the full suite green.
