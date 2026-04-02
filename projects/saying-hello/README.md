<!-- breadcrumbs:start -->
[for-all_tutorials](../../README.md) / [Projects](../README.md) / Saying Hello
<!-- breadcrumbs:end -->

# Saying Hello

Project home for the first project app in the curriculum.

## Contents

- [Spec](spec/README.md)
- [Tutorial](tutorial/README.md)

## Purpose

`saying-hello` is the smallest project in the curriculum. It exists to teach:

- a tiny but real project contract
- true red, green, refactor workflow
- a thin adapter around a tested core logic unit
- the repo-wide coverage expectations

Use the spec as the source of truth, the tutorial as the teaching path, and `setups/` for stack-specific implementation details.

A full run of this project usually produces separate implementation repos outside this curriculum repo:

- one core library repo that owns `greet`
- one adapter repo for each chosen surface path
