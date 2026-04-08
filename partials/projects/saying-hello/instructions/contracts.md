---
partial_kind: contracts-instructions
project: saying-hello
---

# Contracts Instructions

### Replace The Template Files

From the repo root, remove the placeholder files that `dotnet new` created:

```bash
rm src/SayingHello.Contracts/Class1.cs
```

Create the replacement file now:

```bash
touch src/SayingHello.Contracts/IGreetingService.cs
```

### Define The Greeting Contract

Create `src/SayingHello.Contracts/IGreetingService.cs` with this exact code:

```csharp
namespace SayingHello.Contracts;

public interface IGreetingService
{
    string Greet(string name);
}
```

After the contract file exists, make this commit:

```bash
git add src/SayingHello.Contracts/IGreetingService.cs
git commit -m "Define Greeting Service Contract"
```
