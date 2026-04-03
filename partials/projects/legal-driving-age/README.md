---
partial_kind: project-root
project: legal-driving-age
manifest: manifest.yaml
---

# Legal Driving Age

Project home for the fifteenth project app in the curriculum.

## Contents

- [Spec](spec/README.md)
- [Instructions](instructions/README.md)
- [Manifest](manifest.yaml)

## Purpose

`legal-driving-age` exists to teach:

- parsing a prompted age into a numeric value at the adapter boundary
- comparing that age against a fixed legal threshold
- branching between two exact output messages
- separating eligibility comparison from output formatting
- keeping adapters thin while the core owns the threshold and message rules

This project follows the shared workflow and output model in [Projects](../README.md#shared-workflow) and [Projects](../README.md#shared-output-model).

The supported compiled tutorial outputs for this project are declared in [Manifest](manifest.yaml).

