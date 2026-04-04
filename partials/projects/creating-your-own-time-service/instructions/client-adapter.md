---
partial_kind: adapter-instructions
project: creating-your-own-time-service
---

# Client Adapter Instructions

Project-specific client-adapter instruction fragment for the `creating-your-own-time-service` project.

### 1. Red: Add A Client-Adapter Test

In a separate client adapter repo, add a failing test that proves the chosen client adapter fetches the service response, delegates response parsing and display formatting to the core logic, and renders the canonical message exactly.

### 2. Green: Add The Thin Client Adapter

Add the thinnest possible client adapter:

- make an HTTP GET request to the service adapter endpoint
- pass the response body to `parse_current_time_service_response`
- pass the parsed payload to `format_current_time_display`
- render the returned message exactly
- keep HTTP transport and UI code out of the core parsing and display logic

### 3. Refactor The Client Adapter

Clean up any remaining duplication while keeping the full suite green.
