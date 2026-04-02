<!-- breadcrumbs:start -->
[for-all_tutorials](../../README.md) / [Projects](../README.md) / Counting the Number of Characters
<!-- breadcrumbs:end -->

# Counting the Number of Characters

Project home for the second project app in the curriculum.

## Contents

- [Spec](spec/README.md)
- [Tutorial](tutorial/README.md)

## Purpose

`count-characters` exists to teach:

- turning one text input into one derived output
- preserving user input exactly, including whitespace, instead of normalizing it away
- separating counting logic from message-formatting logic
- keeping adapters thin while the core owns the counting and formatting rules

Use the spec as the source of truth, the tutorial as the teaching path, and `setups/` for stack-specific implementation details.

A full run of this project usually produces separate implementation repos outside this curriculum repo:

- one core library repo that owns the counting and formatting rules
- one adapter repo for each chosen surface path
