---
partial_kind: adapter-instructions
project: creating-your-own-time-service
---

# Service Adapter Instructions

Project-specific service-adapter instruction fragment for the `creating-your-own-time-service` project.

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
