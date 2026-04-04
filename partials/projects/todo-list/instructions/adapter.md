---
partial_kind: adapter-instructions
project: todo-list
---

# Adapter Instructions

Project-specific adapter instruction fragment for the `todo-list` adapter repo.

### 1. Red: Add An Adapter-Level Test

In a separate adapter repo, add a failing test that proves the chosen command-line adapter reads the stored tasks from one permanent local JSON file, prompts repeatedly for new tasks until a blank entry, delegates list parsing and updates to the core logic, displays the current tasks, removes one completed task, and writes the final serialized JSON back to the same file.

### 2. Green: Add The Thin Command-Line Adapter

Add the thinnest possible adapter for the command-line surface you are building:

- read stored task JSON from one permanent local JSON file
- pass that text to `parse_task_storage`
- prompt repeatedly for chores or tasks
- stop prompting for new tasks when the user enters a blank task
- for each non-blank task:
  - pass the current list and the task to `append_task`
- pass the resulting list to `format_task_list`
- render the returned lines in order
- prompt for one completed task to remove
- pass the current list and the removal text to `remove_task_by_exact_text`
- pass the final list to `serialize_task_storage`
- write the serialized JSON back to the same permanent local JSON file
- keep prompt flow and file I/O code out of the core task logic

### 3. Refactor

Clean up any remaining duplication while keeping the full suite green.
