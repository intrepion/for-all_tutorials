<!-- breadcrumbs:start -->
[for-all_tutorials](../../../README.md) / [Projects](../../README.md) / [Movie Recommendations](../README.md) / Tutorial
<!-- breadcrumbs:end -->

# Tutorial

Instructions for building `movie-recommendations` from the spec using one setup path and separate core and adapter walkthroughs.

## Contents

- [Core Walkthrough](core.md)
- [Adapter Walkthrough](adapter.md)

## Project-Specific Flow

Follow the shared tutorial workflow in [Projects](../../README.md#shared-tutorial-workflow), then use this project's walkthrough files:

- core repo: [Core Walkthrough](core.md)
- adapter repo: [Adapter Walkthrough](adapter.md)

For this project, the core repo should expose `parse_movie_details_response`, `format_movie_details_report`, and `build_audience_recommendation`, and the adapter repo should fetch one movie-details response from the chosen approved provider and render the canonical report and any recommendation by delegating to those core functions.

Use the shared finish checklist in [Projects](../../README.md#shared-finish-checklist) together with the project-specific rules in [Spec](../spec/README.md).
