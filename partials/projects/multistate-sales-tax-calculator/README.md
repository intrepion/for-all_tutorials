---
partial_kind: project-root
project: multistate-sales-tax-calculator
manifest: manifest.yaml
---

# Multistate Sales Tax Calculator

Project home for the nineteenth project app in the curriculum.

## Contents

- [Spec](spec/README.md)
- [Instructions](instructions/README.md)
- [Manifest](manifest.yaml)

## Purpose

`multistate-sales-tax-calculator` exists to teach:

- parsing an order amount into exact cents at the adapter boundary
- branching first by state and then by county for nested tax rules
- applying different tax rates for Wisconsin, Wisconsin counties, and Illinois
- rounding tax to the nearest cent with deterministic half-up rounding
- separating tax calculation from output formatting
- keeping adapters thin while the core owns the nested decision and money rules

This project follows the shared workflow and output model in [Projects](../README.md#shared-workflow) and [Projects](../README.md#shared-output-model).

The supported compiled tutorial outputs for this project are declared in [Manifest](manifest.yaml).

