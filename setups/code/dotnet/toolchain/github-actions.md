<!-- breadcrumbs:start -->
[for-all_tutorials](../../../../README.md) / [Setups](../../../README.md) / [Code](../../README.md) / [dotnet](../README.md) / [Toolchain](README.md) / github-actions.md
<!-- breadcrumbs:end -->

# GitHub Actions

Add a CI workflow after the repo builds and tests cleanly on your machine.

Create `.github/workflows/ci.yml` with a workflow like this:

```yaml
name: CI

on:
  push:
    branches:
      - main
  pull_request:

jobs:
  test:
    runs-on: ubuntu-latest

    steps:
      - name: Check out code
        uses: actions/checkout@v4

      - name: Set up .NET
        uses: actions/setup-dotnet@v4
        with:
          dotnet-version: 10.0.x

      - name: Restore
        run: dotnet restore

      - name: Test
        run: dotnet test --collect:"XPlat Code Coverage"
```

If you intentionally chose a different .NET SDK family for the repo, change `10.0.x` to match.

Once the workflow file exists:

1. commit it
2. push the repo
3. confirm the GitHub Actions run passes
