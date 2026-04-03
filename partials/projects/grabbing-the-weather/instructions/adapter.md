---
partial_kind: adapter-instructions
project: grabbing-the-weather
---

# Adapter Instructions

Project-specific adapter instruction fragment for the `grabbing-the-weather` adapter repo.

### 1. Red: Add An Adapter-Level Test

In a separate adapter repo, add a failing test that proves the chosen adapter prompts for the city query, performs the HTTP GET request with `q`, `units=imperial`, and `appid`, delegates response parsing and report formatting to the core logic, and renders the returned lines in order.

### 2. Green: Add The Thin Surface Adapter

Add the thinnest possible adapter for the surface you are building:

- render the equivalent of `Where are you?`
- make an HTTP GET request to `https://api.openweathermap.org/data/2.5/weather`
- pass the entered location as the `q` query parameter
- pass `units=imperial`
- pass `appid=<api key>`
- pass the response body to `parse_current_weather_response`
- pass the parsed report to `format_current_weather_report`
- render the returned lines in order
- keep HTTP transport, secret handling, and response retrieval code out of the core parsing and formatting logic

### 3. Refactor

Clean up any remaining duplication while keeping the full suite green.

