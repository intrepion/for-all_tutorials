---
partial_kind: instructions-index
project: creating-your-own-time-service
---

# Instructions

Instructions for building `creating-your-own-time-service` from the spec using one setup path and separate core and adapter walkthroughs.

## Contents

- [Core Instructions](core.md)
- [Adapter Instructions](adapter.md)
- [Service Adapter Instructions](service-adapter.md)
- [Client Adapter Instructions](client-adapter.md)

## Project-Specific Flow

Follow the shared repository-creation and instruction workflow in [Projects](../../README.md#shared-repository-creation) and [Projects](../../README.md#shared-instruction-workflow), then use this project's instruction files:

- core repo: [Core Instructions](core.md)
- service adapter repo: [Service Adapter Instructions](service-adapter.md)
- client adapter repo: [Client Adapter Instructions](client-adapter.md)

For this project, the core repo should expose `build_current_time_payload`, `format_current_time_service_response`, `parse_current_time_service_response`, and `format_current_time_display`. A complete run should then build both a service adapter repo and a client adapter repo that delegate to those core functions.

This file is a source index, not a compiled tutorial. Use the shared finish checklist in [Projects](../../README.md#shared-finish-checklist) together with the project-specific rules in [Spec](../spec/README.md) and the declared outputs in [Manifest](../manifest.yaml).
