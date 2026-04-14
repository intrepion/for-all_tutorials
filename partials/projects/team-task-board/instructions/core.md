---
partial_kind: core-instructions
project: team-task-board
---

# Core Instructions

Project-specific core instruction fragment for the `team-task-board` core repo.

### 1. Red: Parse The Canonical Team Task Storage

Start with parsing the canonical team task JSON:

```text
given the canonical team task storage json text
when parse_team_task_storage is called
then the returned tasks preserve the exact ids, owner ids, text, visibility, and source order
```

Suggested generic test name:

```text
parse_team_task_storage_preserves_the_canonical_team_tasks
```

### 2. Green: Make The First Test Pass

Create the smallest production code that makes the parsing test pass.

### 3. Red: Filter Visible Tasks For Anonymous Users

Write the next failing test:

```text
given the parsed canonical team task storage
when filter_visible_tasks is called for anonymous
then only the public task task-1 is returned
```

### 4. Green: Make The Anonymous Filter Test Pass

Make the smallest safe change that gets both tests green.

### 5. Red: Filter Visible Tasks For Normal Users And Admins

Write failing tests that require:

```text
user("user-alice") sees task-1 and task-3
admin("user-admin") sees task-1, task-2, and task-3
```

### 6. Green: Make The User And Admin Filter Tests Pass

Make them pass while keeping the earlier tests green.

### 7. Red: Add Deletion Authorization

Write failing tests that require:

```text
can_delete_task(task-2, user("user-bob")) is true
can_delete_task(task-2, user("user-alice")) is false
can_delete_task(task-2, admin("user-admin")) is true
```

### 8. Green: Make The Authorization Tests Pass

Make them pass while keeping the earlier tests green.

### 9. Red: Add Authorized Removal And Formatting

Write failing tests for:

```text
remove_task_by_id removes an authorized exact match and preserves order
format_team_task_list returns lines in the shape <id> | <visibility> | <text>
```

### 10. Green: Make The Removal And Formatting Tests Pass

Make them pass while keeping the earlier tests green.
