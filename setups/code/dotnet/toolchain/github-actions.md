<!-- breadcrumbs:start -->
[for-all_tutorials](../../../../README.md) / [Setups](../../../README.md) / [Code](../../README.md) / [dotnet](../README.md) / [Toolchain](README.md) / github-actions.md
<!-- breadcrumbs:end -->

# GitHub Actions

Add a CI workflow after the repo builds and tests cleanly on your machine.

Create the workflow file:

```bash
mkdir -p .github/workflows
touch .github/workflows/ci.yml
```

Then put this exact workflow in `.github/workflows/ci.yml`:

```yaml
name: CI

on:
  push:
    branches:
      - main
  pull_request:
    branches:
      - main

jobs:
  test:
    runs-on: ubuntu-latest

    steps:
      - name: Check out code
        uses: actions/checkout@v6

      - name: Set up .NET
        uses: actions/setup-dotnet@v5
        with:
          dotnet-version: 10.0.x

      - name: Verify formatting
        run: dotnet format --verify-no-changes

      - name: Restore
        run: dotnet restore

      - name: Test
        run: dotnet test
```

This workflow runs for pull requests that target `main` and for pushes to `main`, including merge commits.

If you intentionally chose a different .NET SDK family for the repo, change `10.0.x` to match.

Once the workflow file exists:

1. commit it
2. push the repo
3. confirm the GitHub Actions run passes
