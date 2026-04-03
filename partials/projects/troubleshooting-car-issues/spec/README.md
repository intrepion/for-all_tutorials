---
partial_kind: project-spec
project: troubleshooting-car-issues
---

# Spec

Canonical project contract for the `troubleshooting-car-issues` project.

## Goal

Build a small project app that introduces:

- resolving a nested yes/no troubleshooting tree
- asking only the follow-up questions needed for the current branch
- returning one troubleshooting diagnosis from the completed answer path
- separating decision-tree resolution from output formatting
- test-first decision and formatting logic in small core functions
- thin surface adapters

## Core Logic Contract

The shared core contracts are:

```text
diagnose_car_issue(answers) -> diagnosis_code
format_car_diagnosis_message(diagnosis_code) -> string[]
```

Where `answers` contains these boolean values, with only the branch-relevant values required:

```text
silent_when_turning_key
battery_terminals_corroded
clicking_noise
cranks_but_fails_to_start
engine_starts_then_dies
has_fuel_injection
```

Canonical behavior:

- `diagnose_car_issue` resolves the decision tree in this order:
  - if `silent_when_turning_key` is `true`:
    - if `battery_terminals_corroded` is `true`, return `clean_terminals`
    - otherwise return `replace_cables`
  - otherwise, if `clicking_noise` is `true`, return `replace_battery`
  - otherwise, if `cranks_but_fails_to_start` is `true`, return `check_spark_plugs`
  - otherwise, if `engine_starts_then_dies` is `true`:
    - if `has_fuel_injection` is `true`, return `get_service`
    - otherwise return `check_choke`
  - otherwise return `not_possible`
- `format_car_diagnosis_message` returns:
  - for `clean_terminals`:
    - `Clean the terminals and try starting again.`
  - for `replace_cables`:
    - `The battery cables may be damaged.`
    - `Replace cables and try again.`
  - for `replace_battery`:
    - `Replace the battery.`
  - for `check_spark_plugs`:
    - `Check the spark plug connections.`
  - for `get_service`:
    - `Get it in for service.`
  - for `check_choke`:
    - `Check to ensure the choke is opening and closing.`
  - for `not_possible`:
    - `This should not be possible.`

Examples:

- if `silent_when_turning_key = true` and `battery_terminals_corroded = false`, `diagnose_car_issue(...)` returns `replace_cables`
- `format_car_diagnosis_message("replace_cables")` returns:
  - `The battery cables may be damaged.`
  - `Replace cables and try again.`
- if `silent_when_turning_key = false`, `clicking_noise = true`, `diagnose_car_issue(...)` returns `replace_battery`
- if `silent_when_turning_key = false`, `clicking_noise = false`, `cranks_but_fails_to_start = false`, `engine_starts_then_dies = true`, `has_fuel_injection = false`, `diagnose_car_issue(...)` returns `check_choke`

## Non-Goals

This project does not include:

- persistence
- localization
- authentication
- network access
- configuration files
- input normalization beyond parsing yes/no answers in the adapter
- probabilistic diagnosis or scoring
- multiple simultaneous diagnoses
- collecting all possible answers up front instead of following the branch path

## Surface Expectations

This spec follows the shared surface and setup-path rules in [Projects](../../README.md#shared-spec-conventions).

For this project, every tutorial run should adapt the same decision-tree and diagnosis-formatting behavior instead of redefining it.

Adapters should ask only the prompts needed for the current branch and should convert those answers into the branch-relevant boolean values expected by the core logic.

## Output Repository Expectations

This project follows the shared output model in [Projects](../../README.md#shared-output-model).

For this project, the core repo owns `diagnose_car_issue` and `format_car_diagnosis_message`, and adapter repos must not reimplement them.

## Testing And Coverage Contract

This spec follows the shared coverage policy in [Projects](../../README.md#shared-coverage-policy).

This project should be built in a spec-driven and test-driven way.

Minimum test expectations:

- tests for each diagnosis leaf in the decision tree
- tests that branch-irrelevant answers are not needed once a diagnosis path is already determined
- tests that `format_car_diagnosis_message` preserves the exact two-line `replace_cables` report
- tests that `format_car_diagnosis_message` preserves the exact one-line reports for the other diagnosis leaves
- tests for every adapter built in the chosen run that prove it asks the correct branch-specific prompts and delegates to the core logic correctly

