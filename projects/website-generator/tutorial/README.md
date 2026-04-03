<!-- breadcrumbs:start -->
[for-all_tutorials](../../../README.md) / [Projects](../../README.md) / [Website Generator](../README.md) / Tutorial
<!-- breadcrumbs:end -->

# Tutorial

Instructions for building `website-generator` from the spec using one setup path and separate core and adapter walkthroughs.

## Contents

- [Core Walkthrough](core.md)
- [Adapter Walkthrough](adapter.md)

## Project-Specific Flow

Follow the shared tutorial workflow in [Projects](../../README.md#shared-tutorial-workflow), then use this project's walkthrough files:

- core repo: [Core Walkthrough](core.md)
- adapter repo: [Adapter Walkthrough](adapter.md)

For this project, the core repo should expose `build_website_skeleton_plan`, `render_index_html`, and `format_creation_report`, and the adapter repo should prompt for the generation values, create the planned filesystem entries, write the canonical `index.html`, and render the canonical creation report by delegating planning and formatting to those core functions.

Use the shared finish checklist in [Projects](../../README.md#shared-finish-checklist) together with the project-specific rules in [Spec](../spec/README.md).
