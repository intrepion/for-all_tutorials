<!-- breadcrumbs:start -->
[for-all_tutorials](../../../../../README.md) / [Tutorials](../../../../README.md) / [Level 001: Foundations](../../../README.md) / [Todo List](../../README.md) / [Full-Stack](../README.md) / Todo List
<!-- breadcrumbs:end -->

# Todo List

Level 001: Foundations full-stack tutorial using C# / Blazor Web App (Server).

## Tutorial Goal

Build a small server-hosted Todo List that matches the canonical spec in [specs/level-001-foundations/todo-list/README.md](/Users/intrepion/Code/github/intrepion/for-all_tutorials/specs/level-001-foundations/todo-list/README.md):

- create, edit, toggle, and delete todos
- filter by `all`, `active`, and `completed`
- validate on the server
- persist to SQLite
- keep the UI simple and spec-focused

## Version Note

This repo now uses the human-readable stack label `C# / Blazor Web App (Server)` while keeping the stable folder slug `csharp-blazor-web-app-server`.

In the current SDK in this workspace, `10.0.201`, the supported project template is the unified Blazor Web App template:

```bash
dotnet new blazor -int Server
```

For this tutorial, that template is the modern way to build the server-interactive Blazor app this stack is meant to represent.

## What You Should End Up With

- a single `/` page implemented in `Components/Pages/Home.razor`
- interactive server rendering for the todo UI
- SQLite-backed persistence through EF Core
- short-lived `DbContext` usage via `IDbContextFactory<TodoDbContext>`
- HTML-escaped rendering through normal Razor output

## Prerequisites

- .NET SDK `10.x`
- a shell with `dotnet`
- the `dotnet-ef` tool for migrations

Install the EF Core tool once if needed:

```bash
dotnet tool install --global dotnet-ef
```

## Create The Project

From a clean working directory:

```bash
dotnet new blazor -n TodoList.BlazorServer -o TodoList.BlazorServer -int Server -e
cd TodoList.BlazorServer
dotnet add package Microsoft.EntityFrameworkCore.Sqlite
dotnet add package Microsoft.EntityFrameworkCore.Design
```

Why this shape:

- `-int Server` gives you server-side interactivity
- `-e` starts from the empty template so the tutorial stays close to the repo spec
- SQLite keeps persistence durable without introducing extra infrastructure

## Suggested File Map

Add these files:

- `Data/TodoItem.cs`
- `Data/TodoDbContext.cs`
- `Services/TodoService.cs`

Update these files:

- `Program.cs`
- `Components/Pages/Home.razor`

You can keep everything else from the template as-is for this tutorial.

## Data Model

Use a small entity that maps directly to the spec:

```csharp
public sealed class TodoItem
{
    public Guid Id { get; set; } = Guid.NewGuid();
    public string Title { get; set; } = "";
    public bool Completed { get; set; }
    public DateTimeOffset CreatedAt { get; set; } = DateTimeOffset.UtcNow;
    public DateTimeOffset UpdatedAt { get; set; } = DateTimeOffset.UtcNow;
}
```

Notes:

- `Guid` is a simple server-generated identifier
- `CreatedAt` gives you stable ordering
- `UpdatedAt` changes on edit and toggle

## Persistence Strategy

Blazor Web App projects using server interactivity keep long-lived circuits open, so avoid holding a single `DbContext` inside a component for the life of the circuit. Instead:

- register `IDbContextFactory<TodoDbContext>`
- create a fresh context per operation in your service layer
- keep EF Core concerns out of the page component

In `Program.cs`, register the database and service:

```csharp
using Microsoft.EntityFrameworkCore;
using TodoList.BlazorServer.Components;
using TodoList.BlazorServer.Data;
using TodoList.BlazorServer.Services;

var builder = WebApplication.CreateBuilder(args);

builder.Services.AddRazorComponents()
    .AddInteractiveServerComponents();

builder.Services.AddDbContextFactory<TodoDbContext>(options =>
    options.UseSqlite("Data Source=todos.db"));

builder.Services.AddScoped<TodoService>();
```

Before `app.Run();`, ensure the database exists and migrations are applied:

```csharp
using (var scope = app.Services.CreateScope())
{
    var factory = scope.ServiceProvider.GetRequiredService<IDbContextFactory<TodoDbContext>>();
    await using var db = await factory.CreateDbContextAsync();
    await db.Database.MigrateAsync();
}
```

Create an initial migration after your `DbContext` exists:

```bash
dotnet ef migrations add InitialCreate
```

