---
partial_kind: core-instructions
project: todo-list
---

# Core Instructions

Project-specific core instruction fragment for the `todo-list` core repo.

### 1. Red: Write The First Failing Test

Start with parsing the canonical stored task data:

```text
given the canonical stored task JSON
when parse_task_storage is called
then the returned task list preserves the exact canonical task texts in order
```

At this point, `parse_task_storage` may not exist yet. That is fine. The first run should fail either at compile time or at test execution time.

Suggested generic test name:

```text
parse_task_storage_preserves_the_canonical_tasks_in_order
```

### 2. Green: Make The First Test Pass

Create the smallest production code that makes the first test pass.

At this stage, it is acceptable if the code only handles the canonical parsing case. Do not jump ahead and implement append, removal, or serialization behavior before those tests exist.

### 3. Refactor

Do the smallest cleanup that keeps the first test green.

### 4. Red: Add Task Append And Removal

Write the next failing tests:

```text
given an existing task list and a new task
when append_task is called
then the returned list appends the task and preserves the existing order
```

and

```text
given the canonical task list
when remove_task_by_exact_text is called with Buy milk
then the returned list removes that task and preserves the remaining order
```

Suggested generic test names:

```text
append_task_appends_the_task_to_the_end_of_a_new_list
remove_task_by_exact_text_removes_the_first_exact_match
```

### 5. Green: Make The List-Update Tests Pass

Make the smallest safe change that gets those tests green.

### 6. Red: Add Formatting And Serialization

Write the next failing tests:

```text
given a task list
when format_task_list is called
then the returned lines preserve the exact task text and order
```

and

```text
given a task list
when serialize_task_storage is called
then the returned JSON preserves the task order in a JSON array
```

Suggested generic test names:

```text
format_task_list_returns_one_line_per_task_in_order
serialize_task_storage_returns_a_json_array_in_order
```

### 7. Green: Make The Formatting Tests Pass

Make the smallest safe change that gets the full suite green.

### 8. Refactor The Core Logic

Now that the full behavior is covered, refactor toward a small, clear core abstraction.

The exact shape can vary by stack:

- five functions in one module
- one service object with parse, append, remove, format, and serialize methods
- one module plus a small immutable task-list type

The important thing is that task parsing, list updates, formatting, and JSON serialization stay small, explicit, and directly testable.
