<!-- breadcrumbs:start -->
[for-all_tutorials](../../../README.md) / [Projects](../../README.md) / [Website Generator](../README.md) / Spec
<!-- breadcrumbs:end -->

# Spec

Canonical project contract for the `website-generator` project.

## Goal

Build a small project app that introduces:

- turning prompted values into a deterministic website skeleton plan
- generating the canonical contents of one `index.html` file
- choosing optional `js/` and `css/` directories from yes-or-no input
- formatting the created-path report in a canonical order
- test-first planning and HTML generation logic in small core functions
- thin surface adapters

## Core Logic Contract

The shared core contracts are:

```text
build_website_skeleton_plan(
  site_name: string,
  author: string,
  include_javascript_folder: boolean,
  include_css_folder: boolean
) -> website_skeleton_plan
render_index_html(site_name: string, author: string) -> string[]
format_creation_report(website_skeleton_plan) -> string[]
```

Where `website_skeleton_plan` contains:

```text
root_directory_path
index_html_path
include_javascript_folder
javascript_directory_path
include_css_folder
css_directory_path
index_html_lines
```

Canonical behavior:

- `build_website_skeleton_plan`:
  - uses `site_name` exactly as provided for the root directory name
  - sets `root_directory_path` to `./<site_name>`
  - sets `index_html_path` to `./<site_name>/index.html`
  - sets `javascript_directory_path` to `./<site_name>/js/`
  - sets `css_directory_path` to `./<site_name>/css/`
  - preserves the provided boolean values for whether the optional folders should exist
  - sets `index_html_lines` to the exact result of `render_index_html(site_name, author)`
- `render_index_html` returns these exact lines in this exact order:
  - `<!doctype html>`
  - `<html lang="en">`
  - `<head>`
  - `    <meta name="author" content="<author>">`
  - `    <title><site_name></title>`
  - `</head>`
  - `<body>`
  - `</body>`
  - `</html>`
- `format_creation_report` returns:
  - `Created <root_directory_path>`
  - `Created <index_html_path>`
  - `Created <javascript_directory_path>` when `include_javascript_folder` is true
  - `Created <css_directory_path>` when `include_css_folder` is true
  - the creation lines appear in that exact order

Examples:

- `render_index_html("awesomeco", "Max Power")` returns:
  - `<!doctype html>`
  - `<html lang="en">`
  - `<head>`
  - `    <meta name="author" content="Max Power">`
  - `    <title>awesomeco</title>`
  - `</head>`
  - `<body>`
  - `</body>`
  - `</html>`
- `build_website_skeleton_plan("awesomeco", "Max Power", true, true)` returns a plan whose important fields are:
  - `root_directory_path = "./awesomeco"`
  - `index_html_path = "./awesomeco/index.html"`
  - `javascript_directory_path = "./awesomeco/js/"`
  - `css_directory_path = "./awesomeco/css/"`
  - `index_html_lines = render_index_html("awesomeco", "Max Power")`
- `format_creation_report(build_website_skeleton_plan("awesomeco", "Max Power", true, true))` returns:
  - `Created ./awesomeco`
  - `Created ./awesomeco/index.html`
  - `Created ./awesomeco/js/`
  - `Created ./awesomeco/css/`

## Non-Goals

This project does not include:

- persistence beyond creating the requested directories and one `index.html` file
- localization
- authentication
- network access
- configuration files
- generating JavaScript or CSS files inside the optional directories
- linking CSS or JavaScript files from the generated HTML
- validating `site_name` as a safe filesystem slug beyond using the provided value as-is
- additional HTML metadata beyond the canonical author meta tag and title tag

## Surface Expectations

This spec follows the shared surface and setup-path rules in [Projects](../../README.md#shared-spec-conventions).

For this project, every tutorial run should adapt the same website generation plan, HTML content rules, and creation-report order instead of redefining them.

Adapters should:

- prompt for the site name
- prompt for the author
- prompt for whether a JavaScript folder should be created
- prompt for whether a CSS folder should be created
- convert the yes-or-no answers into booleans before calling the core
- pass the collected values to `build_website_skeleton_plan`
- create the root directory from the returned plan
- create the `index.html` file using the returned `index_html_lines`
- create the optional `js/` and `css/` directories according to the returned plan
- pass the plan to `format_creation_report`
- render the returned lines in order

For prompt-driven adapters, the prompts should be equivalent to:

```text
Site name:
Author:
Do you want a folder for JavaScript?
Do you want a folder for CSS?
```

## Output Repository Expectations

This project follows the shared output model in [Projects](../../README.md#shared-output-model).

For this project, the core repo owns `build_website_skeleton_plan`, `render_index_html`, and `format_creation_report`, and adapter repos must not reimplement them.

## Testing And Coverage Contract

This spec follows the shared coverage policy in [Projects](../../README.md#shared-coverage-policy).

This project should be built in a spec-driven and test-driven way.

Minimum test expectations:

- a test that `render_index_html("awesomeco", "Max Power")` returns the exact canonical HTML lines
- a test that `build_website_skeleton_plan("awesomeco", "Max Power", true, true)` returns the canonical root, file, and optional directory paths
- a test that `build_website_skeleton_plan` preserves `false` for optional folders when they are not requested
- a test that `format_creation_report` preserves the exact canonical creation line order
- tests for every adapter built in the chosen run that prove it parses the yes-or-no answers, creates the planned filesystem entries, writes the canonical `index.html` contents, and renders the creation report correctly
