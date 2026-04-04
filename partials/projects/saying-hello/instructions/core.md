---
partial_kind: core-instructions
project: saying-hello
---

# Core Instructions

### Replace The Template Files

From the repo root, remove the placeholder files that `dotnet new` created:

```bash
rm src/SayingHello/Class1.cs
rm tests/SayingHello.Tests/UnitTest1.cs
```

You will replace them with these exact files:

- `src/SayingHello/Greeting.cs`
- `tests/SayingHello.Tests/GreetingTests.cs`

### 1. Red: Add The First Failing Test

Create `tests/SayingHello.Tests/GreetingTests.cs` with this exact code:

```csharp
using SayingHello;
using Xunit;

namespace SayingHello.Tests;

public sealed class GreetingTests
{
    [Fact]
    public void Greet_returns_personalized_greeting_for_non_empty_name()
    {
        var result = Greeting.Greet("Ada");

        Assert.Equal("Hello, Ada!", result);
    }
}
```

Run:

```bash
dotnet test
```

This should fail because `Greeting` does not exist yet.

### 2. Green: Add The Smallest Production Code

Create `src/SayingHello/Greeting.cs` with this exact code:

```csharp
namespace SayingHello;

public static class Greeting
{
    public static string Greet(string name)
    {
        return $"Hello, {name}!";
    }
}
```

Run:

```bash
dotnet test
```

This should pass.

### 3. Red: Add The Trimming Test

Replace `tests/SayingHello.Tests/GreetingTests.cs` with this exact code:

```csharp
using SayingHello;
using Xunit;

namespace SayingHello.Tests;

public sealed class GreetingTests
{
    [Fact]
    public void Greet_returns_personalized_greeting_for_non_empty_name()
    {
        var result = Greeting.Greet("Ada");

        Assert.Equal("Hello, Ada!", result);
    }

    [Fact]
    public void Greet_trims_leading_and_trailing_whitespace()
    {
        var result = Greeting.Greet("  Ada  ");

        Assert.Equal("Hello, Ada!", result);
    }
}
```

Run:

```bash
dotnet test
```

This should fail.

### 4. Green: Make The Trimming Test Pass

Replace `src/SayingHello/Greeting.cs` with this exact code:

```csharp
namespace SayingHello;

public static class Greeting
{
    public static string Greet(string name)
    {
        var trimmedName = name.Trim();

        return $"Hello, {trimmedName}!";
    }
}
```

Run:

```bash
dotnet test
```

This should pass.

### 5. Red: Add The Empty-String Test

Replace `tests/SayingHello.Tests/GreetingTests.cs` with this exact code:

```csharp
using SayingHello;
using Xunit;

namespace SayingHello.Tests;

public sealed class GreetingTests
{
    [Fact]
    public void Greet_returns_personalized_greeting_for_non_empty_name()
    {
        var result = Greeting.Greet("Ada");

        Assert.Equal("Hello, Ada!", result);
    }

    [Fact]
    public void Greet_trims_leading_and_trailing_whitespace()
    {
        var result = Greeting.Greet("  Ada  ");

        Assert.Equal("Hello, Ada!", result);
    }

    [Fact]
    public void Greet_returns_generic_greeting_for_empty_string()
    {
        var result = Greeting.Greet("");

        Assert.Equal("Hello!", result);
    }
}
```

Run:

```bash
dotnet test
```

This should fail.

### 6. Green: Make The Empty-String Test Pass

Replace `src/SayingHello/Greeting.cs` with this exact code:

```csharp
namespace SayingHello;

public static class Greeting
{
    public static string Greet(string name)
    {
        if (name.Length == 0)
        {
            return "Hello!";
        }

        var trimmedName = name.Trim();

        return $"Hello, {trimmedName}!";
    }
}
```

Run:

```bash
dotnet test
```

This should pass.

### 7. Red: Add The Whitespace-Only Test

Replace `tests/SayingHello.Tests/GreetingTests.cs` with this exact code:

```csharp
using SayingHello;
using Xunit;

namespace SayingHello.Tests;

public sealed class GreetingTests
{
    [Fact]
    public void Greet_returns_personalized_greeting_for_non_empty_name()
    {
        var result = Greeting.Greet("Ada");

        Assert.Equal("Hello, Ada!", result);
    }

    [Fact]
    public void Greet_trims_leading_and_trailing_whitespace()
    {
        var result = Greeting.Greet("  Ada  ");

        Assert.Equal("Hello, Ada!", result);
    }

    [Fact]
    public void Greet_returns_generic_greeting_for_empty_string()
    {
        var result = Greeting.Greet("");

        Assert.Equal("Hello!", result);
    }

    [Fact]
    public void Greet_returns_generic_greeting_for_whitespace_only_input()
    {
        var result = Greeting.Greet("   ");

        Assert.Equal("Hello!", result);
    }
}
```

Run:

```bash
dotnet test
```

This should fail.

### 8. Green: Finish The Core Behavior

Replace `src/SayingHello/Greeting.cs` with this exact code:

```csharp
namespace SayingHello;

public static class Greeting
{
    public static string Greet(string name)
    {
        var trimmedName = name.Trim();

        if (trimmedName.Length == 0)
        {
            return "Hello!";
        }

        return $"Hello, {trimmedName}!";
    }
}
```

Run:

```bash
dotnet test
```

This should pass with all four tests green.
