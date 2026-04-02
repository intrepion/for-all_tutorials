<!-- breadcrumbs:start -->
[for-all_tutorials](../../../README.md) / [Projects](../../README.md) / [Saying Hello](../README.md) / Tutorial
<!-- breadcrumbs:end -->

# Tutorial

Instructions for building `saying-hello` from the spec using one setup path and the TDD walkthrough.

## Contents

- [TDD Walkthrough](tdd.md)

## Instructions

1. Read [Spec](../spec/README.md).
2. Choose one setup path from [setups/README.md](../../../setups/README.md).
3. Use that setup path to scaffold the project and test environment.
4. Open [TDD Walkthrough](tdd.md).
5. Follow the TDD steps in order without skipping ahead.
6. Keep the core greeting rules in one small tested unit.
7. Add the required `command-line/all` adapter only when the TDD walkthrough tells you to.
8. Verify that the finished project still matches [Spec](../spec/README.md).
9. Verify that coverage meets the thresholds defined in [Spec](../spec/README.md).

## Finish When

- the behavior matches [Spec](../spec/README.md)
- the project was built in a real red, green, refactor sequence
- the core `greet` logic is thoroughly tested
- the command-line adapter is thin and does not redefine greeting rules
- the coverage goals are met
