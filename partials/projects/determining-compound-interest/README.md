---
partial_kind: project-root
project: determining-compound-interest
manifest: manifest.yaml
---

# Determining Compound Interest

Project home for the thirteenth project app in the curriculum.

## Contents

- [Spec](spec/README.md)
- [Instructions](instructions/README.md)
- [Manifest](manifest.yaml)

## Purpose

`determining-compound-interest` exists to teach:

- parsing a whole-number principal amount at the adapter boundary
- parsing an entered interest rate into an exact scaled core value
- computing compound growth with the exponent form of the investment formula
- rounding the final accrued amount to cents with deterministic half-up rounding
- separating compound-interest calculation from output formatting
- keeping adapters thin while the core owns the scaling, exponent-based calculation, and reporting rules

This project follows the shared workflow and output model in [Projects](../README.md#shared-workflow) and [Projects](../README.md#shared-output-model).

The supported compiled tutorial outputs for this project are declared in [Manifest](manifest.yaml).

