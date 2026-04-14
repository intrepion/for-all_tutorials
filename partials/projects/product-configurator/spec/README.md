---
partial_kind: project-spec
project: product-configurator
---

# Spec

Canonical project contract for the `product-configurator` project.

## Goal

Build a small project app that introduces:

- switching among multiple 3D model variants
- exact-id variant selection
- per-variant camera orbit defaults
- per-variant exposure defaults
- thin adapters

## Canonical Variant Catalog

Every tutorial run for this project should use a variant catalog equivalent to:

```text
[
  {
    id: cube,
    src: assets/models/cube.glb,
    camera_orbit: 45deg 55deg 2.5m,
    exposure: 1.0
  },
  {
    id: tall-box,
    src: assets/models/tall-box.glb,
    camera_orbit: 35deg 65deg 3.2m,
    exposure: 1.1
  },
  {
    id: flat-box,
    src: assets/models/flat-box.glb,
    camera_orbit: 55deg 40deg 2.2m,
    exposure: 0.9
  }
]
```

## Core Logic Contract

The shared core contracts are:

```text
build_variant_catalog() -> variant[]
find_variant_by_exact_id(variant_catalog, variant_id: string) -> variant | null
build_variant_scene(variant) -> viewer_scene_config
```

Canonical behavior:

- `build_variant_catalog` returns the canonical three-variant catalog in order
- `find_variant_by_exact_id`:
  - compares `variant_id` by exact string match
  - does not trim, normalize, or case-fold `variant_id`
  - returns the first exact matching variant when present
  - returns `null` when no exact matching variant is present
- `build_variant_scene` copies the chosen variant's `src`, `camera_orbit`, and `exposure` into a viewer scene configuration

Examples:

- `find_variant_by_exact_id(build_variant_catalog(), "cube").camera_orbit` returns `45deg 55deg 2.5m`
- `find_variant_by_exact_id(build_variant_catalog(), "tall-box").exposure` returns `1.1`
- `find_variant_by_exact_id(build_variant_catalog(), "Cube")` returns `null`

## Non-Goals

This project does not include:

- persistence of the last selected variant
- remote model downloads
- progress indicators
- AR launch behavior

## Surface Expectations

This spec follows the shared surface and setup-path rules in [Projects](../../README.md#shared-spec-conventions).

For this project, every tutorial run should adapt the same variant catalog and exact-id selection rules instead of redefining per-model defaults in the adapter.

Adapters should:

- render the canonical variant choices
- react to a chosen variant id
- delegate variant lookup and scene building to the core
- apply each chosen variant's orbit and exposure before render

## Output Repository Expectations

This project follows the shared output model in [Projects](../../README.md#shared-output-model).

For this project, the core repo owns the variant catalog and default scene values and adapter repos must not quietly substitute different defaults for the same variant ids.

## Testing And Coverage Contract

This spec follows the shared coverage policy in [Projects](../../README.md#shared-coverage-policy).

This project should be built in a spec-driven and test-driven way.

Minimum test expectations:

- tests for the canonical variant catalog contents and order
- tests for exact-id lookup success and failure
- tests for each variant's camera orbit and exposure values
- tests for every adapter built in the chosen run that prove model switching applies the core-owned scene for the selected variant
