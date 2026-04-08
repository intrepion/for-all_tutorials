---
partial_kind: core-instructions
project: saying-hello
testing: tunit
---

# Core Instructions

### Replace The Template Files

From the repo root, remove the placeholder files that `dotnet new` created:

```bash
rm src/SayingHello/Class1.cs
rm tests/SayingHello.Tests/BasicTests.cs
rm tests/SayingHello.Tests/Calculator.cs
rm tests/SayingHello.Tests/DataDrivenTests.cs
rm tests/SayingHello.Tests/DependencyInjectionTests.cs
rm tests/SayingHello.Tests/HooksAndLifecycle.cs
```

Create the replacement files now:

```bash
touch src/SayingHello/GreetingService.cs
touch tests/SayingHello.Tests/GreetingServiceTests.cs
```

### 1. Red: Add The First Failing Test

Create `tests/SayingHello.Tests/GreetingServiceTests.cs` with this exact code:

```csharp
using System.Threading.Tasks;
using SayingHello;
using TUnit.Assertions;
using TUnit.Assertions.Extensions;
using TUnit.Core;

namespace SayingHello.Tests;

public sealed class GreetingServiceTests
{
    [Test]
    public async Task Greet_returns_personalized_greeting_for_non_empty_name()
    {
        var sut = new GreetingService();

        var result = sut.Greet("Ada");

        await Assert.That(result).IsEqualTo("Hello, Ada!");
    }
}
```

Run:

```bash
dotnet test
```

This should fail because `GreetingService` does not exist yet.

### 2. Green: Add The Smallest Production Code

Create `src/SayingHello/GreetingService.cs` with this exact code:

```csharp
using SayingHello.Contracts;

namespace SayingHello;

public sealed class GreetingService : IGreetingService
{
    public string Greet(string name)
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

Replace `tests/SayingHello.Tests/GreetingServiceTests.cs` with this exact code:

```csharp
using System.Threading.Tasks;
using SayingHello;
using TUnit.Assertions;
using TUnit.Assertions.Extensions;
using TUnit.Core;

namespace SayingHello.Tests;

public sealed class GreetingServiceTests
{
    [Test]
    public async Task Greet_returns_personalized_greeting_for_non_empty_name()
    {
        var sut = new GreetingService();

        var result = sut.Greet("Ada");

        await Assert.That(result).IsEqualTo("Hello, Ada!");
    }

    [Test]
    public async Task Greet_trims_leading_and_trailing_whitespace()
    {
        var sut = new GreetingService();

        var result = sut.Greet("  Ada  ");

        await Assert.That(result).IsEqualTo("Hello, Ada!");
    }
}
```

Run:

```bash
dotnet test
```

This should fail.

### 4. Green: Make The Trimming Test Pass

Replace `src/SayingHello/GreetingService.cs` with this exact code:

```csharp
using SayingHello.Contracts;

namespace SayingHello;

public sealed class GreetingService : IGreetingService
{
    public string Greet(string name)
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

Replace `tests/SayingHello.Tests/GreetingServiceTests.cs` with this exact code:

```csharp
using System.Threading.Tasks;
using SayingHello;
using TUnit.Assertions;
using TUnit.Assertions.Extensions;
using TUnit.Core;

namespace SayingHello.Tests;

public sealed class GreetingServiceTests
{
    [Test]
    public async Task Greet_returns_personalized_greeting_for_non_empty_name()
    {
        var sut = new GreetingService();

        var result = sut.Greet("Ada");

        await Assert.That(result).IsEqualTo("Hello, Ada!");
    }

    [Test]
    public async Task Greet_trims_leading_and_trailing_whitespace()
    {
        var sut = new GreetingService();

        var result = sut.Greet("  Ada  ");

        await Assert.That(result).IsEqualTo("Hello, Ada!");
    }

    [Test]
    public async Task Greet_returns_generic_greeting_for_empty_string()
    {
        var sut = new GreetingService();

        var result = sut.Greet("");

        await Assert.That(result).IsEqualTo("Hello!");
    }
}
```

Run:

```bash
dotnet test
```

This should fail.

### 6. Green: Make The Empty-String Test Pass

Replace `src/SayingHello/GreetingService.cs` with this exact code:

```csharp
using SayingHello.Contracts;

namespace SayingHello;

public sealed class GreetingService : IGreetingService
{
    public string Greet(string name)
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

Replace `tests/SayingHello.Tests/GreetingServiceTests.cs` with this exact code:

```csharp
using System.Threading.Tasks;
using SayingHello;
using TUnit.Assertions;
using TUnit.Assertions.Extensions;
using TUnit.Core;

namespace SayingHello.Tests;

public sealed class GreetingServiceTests
{
    [Test]
    public async Task Greet_returns_personalized_greeting_for_non_empty_name()
    {
        var sut = new GreetingService();

        var result = sut.Greet("Ada");

        await Assert.That(result).IsEqualTo("Hello, Ada!");
    }

    [Test]
    public async Task Greet_trims_leading_and_trailing_whitespace()
    {
        var sut = new GreetingService();

        var result = sut.Greet("  Ada  ");

        await Assert.That(result).IsEqualTo("Hello, Ada!");
    }

    [Test]
    public async Task Greet_returns_generic_greeting_for_empty_string()
    {
        var sut = new GreetingService();

        var result = sut.Greet("");

        await Assert.That(result).IsEqualTo("Hello!");
    }

    [Test]
    public async Task Greet_returns_generic_greeting_for_whitespace_only_input()
    {
        var sut = new GreetingService();

        var result = sut.Greet("   ");

        await Assert.That(result).IsEqualTo("Hello!");
    }
}
```

Run:

```bash
dotnet test
```

This should fail.

### 8. Green: Finish The Core Behavior

Replace `src/SayingHello/GreetingService.cs` with this exact code:

```csharp
using SayingHello.Contracts;

namespace SayingHello;

public sealed class GreetingService : IGreetingService
{
    public string Greet(string name)
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
