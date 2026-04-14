---
partial_kind: project-root
project: team-task-board
manifest: manifest.yaml
---

# Team Task Board

Project home for the `team-task-board` curriculum project.

## Contents

- [Spec](spec/README.md)
- [Instructions](instructions/README.md)
- [Manifest](manifest.yaml)

## Purpose

`team-task-board` exists to teach:

- persisting shared task records with owners and visibility
- authenticating the current principal as anonymous, user, or admin
- authorizing different read and delete behaviors by role
- filtering visible tasks for the current principal
- separating authentication and authorization rules from adapter-managed login and storage flow
- keeping adapters thin while the core owns task visibility and deletion permissions

This project follows the shared workflow and output model in [Projects](../README.md#shared-workflow) and [Projects](../README.md#shared-output-model).

The supported compiled tutorial outputs for this project are declared in [Manifest](manifest.yaml).
