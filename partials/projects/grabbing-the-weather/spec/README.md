---
partial_kind: project-spec
project: grabbing-the-weather
---

# Spec

Canonical project contract for the `grabbing-the-weather` project.

## Goal

Build a small project app that introduces:

- prompting for a city query
- fetching current weather data from OpenWeatherMap
- parsing the response into a city name and a current Fahrenheit temperature
- rounding the reported temperature to the nearest whole degree
- formatting a small weather report
- test-first parsing and formatting logic in small core functions
- thin surface adapters

## Canonical Sample API Response

For deterministic tests and examples, use a response equivalent to this JSON document:

```json
{
  "name": "Chicago",
  "main": {
    "temp": 65.0
  }
}
```

## Core Logic Contract

The shared core contracts are:

```text
parse_current_weather_response(json_text: string) -> current_weather_report
format_current_weather_report(current_weather_report) -> string[]
```

Where `current_weather_report` contains:

```text
city_name
temperature_fahrenheit
```

Canonical behavior:

- `parse_current_weather_response`:
  - parses the JSON document
  - reads `name` into `city_name`
  - reads `main.temp` as the current Fahrenheit temperature
  - rounds the temperature to the nearest whole degree using half-up rounding
  - returns that rounded whole number as `temperature_fahrenheit`
- `format_current_weather_report` returns these lines in this exact order:
  - `<city_name> weather:`
  - `<temperature_fahrenheit> degrees Fahrenheit`

Examples:

- `parse_current_weather_response(canonical_json_text)` returns:
  - `city_name = "Chicago"`
  - `temperature_fahrenheit = 65`
- `parse_current_weather_response("{\"name\":\"Chicago\",\"main\":{\"temp\":65.4}}")` returns:
  - `city_name = "Chicago"`
  - `temperature_fahrenheit = 65`
- `parse_current_weather_response("{\"name\":\"Chicago\",\"main\":{\"temp\":65.5}}")` returns:
  - `city_name = "Chicago"`
  - `temperature_fahrenheit = 66`
- `format_current_weather_report({ city_name: "Chicago", temperature_fahrenheit: 65 })` returns:
  - `Chicago weather:`
  - `65 degrees Fahrenheit`

## Non-Goals

This project does not include:

- localization
- authentication beyond supplying the API key required by the weather provider
- writing weather data to a file
- forecast endpoints
- Celsius or Kelvin display
- retry loops in the core
- multiple-city reports
- caching weather data

## Surface Expectations

This spec follows the shared surface and setup-path rules in [Projects](../../README.md#shared-spec-conventions).

For this project, every tutorial run should adapt the same response parsing and weather-report formatting behavior instead of redefining it.

Adapters should:

- prompt for one city query
- make an HTTP GET request to OpenWeatherMap Current Weather Data:
  - `https://api.openweathermap.org/data/2.5/weather`
- pass the city query with the `q` parameter
- request Fahrenheit output by passing `units=imperial`
- authenticate with an OpenWeatherMap API key by passing `appid=<api key>`
- pass the response body to `parse_current_weather_response`
- pass the parsed report to `format_current_weather_report`
- render the returned lines in order

For prompt-driven adapters, the prompt should be equivalent to:

```text
Where are you?
```

## Output Repository Expectations

This project follows the shared output model in [Projects](../../README.md#shared-output-model).

For this project, the core repo owns `parse_current_weather_response` and `format_current_weather_report`, and adapter repos must not reimplement them.

## Testing And Coverage Contract

This spec follows the shared coverage policy in [Projects](../../README.md#shared-coverage-policy).

This project should be built in a spec-driven and test-driven way.

Minimum test expectations:

- a test that `parse_current_weather_response` preserves the canonical city name and rounded whole-degree temperature
- a test that `parse_current_weather_response` applies half-up rounding for a `.5` temperature
- a test that `format_current_weather_report` preserves the exact canonical two-line output
- tests for every adapter built in the chosen run that prove it prompts for the city query, calls the weather endpoint with `q`, `units=imperial`, and `appid`, and delegates parsing and formatting to the core logic correctly

