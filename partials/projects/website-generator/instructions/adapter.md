---
partial_kind: adapter-instructions
project: website-generator
---

# Adapter Instructions

Project-specific adapter instruction fragment for the `website-generator` adapter repo.

### 1. Red: Add An Adapter-Level Test

In a separate adapter repo, add a failing test that proves the chosen adapter collects the four prompted values, parses the yes-or-no answers, delegates planning and report formatting to the core logic, creates the planned directories and `index.html` file, writes the canonical `index.html` contents, and renders the returned creation lines in order.

### 2. Green: Add The Thin Surface Adapter

Add the thinnest possible adapter for the surface you are building:

- render the equivalents of:
  - `Site name:`
  - `Author:`
  - `Do you want a folder for JavaScript?`
  - `Do you want a folder for CSS?`
- convert the two yes-or-no answers into booleans
- pass the collected values to `build_website_skeleton_plan`
- create the planned root directory
- create the planned `index.html` file using the returned `index_html_lines`
- create the optional `js/` and `css/` directories when requested by the plan
- pass the plan to `format_creation_report`
- render the returned lines in order
- keep prompt handling and filesystem side effects out of the core website-generation logic

### 3. Refactor

Clean up any remaining duplication while keeping the full suite green.

