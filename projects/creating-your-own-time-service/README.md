<!-- breadcrumbs:start -->
[for-all_tutorials](../../README.md) / [Projects](../README.md) / Creating Your Own Time Service
<!-- breadcrumbs:end -->

# Creating Your Own Time Service

Project home for the `creating-your-own-time-service` curriculum project.

## Contents

- [Spec](spec/README.md)
- [Tutorial](tutorial/README.md)

## Purpose

`creating-your-own-time-service` exists to teach:

- exposing a small JSON response from a web service adapter
- consuming that service from a separate client adapter
- keeping the service payload shape deterministic
- parsing a JSON time payload in the client path
- formatting a human-readable UTC time report
- separating clock access and HTTP transport from parsing and formatting logic
- keeping both service and client adapters thin while the core owns the payload and display rules

This project follows the shared workflow and output model in [Projects](../README.md#shared-workflow) and [Projects](../README.md#shared-output-model).
