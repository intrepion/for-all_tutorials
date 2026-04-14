---
partial_kind: adapter-instructions
project: team-task-board
---

# Adapter Instructions

Project-specific adapter instruction fragment for the `team-task-board` adapter repo.

### 1. Red: Prove Anonymous Users Only See Public Tasks

Write a failing adapter test:

```text
given the current principal is anonymous
when the team task board is rendered
then only the public tasks are shown
and no delete action is available
```

### 2. Green: Render The Anonymous View

Make the smallest adapter change that delegates visible-task filtering to `filter_visible_tasks` and list formatting to `format_team_task_list`.

### 3. Red: Prove Normal Users Can Delete Only Their Own Tasks

Write a failing adapter test:

```text
given the current principal is user("user-bob")
when the board includes task-2 owned by user-bob and task-1 owned by user-alice
then delete is allowed for task-2
and delete is not allowed for task-1
```

### 4. Green: Wire Owner Deletion

Make the smallest adapter change that uses `find_task_by_id`, `can_delete_task`, and `remove_task_by_id` for the normal-user path.

### 5. Red: Prove Admins Can Delete Any Task

Write a failing adapter test for the admin path.

### 6. Green: Wire Admin Deletion

Make it pass while keeping the earlier anonymous and normal-user paths green.
