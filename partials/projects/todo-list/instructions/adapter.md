---
partial_kind: adapter-instructions
project: todo-list
---

# Adapter Instructions

Project-specific adapter instruction fragment for the `todo-list` adapter repo.

### 1. Red: Add An Adapter-Level Test

In a separate adapter repo, add a failing test that proves the chosen adapter loads stored tasks from the chosen durable storage mechanism, collects repeated task input until a blank entry, delegates list parsing and updates to the core logic, displays the current tasks, removes one completed task, and persists the final task list without changing the canonical JSON task semantics.

### 2. Green: Add The Thin Adapter

Add the thinnest possible adapter for the chosen surface and storage approach:

- load the stored tasks from the chosen durable storage mechanism in a form equivalent to the canonical JSON task array
- pass equivalent stored JSON text to `parse_task_storage`
- collect repeated chores or tasks through the chosen surface
- stop task entry when the user enters a blank task
- for each non-blank task:
  - pass the current list and the task to `append_task`
- pass the resulting list to `format_task_list`
- render or return the resulting list in the form the chosen surface requires
- collect one completed task to remove through the chosen surface
- pass the current list and the removal text to `remove_task_by_exact_text`
- pass the final list to `serialize_task_storage`
- if the chosen storage engine is not itself a local JSON file, translate the serialized JSON array into the chosen durable storage representation before saving it
- keep surface interaction and storage code out of the core task logic

### 3. Refactor

Clean up any remaining duplication while keeping the full suite green.
