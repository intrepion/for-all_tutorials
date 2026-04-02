<!-- breadcrumbs:start -->
[for-all_tutorials](../../../README.md) / [Projects](../../README.md) / [Counting the Number of Characters](../README.md) / Tutorial
<!-- breadcrumbs:end -->

# Tutorial

Instructions for building `count-characters` from the spec using one setup path and the TDD walkthrough.

## Contents

- [TDD Walkthrough](tdd.md)

## Instructions

1. Read [Spec](../spec/README.md).
2. Choose one setup path from [setups/README.md](../../../setups/README.md).
3. Create a separate core library repo for your chosen ecosystem, language, and test runner.
4. Use the setup guides for that core repo.
5. Open [TDD Walkthrough](tdd.md) and follow steps `1` through `21` in the core repo.
6. Create a separate adapter repo for the chosen surface and target.
7. Use the setup guides for that adapter repo.
8. Continue with [TDD Walkthrough](tdd.md) steps `22` through `24` in the adapter repo.
9. Verify that the finished core and adapter repos still match [Spec](../spec/README.md).
10. Verify that coverage meets the thresholds defined in [Spec](../spec/README.md).

## Finish When

- the behavior matches [Spec](../spec/README.md)
- the project was built in a real red, green, refactor sequence
- the core counting and message-formatting logic live in their own thoroughly tested repo
- the chosen adapter is thin and does not redefine counting or formatting rules
- the coverage goals are met
