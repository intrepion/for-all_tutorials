---
partial_kind: project-spec
project: whos-in-space
---

# Spec

Canonical project contract for the `whos-in-space` project.

## Goal

Build a small project app that introduces:

- fetching JSON from the Open Notify people-in-space endpoint
- parsing the response into structured person records
- preserving the response order of the people list
- formatting a tabular report from the parsed response
- test-first parsing and formatting logic in small core functions
- thin surface adapters

## Canonical Sample API Response

For deterministic tests and examples, use a response equivalent to this JSON document:

```json
{
  "message": "success",
  "number": 3,
  "people": [
    { "name": "Gennady Padalka", "craft": "ISS" },
    { "name": "Mikhail Kornienko", "craft": "ISS" },
    { "name": "Scott Kelly", "craft": "ISS" }
  ]
}
```

## Core Logic Contract

The shared core contracts are:

```text
parse_people_in_space_response(json_text: string) -> people_in_space_report
format_people_in_space_report(people_in_space_report) -> string[]
```

Where `people_in_space_report` contains:

```text
number_of_people
people
```

and each person contains:

```text
name
craft
```

Canonical behavior:

- `parse_people_in_space_response`:
  - parses the JSON document
  - preserves the exact text of each `name`
  - preserves the exact text of each `craft`
  - preserves the source order of the `people` array
  - reads the `number` field into `number_of_people`
  - ignores the `message` field after successful parsing
- `format_people_in_space_report` returns these lines in this exact order:
  - `There are <number_of_people> people in space right now:`
  - `Name | Craft`
  - `--------------------|------`
  - one line per person in this exact format:
    - `<name> | <craft>`

Examples:

- `parse_people_in_space_response(canonical_json_text)` returns:
  - `number_of_people = 3`
  - people in this order:
    - `{ name: "Gennady Padalka", craft: "ISS" }`
    - `{ name: "Mikhail Kornienko", craft: "ISS" }`
    - `{ name: "Scott Kelly", craft: "ISS" }`
- `format_people_in_space_report(parse_people_in_space_response(canonical_json_text))` returns:
  - `There are 3 people in space right now:`
  - `Name | Craft`
  - `--------------------|------`
  - `Gennady Padalka | ISS`
  - `Mikhail Kornienko | ISS`
  - `Scott Kelly | ISS`

## Non-Goals

This project does not include:

- localization
- authentication
- writing response data to a file
- retry logic in the core
- sorting or filtering the returned people list
- caching the API response
- handling multiple API providers
- redefining the response schema

## Surface Expectations

This spec follows the shared surface and setup-path rules in [Projects](../../README.md#shared-spec-conventions).

For this project, every tutorial run should adapt the same response parsing and table-formatting behavior instead of redefining it.

Adapters should:

- make an HTTP GET request to the Open Notify people-in-space endpoint:
  - `http://api.open-notify.org/astros.json`
- pass the response body to `parse_people_in_space_response`
- pass the parsed report to `format_people_in_space_report`
- render the returned lines in order

## Output Repository Expectations

This project follows the shared output model in [Projects](../../README.md#shared-output-model).

For this project, the core repo owns `parse_people_in_space_response` and `format_people_in_space_report`, and adapter repos must not reimplement them.

## Testing And Coverage Contract

This spec follows the shared coverage policy in [Projects](../../README.md#shared-coverage-policy).

This project should be built in a spec-driven and test-driven way.

Minimum test expectations:

- a test that `parse_people_in_space_response` preserves the canonical number and people order
- a test that `parse_people_in_space_response` preserves the exact canonical names and craft values
- a test that `format_people_in_space_report` preserves the exact summary line, header line, and separator line
- a test that `format_people_in_space_report` preserves the exact canonical person rows
- tests for every adapter built in the chosen run that prove it fetches the endpoint, delegates parsing and formatting to the core logic, and renders the resulting table correctly

