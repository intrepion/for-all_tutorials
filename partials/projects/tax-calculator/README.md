---
partial_kind: project-root
project: tax-calculator
manifest: manifest.yaml
---

# Tax Calculator

Project home for the fourteenth project app in the curriculum.

## Contents

- [Spec](spec/README.md)
- [Instructions](instructions/README.md)
- [Manifest](manifest.yaml)

## Purpose

`tax-calculator` exists to teach:

- parsing an order amount into exact cents at the adapter boundary
- branching on a simple state rule in the core logic
- applying a fixed `5.5%` tax rate only for Wisconsin orders
- rounding tax to the nearest cent with deterministic half-up rounding
- separating tax calculation from output formatting
- keeping adapters thin while the core owns the branch and money rules

This project follows the shared workflow and output model in [Projects](../README.md#shared-workflow) and [Projects](../README.md#shared-output-model).

The supported compiled tutorial outputs for this project are declared in [Manifest](manifest.yaml).

