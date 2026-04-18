---
partial_kind: projects-root
source_scope: project-partials
---

# Projects

Project-source slices for future compiled tutorials.

Some project manifests in this folder already declare active compiled tutorial outputs.

Other project manifests are intentionally spec-first and keep `compiled_outputs: []` until their first tutorial path is ready.

## Contents

- [Adding Numbers](adding-numbers/README.md)
- [Anagram Checker](anagram-checker/README.md)
- [AR Preview](ar-preview/README.md)
- [Area of a Rectangular Room](area-of-a-rectangular-room/README.md)
- [Blood Alcohol Calculator](blood-alcohol-calculator/README.md)
- [BMI Calculator](bmi-calculator/README.md)
- [Comparing Numbers](comparing-numbers/README.md)
- [Computing Simple Interest](computing-simple-interest/README.md)
- [Computing Statistics](computing-statistics/README.md)
- [Counting the Number of Characters](count-characters/README.md)
- [Creating Your Own Time Service](creating-your-own-time-service/README.md)
- [Currency Conversion](currency-conversion/README.md)
- [Determining Compound Interest](determining-compound-interest/README.md)
- [Employee List Removal](employee-list-removal/README.md)
- [Filtering Records](filtering-records/README.md)
- [Filtering Values](filtering-values/README.md)
- [Flickr Photo Search](flickr-photo-search/README.md)
- [Grabbing the Weather](grabbing-the-weather/README.md)
- [Guess the Number Game](guess-the-number-game/README.md)
- [Handling Bad Input](handling-bad-input/README.md)
- [Karvonen Heart Rate](karvonen-heart-rate/README.md)
- [Legal Driving Age](legal-driving-age/README.md)
- [Lighting Studio](lighting-studio/README.md)
- [Mad Lib](mad-lib/README.md)
- [Magic 8 Ball](magic-8-ball/README.md)
- [Months to Pay Off a Credit Card](months-to-pay-off-a-credit-card/README.md)
- [Model Showroom](model-showroom/README.md)
- [Movie Recommendations](movie-recommendations/README.md)
- [Multiplication Table](multiplication-table/README.md)
- [Multistate Sales Tax Calculator](multistate-sales-tax-calculator/README.md)
- [Name Sorter](name-sorter/README.md)
- [Numbers to Names](numbers-to-names/README.md)
- [Paint Calculator](paint-calculator/README.md)
- [Parsing a Data File](parsing-a-data-file/README.md)
- [Password Generator](password-generator/README.md)
- [Password Strength Indicator](password-strength-indicator/README.md)
- [Password Validation](password-validation/README.md)
- [Picking a Winner](picking-a-winner/README.md)
- [Pizza Party](pizza-party/README.md)
- [Printing Quotes](printing-quotes/README.md)
- [Product Configurator](product-configurator/README.md)
- [Product Search](product-search/README.md)
- [Prism Face Cropper](prism-face-cropper/README.md)
- [Prism Face Selector](prism-face-selector/README.md)
- [Prism Net Layout](prism-net-layout/README.md)
- [Prism Scene Editor](prism-scene-editor/README.md)
- [Prism Widget Renderer](prism-widget-renderer/README.md)
- [Pushing Notes to Firebase](pushing-notes-to-firebase/README.md)
- [Remote Model Loader](remote-model-loader/README.md)
- [Retirement Calculator](retirement-calculator/README.md)
- [Saying Hello](saying-hello/README.md)
- [Self-Checkout](self-checkout/README.md)
- [Simple Math](simple-math/README.md)
- [Sorting Records](sorting-records/README.md)
- [Tax Calculator](tax-calculator/README.md)
- [Team Task Board](team-task-board/README.md)
- [Temperature Converter](temperature-converter/README.md)
- [Text Sharing](text-sharing/README.md)
- [Todo List](todo-list/README.md)
- [Tracking Inventory](tracking-inventory/README.md)
- [Trivia App](trivia-app/README.md)
- [Troubleshooting Car Issues](troubleshooting-car-issues/README.md)
- [URL Shortener](url-shortener/README.md)
- [Validating Inputs](validating-inputs/README.md)
- [Website Generator](website-generator/README.md)
- [Who's in Space?](whos-in-space/README.md)
- [Word Finder](word-finder/README.md)
- [Word Frequency Finder](word-frequency-finder/README.md)

## Shared Workflow

Every project slice in this folder follows the same high-level pattern:

1. Use the project `spec/README.md` as the source of truth.
2. Use the project `instructions/README.md`, `instructions/core.md`, and the relevant adapter instruction fragment or fragments in `instructions/` as the source material for future compiled tutorials.
3. Use `partials/setups/` for stack-specific implementation details.
4. Build the core logic first in a separate core library repo.
5. Add one or more separate adapter repos only after the core logic is tested.

## Shared Output Model

The code produced from these tutorials should live outside this curriculum repo.

The default model is:

- one core library repo owns the project rules
- one adapter repo owns each chosen surface and target
- adapter repos depend on the core library instead of copying its logic
- adapter repo names always include a storage slot, using one canonical value from the shared storage taxonomy

## Shared Repository Creation

Before starting a project walkthrough, create the output repository that walkthrough will target.

For a core repo:

- use the name shape `<project>-<ecosystem>-<language>-<test-framework>-core`
- use a short description that says it is the manual, spec-driven, test-driven core library for that project and stack

For an adapter repo:

- use the name shape `<project>-<ecosystem>-<language>-<test-framework>-<storage>-<surface>-<target>-<framework>`
- use a short description that says it is the manual adapter for that project and stack and that it consumes the separately tested core library

Recommended sequence:

1. choose the repo role: core or adapter
2. choose the repo name from the shared naming shape
3. create the remote repo in the forge you use, with a short one-sentence description
4. create or clone the local working copy for that repo
5. then start the setup guides and the project walkthrough for that repo

## Shared Spec Conventions

Project specs in this folder share these rules:

- a single run only needs one chosen surface and target path
- project specs should describe the project behavior and adapter responsibilities without unnecessarily fixing one surface, target, framework, or local persistence shape
- when persistence is part of a project and the project is not defined around a named external service, adapters may choose either local-file storage or database-backed storage
- when a chosen storage engine is not itself raw JSON text, the adapter may translate between that storage representation and any canonical JSON examples used by the spec
- the currently supported setup paths belong in `partials/setups/`, not in each spec
- adapters must not redefine the core project rules

## Shared Coverage Policy

Unless a local spec explicitly declares an exception, projects in this folder use this coverage policy:

- baseline coverage is `90%` code and `85%` branch
- core validation and service logic should target `100%` code and `100%` branch

Local specs should only restate coverage numbers when a project has a special-case requirement that differs from this shared policy.

## Shared Instruction Workflow

Project instruction indexes in this folder share this flow:

1. read the local spec
2. choose one setup path
3. create a separate core library repo using [Shared Repository Creation](#shared-repository-creation)
4. use the setup guides for that core repo
5. follow the local core walkthrough for the core repo
6. create a separate adapter repo for the chosen surface and target using [Shared Repository Creation](#shared-repository-creation)
7. use the setup guides for that adapter repo
8. follow the local adapter walkthrough for the adapter repo
9. verify that the finished repos still match the local spec

## Shared Finish Checklist

A project run is complete when:

- the behavior matches the local spec
- the project was built in a real red, green, refactor sequence
- the core logic lives in its own thoroughly tested repo
- the chosen adapter is thin and does not redefine the project rules
- the coverage goals are met
