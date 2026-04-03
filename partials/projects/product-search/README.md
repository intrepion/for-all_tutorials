---
partial_kind: project-root
project: product-search
manifest: manifest.yaml
---

# Product Search

Project home for the `product-search` curriculum project.

## Contents

- [Spec](spec/README.md)
- [Instructions](instructions/README.md)
- [Manifest](manifest.yaml)

## Purpose

`product-search` exists to teach:

- reading a canonical JSON inventory file in the adapter
- parsing JSON product records into a structured catalog
- locating one product by name from that parsed catalog
- formatting either a product report or a not-found message
- separating JSON parsing, lookup, and formatting from file input and retry-loop behavior
- keeping adapters thin while the core owns the inventory lookup rules

This project follows the shared workflow and output model in [Projects](../README.md#shared-workflow) and [Projects](../README.md#shared-output-model).

The supported compiled tutorial outputs for this project are declared in [Manifest](manifest.yaml).

