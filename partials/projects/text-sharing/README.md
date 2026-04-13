---
partial_kind: project-root
project: text-sharing
manifest: manifest.yaml
---

# Text Sharing

Project home for the `text-sharing` curriculum project.

## Contents

- [Spec](spec/README.md)
- [Instructions](instructions/README.md)
- [Manifest](manifest.yaml)

## Purpose

`text-sharing` exists to teach:

- building an app that accepts text input and exposes content-display and edit flows
- persisting shared text snippets in a local data store
- generating local share URLs from adapter-chosen snippet IDs
- loading a saved snippet from its share URL
- pre-filling an edit experience from an existing snippet
- separating routing, persistence, and surface interaction from deterministic snippet logic
- keeping adapters thin while the core owns the snippet rules

This project follows the shared workflow and output model in [Projects](../README.md#shared-workflow) and [Projects](../README.md#shared-output-model).

The supported compiled tutorial outputs for this project are declared in [Manifest](manifest.yaml).
