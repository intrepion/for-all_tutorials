<!-- breadcrumbs:start -->
[for-all_tutorials](../../../../../../README.md) / [Setups](../../../../../README.md) / [Code](../../../../README.md) / [python](../../../README.md) / [Frameworks](../../README.md) / [Web](../README.md) / [Full Stack](README.md) / fastapi-jinja2-htmx.md
<!-- breadcrumbs:end -->

# FastAPI + Jinja2 + HTMX

Reusable Python `web/full-stack` framework guidance for adapters that use FastAPI + Jinja2 + HTMX for async, server-rendered web interfaces.

## Use This After

- [Toolchain](../../../toolchain/README.md)
- one guide from [Testing](../../../testing/README.md) when one exists
- [Full Stack](../../../adapters/web/full-stack.md)

## Adapter Rule

A FastAPI + Jinja2 + HTMX adapter should stay thin:

- translate web input into calls to the core library
- translate core results into rendered templates, response models, or incremental page updates
- keep validation and service decisions in the core project logic

## Ready State

The FastAPI + Jinja2 + HTMX setup is ready when:

- the adapter repo already follows the generic `web/full-stack` setup
- FastAPI-, Jinja2-, and HTMX-specific host code delegates to the core library
- project rules remain in the separately tested core library repo
