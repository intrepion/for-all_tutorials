---
partial_kind: adapter-instructions
project: saying-hello
testing: xunit
---

# Adapter Instructions

### Replace The Template Files

From the repo root, remove the placeholder files that `dotnet new` created:

```bash
rm src/SayingHello.CommandLine/Program.cs
rm tests/SayingHello.CommandLine.Tests/UnitTest1.cs
```

Create the replacement files now:

```bash
touch src/SayingHello.CommandLine/CommandLineGreeting.cs
touch src/SayingHello.CommandLine/Program.cs
touch tests/SayingHello.CommandLine.Tests/CommandLineGreetingTests.cs
```

Add NSubstitute to the adapter test project:

```bash
dotnet add tests/SayingHello.CommandLine.Tests/SayingHello.CommandLine.Tests.csproj package NSubstitute
```

### 1. Red: Add The First Failing Adapter Test

Create `tests/SayingHello.CommandLine.Tests/CommandLineGreetingTests.cs` with this exact code:

```csharp
using NSubstitute;
using SayingHello.CommandLine;
using SayingHello.Contracts;
using Xunit;

namespace SayingHello.CommandLine.Tests;

public sealed class CommandLineGreetingTests
{
    [Fact]
    public void Build_message_delegates_to_greeting_service_for_first_argument()
    {
        var greetingService = Substitute.For<IGreetingService>();
        greetingService.Greet("Ada").Returns("Hello, Ada!");
        var sut = new CommandLineGreeting(greetingService);

        var result = sut.BuildMessage(["Ada"]);

        Assert.Equal("Hello, Ada!", result);
        greetingService.Received(1).Greet("Ada");
    }
}
```

Run:

```bash
dotnet test
```

This should fail because `CommandLineGreeting` does not exist yet.

### 2. Green: Add The Thinnest Adapter Code

Create `src/SayingHello.CommandLine/CommandLineGreeting.cs` with this exact code:

```csharp
using SayingHello.Contracts;

namespace SayingHello.CommandLine;

public sealed class CommandLineGreeting(IGreetingService greetingService)
{
    private readonly IGreetingService _greetingService = greetingService;

    public string BuildMessage(string[] args)
    {
        return _greetingService.Greet(args[0]);
    }
}
```

Create `src/SayingHello.CommandLine/Program.cs` with this exact code:

```csharp
using SayingHello.CommandLine;
using SayingHello.Contracts;

var greetingService = new NotImplementedGreetingService();
var adapter = new CommandLineGreeting(greetingService);

Console.WriteLine(adapter.BuildMessage(args));

internal sealed class NotImplementedGreetingService : IGreetingService
{
    public string Greet(string name)
    {
        throw new NotImplementedException(
            "Finish the matching core tutorial, then replace this placeholder with the real core implementation."
        );
    }
}
```

Run:

```bash
dotnet test
```

This should pass.

### 3. Red: Add The No-Arguments Test

Replace `tests/SayingHello.CommandLine.Tests/CommandLineGreetingTests.cs` with this exact code:

```csharp
using NSubstitute;
using SayingHello.CommandLine;
using SayingHello.Contracts;
using Xunit;

namespace SayingHello.CommandLine.Tests;

public sealed class CommandLineGreetingTests
{
    [Fact]
    public void Build_message_delegates_to_greeting_service_for_first_argument()
    {
        var greetingService = Substitute.For<IGreetingService>();
        greetingService.Greet("Ada").Returns("Hello, Ada!");
        var sut = new CommandLineGreeting(greetingService);

        var result = sut.BuildMessage(["Ada"]);

        Assert.Equal("Hello, Ada!", result);
        greetingService.Received(1).Greet("Ada");
    }

    [Fact]
    public void Build_message_returns_generic_greeting_when_no_arguments_are_present()
    {
        var greetingService = Substitute.For<IGreetingService>();
        greetingService.Greet("").Returns("Hello!");
        var sut = new CommandLineGreeting(greetingService);

        var result = sut.BuildMessage([]);

        Assert.Equal("Hello!", result);
        greetingService.Received(1).Greet("");
    }
}
```

Run:

```bash
dotnet test
```

This should fail because `BuildMessage` indexes into an empty array.

### 4. Green: Handle The Empty Argument List

Replace `src/SayingHello.CommandLine/CommandLineGreeting.cs` with this exact code:

```csharp
using SayingHello.Contracts;

namespace SayingHello.CommandLine;

public sealed class CommandLineGreeting(IGreetingService greetingService)
{
    private readonly IGreetingService _greetingService = greetingService;

    public string BuildMessage(string[] args)
    {
        var name = args.Length > 0 ? args[0] : "";

        return _greetingService.Greet(name);
    }
}
```

Run:

```bash
dotnet test
```

This should pass.

### 5. Stop At The Contract Boundary

Run:

```bash
dotnet test
```

This should pass with both adapter tests green.

Leave the placeholder `NotImplementedGreetingService` in `Program.cs` for now. The matching core tutorial is the next step.
