<!-- breadcrumbs:start -->
[for-all_tutorials](../../../README.md) / [Projects](../../README.md) / [Todo List](../README.md) / Spec
<!-- breadcrumbs:end -->

# Spec

Canonical project contract for the `todo-list` project.

## Goal

Build a small project app that introduces:

- collecting repeated task input from the user
- ending task entry when the user enters a blank task
- persisting tasks in a permanent local storage location
- parsing and serializing a stored task list
- removing one completed task by exact text
- formatting the current task list for display
- test-first parsing, list, and formatting logic in small core functions
- thin command-line adapters

## Canonical Sample Stored Task Data

For deterministic tests and examples, use stored task data equivalent to this plain-text document:

```text
Learn how to invert binary trees
Buy milk
Clean kitchen
```

## Core Logic Contract

The shared core contracts are:

```text
parse_task_storage(storage_text: string) -> string[]
append_task(task_list: string[], task_text: string) -> string[]
remove_task_by_exact_text(task_list: string[], completed_task_text: string) -> string[]
format_task_list(task_list: string[]) -> string[]
serialize_task_storage(task_list: string[]) -> string
```

Canonical behavior:

- `parse_task_storage`:
  - splits stored task text into one task per line
  - preserves the exact text of each non-empty line
  - preserves source order
- `append_task`:
  - returns a new list
  - appends `task_text` exactly as provided
  - preserves the existing task order
- `remove_task_by_exact_text`:
  - compares tasks by exact string match
  - does not trim, normalize, or case-fold `completed_task_text`
  - returns a new list
  - removes the first exact matching task when present
  - preserves the relative order of the remaining tasks
  - returns the unchanged list when no exact match is present
- `format_task_list`:
  - returns one line per task
  - preserves input order
- `serialize_task_storage`:
  - returns one task per line in input order
  - ends the stored text with a trailing newline when the list is non-empty
  - returns an empty string for an empty task list

Examples:

- `parse_task_storage(canonical_storage_text)` returns:
  - `Learn how to invert binary trees`
  - `Buy milk`
  - `Clean kitchen`
- `append_task(["Learn how to invert binary trees", "Buy milk"], "Clean kitchen")` returns:
  - `Learn how to invert binary trees`
  - `Buy milk`
  - `Clean kitchen`
- `remove_task_by_exact_text(parse_task_storage(canonical_storage_text), "Buy milk")` returns:
  - `Learn how to invert binary trees`
  - `Clean kitchen`
- `format_task_list(["Learn how to invert binary trees", "Buy milk"])` returns:
  - `Learn how to invert binary trees`
  - `Buy milk`
- `serialize_task_storage(["Learn how to invert binary trees", "Buy milk"])` returns text equivalent to:

```text
Learn how to invert binary trees
Buy milk
```

## Non-Goals

This project does not include:

- a graphical interface
- due dates
- priorities
- categories or tags
- multiple lists
- syncing tasks across machines
- partial-match removal
- task editing
- network access

## Surface Expectations

This spec follows the shared surface and setup-path rules in [Projects](../../README.md#shared-spec-conventions).

For this project, every tutorial run should use a command-line adapter and adapt the same parsing, append, removal, formatting, and serialization behavior instead of redefining it.

Adapters should:

- use a command-line surface
- read the existing stored tasks from a permanent local storage location
- pass the stored text to `parse_task_storage`
- prompt repeatedly for chores or tasks
- stop prompting for new tasks when the user enters a blank task
- not pass the blank task to `append_task`
- for each non-blank entered task:
  - pass the current task list and the entered task to `append_task`
- display all tasks by passing the current task list to `format_task_list`
- prompt for one task to remove
- pass the current task list and the entered completed task to `remove_task_by_exact_text`
- persist the final task list by passing it to `serialize_task_storage`
- write the serialized text back to the same permanent local storage location

## Output Repository Expectations

This project follows the shared output model in [Projects](../../README.md#shared-output-model).

For this project, the core repo owns `parse_task_storage`, `append_task`, `remove_task_by_exact_text`, `format_task_list`, and `serialize_task_storage`, and adapter repos must not reimplement them.

## Testing And Coverage Contract

This spec follows the shared coverage policy in [Projects](../../README.md#shared-coverage-policy).

This project should be built in a spec-driven and test-driven way.

Minimum test expectations:

- a test that `parse_task_storage` preserves the exact canonical task texts and order
- a test that `append_task` returns a new list with the appended task at the end
- a test that `remove_task_by_exact_text` removes an exact matching task such as `Buy milk`
- a test that `remove_task_by_exact_text` preserves the unchanged list for a non-matching task
- a test that `format_task_list` preserves exact task text and order
- a test that `serialize_task_storage` preserves exact task text and order
- tests for every adapter built in the chosen run that prove it reads from permanent local storage, stops task entry on a blank task without storing it, displays the current tasks, removes the chosen completed task, and writes the final serialized task list back to permanent local storage
