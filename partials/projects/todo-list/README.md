---
partial_kind: project-root
project: todo-list
manifest: manifest.yaml
---

# Todo List

Project home for the `todo-list` curriculum project.

## Contents

- [Spec](spec/README.md)
- [Instructions](instructions/README.md)
- [Manifest](manifest.yaml)

## Purpose

`todo-list` exists to teach:

- collecting repeated task input through an adapter-managed interaction flow
- stopping an input loop with a blank entry
- persisting task data in a durable storage mechanism such as local-file JSON or a database
- parsing and serializing a canonical JSON task-list representation
- removing a completed task by exact text
- separating storage and interaction flow from deterministic list logic
- keeping adapters thin while the core owns the task-list rules

This project follows the shared workflow and output model in [Projects](../README.md#shared-workflow) and [Projects](../README.md#shared-output-model).

The supported compiled tutorial outputs for this project are declared in [Manifest](manifest.yaml).
