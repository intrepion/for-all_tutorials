---
partial_kind: adapter-instructions
project: whos-in-space
---

# Adapter Instructions

Project-specific adapter instruction fragment for the `whos-in-space` adapter repo.

### 1. Red: Add An Adapter-Level Test

In a separate adapter repo, add a failing test that proves the chosen adapter performs the HTTP GET request, delegates response parsing and report formatting to the core logic, and renders the returned lines in order.

### 2. Green: Add The Thin Surface Adapter

Add the thinnest possible adapter for the surface you are building:

- make an HTTP GET request to `http://api.open-notify.org/astros.json`
- pass the response body to `parse_people_in_space_response`
- pass the parsed report to `format_people_in_space_report`
- render the returned lines in order
- keep HTTP transport and response retrieval code out of the core parsing and formatting logic

### 3. Refactor

Clean up any remaining duplication while keeping the full suite green.

