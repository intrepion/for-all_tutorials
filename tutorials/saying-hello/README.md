<!-- breadcrumbs:start -->
[for-all_tutorials](../../README.md) / [Tutorials](../README.md) / Saying Hello
<!-- breadcrumbs:end -->

# Saying Hello

Stack-agnostic TDD walkthrough for the first project app in the curriculum.

## Goal

Build the smallest useful project app in a test-driven way.

The point of this tutorial is not to teach one specific language or framework. It is to teach the sequence:

1. read the spec
2. write a failing test
3. make it pass with the smallest change
4. refactor
5. add the next behavior
6. only then wrap the core logic in a thin surface adapter

## Start With The Spec

Use [specs/saying-hello/README.md](../../specs/saying-hello/README.md) as the source of truth.

The canonical contract is:

```text
greet(name: string) -> string
```

Canonical behavior:

- trim leading and trailing whitespace from `name`
- return `Hello, <name>!` when the trimmed value is non-empty
- return `Hello!` when the trimmed value is empty or whitespace only

## Choose A Setup Path

Before you code, choose one stack path from [setups/README.md](../../setups/README.md).

Current examples in this repo:

- [C# setup root](../../setups/csharp/README.md)
- [C# .NET SDK](../../setups/csharp/sdk/README.md)
- [C# xUnit](../../setups/csharp/testing/xunit/README.md)
- [C# NUnit](../../setups/csharp/testing/nunit/README.md)
- [C# MSTest](../../setups/csharp/testing/mstest/README.md)
- [C# TUnit](../../setups/csharp/testing/tunit/README.md)

Those setup guides explain how to scaffold projects, choose templates, and collect coverage. This tutorial explains the project-specific red-green-refactor sequence.

## TDD Sequence

### 1. Scaffold The Smallest Viable Project

Use your chosen setup path to create:

- one production project that will hold `greet`
- one test project
- one solution or equivalent workspace root if the stack supports it

Do not build the full app yet. Start with the smallest structure that lets you write and run tests.

### 2. Write The First Failing Test

Start with the happiest path:

```text
given name "Ada"
when greet is called
then the result is "Hello, Ada!"
```

At this point, `greet` may not exist yet. That is fine. The first run should fail either at compile time or at test execution time.

Suggested generic test name:

```text
greet_returns_personalized_greeting_for_non_empty_name
```

### 3. Make The First Test Pass

Create the smallest production code that makes the first test pass.

At this stage, it is acceptable if the code only handles the first case. Do not jump ahead and implement trimming or empty-input behavior before those tests exist.

### 4. Add The Trimming Behavior

Write the next failing test:

```text
given name "  Ada  "
when greet is called
then the result is "Hello, Ada!"
```

Suggested generic test name:

```text
greet_trims_leading_and_trailing_whitespace
```

Make it pass with the smallest safe change.

### 5. Add The Empty-String Behavior

Write the next failing test:

```text
given name ""
when greet is called
then the result is "Hello!"
```

Suggested generic test name:

```text
greet_returns_generic_greeting_for_empty_string
```

Make it pass.

### 6. Add The Whitespace-Only Behavior

Write the next failing test:

```text
given name "   "
when greet is called
then the result is "Hello!"
```

Suggested generic test name:

```text
greet_returns_generic_greeting_for_whitespace_only_input
```

Make it pass.

### 7. Refactor The Core Logic

Now that the full behavior is covered, refactor toward a small, clear core abstraction.

The exact shape can vary by stack:

- one function
- one service object
- one module
- one static helper

The important thing is that the greeting rules live in one small unit that can be tested directly.

### 8. Add The Thin Surface Adapter

The initial required surface for this project is `command-line/all`.

Wrap the tested core logic in the thinnest adapter you can:

- accept zero or one name input from the command line
- pass that input to `greet`
- print the returned string
- keep parsing and input/output code out of the core greeting logic

If your stack makes adapter testing practical, add tests that prove the adapter delegates to the core logic correctly.

### 9. Verify Coverage

Use the coverage workflow from your chosen setup path.

The expected thresholds are:

- baseline: `90%` code coverage and `85%` branch coverage
- validation and service logic: `100%` code coverage and `100%` branch coverage

For `saying-hello`, the core `greet` logic should hit the stricter `100%` / `100%` target.

## Definition Of Done

This tutorial is done when:

- the behavior matches [specs/saying-hello/README.md](../../specs/saying-hello/README.md)
- the project was built in a real red, green, refactor sequence
- the core `greet` logic is thoroughly tested
- the command-line adapter is thin and does not redefine greeting rules
- the coverage goals are met

## Next Step

After `saying-hello` is solid in one setup path, the next useful move is to add another setup path for the same project so the tutorial stays constant while only the stack changes.
