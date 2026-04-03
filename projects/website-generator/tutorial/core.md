<!-- breadcrumbs:start -->
[for-all_tutorials](../../../../README.md) / [Projects](../../../README.md) / [Website Generator](../../README.md) / [Tutorial](../README.md) / core.md
<!-- breadcrumbs:end -->

# Core Walkthrough

Project-specific red, green, refactor sequence for the `website-generator` core repo.

### 1. Red: Write The First Failing Test

Start with the canonical HTML output:

```text
given site_name awesomeco
and author Max Power
when render_index_html is called
then the returned lines are the exact canonical html skeleton with the title and author meta tag filled in
```

At this point, `render_index_html` may not exist yet. That is fine. The first run should fail either at compile time or at test execution time.

Suggested generic test name:

```text
render_index_html_returns_the_canonical_html_skeleton
```

### 2. Green: Make The First Test Pass

Create the smallest production code that makes the first test pass.

At this stage, it is acceptable if the code only handles this first case. Do not jump ahead and implement plan building or report formatting before those tests exist.

### 3. Refactor

Do the smallest cleanup that keeps the first test green.

### 4. Red: Add The Website Skeleton Plan

Write the next failing test:

```text
given site_name awesomeco
and author Max Power
and include_javascript_folder true
and include_css_folder true
when build_website_skeleton_plan is called
then the returned plan contains:
./awesomeco
./awesomeco/index.html
./awesomeco/js/
./awesomeco/css/
and index_html_lines equal the canonical html skeleton
```

Suggested generic test name:

```text
build_website_skeleton_plan_returns_the_canonical_paths_and_html_lines
```

### 5. Green: Make The Plan Test Pass

Make the smallest safe change that gets both tests green.

### 6. Refactor

Clean up the implementation while keeping both tests green.

### 7. Red: Add The Optional-Folder Cases

Write the next failing test:

```text
given include_javascript_folder false
and include_css_folder false
when build_website_skeleton_plan is called
then the returned plan still contains the canonical root and index paths
and preserves false for both optional folder flags
```

Suggested generic test name:

```text
build_website_skeleton_plan_preserves_false_for_optional_folders
```

### 8. Green: Make The Optional-Folder Test Pass

Make it pass while keeping the earlier tests green.

### 9. Refactor

Clean up the implementation while keeping all HTML and plan tests green.

### 10. Red: Add The Creation Report

Write the next failing test:

```text
given the canonical plan for awesomeco with javascript and css folders
when format_creation_report is called
then the returned lines are:
Created ./awesomeco
Created ./awesomeco/index.html
Created ./awesomeco/js/
Created ./awesomeco/css/
```

Suggested generic test name:

```text
format_creation_report_returns_the_canonical_creation_lines
```

### 11. Green: Make The Report Test Pass

Make it pass while keeping the earlier tests green.

### 12. Refactor The Core Logic

Now that the full behavior is covered, refactor toward a small, clear core abstraction.

The exact shape can vary by stack:

- three functions in one module
- one service object with plan, html, and report methods
- one module plus a small immutable plan type

The important thing is that website planning, canonical HTML generation, and creation-report formatting stay small, explicit, and directly testable.
