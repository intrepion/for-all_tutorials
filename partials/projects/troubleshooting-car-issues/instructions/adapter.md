---
partial_kind: adapter-instructions
project: troubleshooting-car-issues
---

# Adapter Instructions

Project-specific adapter instruction fragment for the `troubleshooting-car-issues` adapter repo.

### 1. Red: Add An Adapter-Level Test

In a separate adapter repo, add a failing test that proves the chosen adapter asks the correct branch prompts and delegates to the core logic correctly.

### 2. Green: Add The Thin Surface Adapter

Add the thinnest possible adapter for the surface you are building:

- ask `Is the car silent when you turn the key?`
- if the answer is yes, ask `Are the battery terminals corroded?`
- otherwise ask `Does the car make a clicking noise?`
- if that answer is no, ask `Does the car crank up but fail to start?`
- if that answer is no, ask `Does the engine start and then die?`
- if that answer is yes, ask `Does your car have fuel injection?`
- convert the branch-relevant yes/no answers into the boolean values expected by the core logic
- pass those values to `diagnose_car_issue`
- pass the returned diagnosis code to `format_car_diagnosis_message`
- return, render, or print the returned lines in the form that surface requires
- keep prompt flow and yes/no parsing out of the core decision-tree logic

For prompt-driven adapters, the prompts should be equivalent to the decision tree questions listed in the spec and should only be asked when the current branch requires them.

### 3. Refactor

Clean up any remaining duplication while keeping the full suite green.