## Validation Strategy

Blazor forms can give immediate user feedback, but the spec requires server-side validation too. The safest pattern here is:

- use `EditForm` for user feedback
- trim and validate again in `TodoService`
- reject invalid create and edit operations before saving

Server-side rules to enforce in the service:

- title is required
- title is trimmed before validation and persistence
- trimmed title length must be between `1` and `120`
- whitespace-only titles are invalid
- duplicates are allowed

The simplest service-side guard looks like this:

```csharp
private static string NormalizeTitle(string? title) => title?.Trim() ?? "";

private static string? ValidateTitle(string normalizedTitle)
{
    if (normalizedTitle.Length is < 1 or > 120)
    {
        return "Title must be between 1 and 120 characters.";
    }

    return null;
}
```

## Service Layer

Keep the page lean by putting all data operations in `TodoService`.

Recommended surface area:

```csharp
Task<IReadOnlyList<TodoItem>> ListAsync(string filter);
Task<string?> CreateAsync(string title);
Task<string?> UpdateAsync(Guid id, string title);
Task ToggleAsync(Guid id);
Task DeleteAsync(Guid id);
```

Implementation rules:

- order by `CreatedAt` ascending
- filter in the query, not in the UI after loading everything
- update `UpdatedAt` on edit and toggle
- after each write, reload the visible list from storage

That keeps the component focused on interaction and display, while the service owns parity with the spec.

## Build The Page

Replace `Components/Pages/Home.razor` with the todo UI.

The page should:

- keep the route at `@page "/"`
- opt into server interactivity with `@rendermode InteractiveServer`
- read the filter from the query string
- render the create form, filter links, list state, and inline edit state

The query-string binding shape can look like this:

```razor
@page "/"
@rendermode InteractiveServer
@inject TodoService Todos
@inject NavigationManager Navigation

@code {
    [SupplyParameterFromQuery(Name = "filter")]
    private string? Filter { get; set; }
}
```

Use `OnParametersSetAsync()` instead of `OnInitializedAsync()` so the page reloads correctly when the user switches between:

- `/`
- `/?filter=all`
- `/?filter=active`
- `/?filter=completed`

## UI State To Track In The Component

Keep just enough local state to drive the page:

- `newTitle`
- `createError`
- `editingId`
- `editingTitle`
- `editError`
- `items`

Recommended interaction flow:

1. Load todos for the current filter in `OnParametersSetAsync`.
2. Submit the create form through the service.
3. If the service returns an error, show it without mutating the list.
4. On success, clear the input and reload the list.
5. Use the same pattern for edit, toggle, and delete.

This aligns naturally with the acceptance criteria in the spec.

## Filtering

Treat anything other than `active` or `completed` as `all`.

That gives you stable behavior for:

- `GET /`
- `GET /?filter=all`
- `GET /?filter=active`
- `GET /?filter=completed`

When a toggle changes whether a todo matches the active filter, reload the list immediately so the item disappears from the current view when appropriate.

## Empty States

The page should visibly handle:

- no todos at all
- todos exist, but none match the current filter
- validation errors on create
- validation errors on edit

Keep the copy simple. The goal is parity and clarity, not polished product copy.

## Security Notes

This tutorial should satisfy the spec's baseline checklist:

- render titles with normal Razor output such as `@todo.Title`
- do not use `MarkupString` for todo content
- keep validation in the service layer, not only in the UI
- let EF Core generate parameterized SQL
- keep the template's `app.UseAntiforgery();` call in place
- do not expose seed or reset helpers in the normal app

## Benchmark Notes

You do not need to add benchmark helpers yet, but do keep the app ready for them later:

- keep routes stable
- keep behavior deterministic
- avoid extra UI features in the canonical mode
- make it possible to add a seed/reset mechanism later without changing user-visible behavior

## Definition Of Done

This tutorial is complete when all of the following are true:

1. A user can create a valid todo.
2. Blank or whitespace-only input is rejected.
3. Input longer than `120` trimmed characters is rejected.
4. Todos persist across browser refreshes and application restarts.
5. A user can edit, toggle, and delete todos.
6. The `all`, `active`, and `completed` filters behave exactly like the spec.
7. Titles are HTML-escaped when rendered.
8. The tutorial remains close to the canonical app shape without extra features.

## Recommended Next Pass

Once the written tutorial exists, the next improvement would be to add:

- a short "Common Blazor Pitfalls" section
- a "Testing Notes" section for component and integration coverage
- a migration snippet for reseeding local benchmark data
