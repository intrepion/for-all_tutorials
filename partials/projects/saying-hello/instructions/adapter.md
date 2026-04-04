---
partial_kind: adapter-instructions
project: saying-hello
---

# Adapter Instructions

Project-specific adapter instruction fragment for the `saying-hello` adapter repo.

### Recommended Concrete Shape

For the `.NET` `command-line/all/no-framework` run, a good first concrete shape is:

- one adapter unit named `CommandLineGreeting`
- one method named `BuildMessage`
- one test file and test class named `CommandLineGreetingTests`
- a `Program.cs` entry point that does nothing except call `CommandLineGreeting.BuildMessage(args)` and print the result

The adapter should treat the first command-line argument as the optional name. If no argument is present, pass an empty string to the core library.

### 1. Red: Add An Adapter-Level Test

In a separate adapter repo, add a failing test that proves the chosen adapter delegates to the core logic correctly.

For this project, a good first test is:

```text
given first argument "Ada"
when BuildMessage is called
then the result is "Hello, Ada!"
```

Suggested generic test name:

```text
build_message_delegates_to_core_for_first_argument
```

### 2. Green: Add The Thin Surface Adapter

Add the thinnest possible adapter for the surface you are building:

- accept the smallest useful input for that surface
- pass that input to `greet`
- return, render, or print the result in the form that surface requires
- keep transport, parsing, and input/output code out of the core greeting logic

For this command-line run, a good green-state shape is:

```csharp
Console.WriteLine(CommandLineGreeting.BuildMessage(args));
```

with the parsing of `args` kept inside `CommandLineGreeting`.

### 3. Refactor

Clean up any remaining duplication while keeping the full suite green.

For this project, a useful next adapter test is:

```text
given no command-line arguments
when BuildMessage is called
then the result is "Hello!"
```
