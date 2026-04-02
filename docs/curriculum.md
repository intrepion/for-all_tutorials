<!-- breadcrumbs:start -->
[for-all_tutorials](../README.md) / [Docs](README.md) / curriculum.md
<!-- breadcrumbs:end -->

# Curriculum Map

Mutable learning order for project tutorials, kept separate from the filesystem so projects can move without being renamed.

## How To Use This Map

This file answers:

- what should be learned first
- what each project depends on
- which ecosystems, languages, framework guides, testing options, surfaces, and targets are currently documented
- which tutorial path should usually be built first
- what state each project is in

This file should stay incremental:

- add projects when there is real intent to build them
- reorder stages when the curriculum becomes clearer
- avoid turning this into a giant speculative backlog

## Option Catalogs

Keep shared option lists here so project entries can stay small.

### Ecosystems

- `beam`
- `dotnet`
- `go`
- `java`
- `javascript`
- `php`
- `python`
- `ruby`
- `rust`

### Languages By Ecosystem

#### `beam`

- `Elixir`
- `Erlang`

#### `dotnet`

- `C#`
- `F#`
- `Visual Basic`

#### `go`

- `Go`

#### `java`

- `Java`

#### `javascript`

- `JavaScript`
- `TypeScript`

#### `php`

- `PHP`

#### `python`

- `Python`

#### `ruby`

- `Ruby`

#### `rust`

- `Rust`

### Framework Options By Ecosystem And Surface Or Target

#### `beam`

- `command-line`: `builtin`
- `web/full-stack`: `phoenix-liveview`

#### `dotnet`

- `command-line`: `builtin`, `spectre-console`
- `web/full-stack`: `blazor-server`

#### `go`

- `command-line`: `builtin`
- `web/full-stack`: `echo-templates-htmx`

#### `java`

- `command-line`: `builtin`
- `web/full-stack`: `quarkus-native-qute`

#### `javascript`

- `command-line`: `builtin`, `commander`
- `web/full-stack`: `sveltekit-bun-adapter`

#### `php`

- `command-line`: `builtin`, `symfony-console`
- `web/full-stack`: `laravel-octane-livewire`

#### `python`

- `command-line`: `builtin`, `click`, `typer`
- `web/full-stack`: `fastapi-jinja2-htmx`

#### `ruby`

- `command-line`: `builtin`
- `web/full-stack`: `rails-hotwire`

#### `rust`

- `command-line`: `builtin`, `clap`, `ratatui`
- `web/full-stack`: `leptos-ssr`

### Testing Options By Ecosystem

#### `beam`

- no testing-specific guide has been written yet

#### `dotnet`

- `xunit`
- `nunit`
- `mstest`
- `tunit`

#### `go`

- no testing-specific guide has been written yet

#### `java`

- no testing-specific guide has been written yet

#### `javascript`

- no testing-specific guide has been written yet

#### `php`

- no testing-specific guide has been written yet

#### `python`

- no testing-specific guide has been written yet

#### `ruby`

- no testing-specific guide has been written yet

#### `rust`

- `libtest`

### Surface Options By Ecosystem

#### `beam`

- `command-line`
- `web`

#### `dotnet`

- `command-line`
- `web`

#### `go`

- `command-line`
- `web`

#### `java`

- `command-line`
- `web`

#### `javascript`

- `command-line`
- `web`

#### `php`

- `command-line`
- `web`

#### `python`

- `command-line`
- `web`

#### `ruby`

- `command-line`
- `web`

#### `rust`

- `command-line`
- `web`

### Target Options By Surface

- `command-line`: `all`
- `web`: `back-end`, `front-end`, `full-stack`

## Entry Shape

Each project entry should capture:

- `Project`
- `Status`
- `Prerequisites`
- `Recommended Tutorial`
- `Suggested Output Repos`
- `Notes`

Project slugs stay stable even if their place in this map changes.

## Stage 0

### saying-hello

- `Project`: `saying-hello`
- `Status`: `in-progress`
- `Prerequisites`: none
- `Recommended Tutorial`: [projects/saying-hello/tutorial/README.md](../projects/saying-hello/tutorial/README.md)
- `Suggested Output Repos`: one core library repo for the chosen ecosystem, language, and test framework, then one adapter repo for the chosen surface implementation
- `Notes`: start with a tiny core logic contract such as `greet(name) -> string`, follow the project tutorial starting with the core walkthrough, build the core repo first, then add adapter repos that consume that core; choose the exact surface and target later from the shared option catalogs and setup docs

## Stage 1

### count-characters

- `Project`: `count-characters`
- `Status`: `planned`
- `Prerequisites`: `saying-hello`
- `Recommended Tutorial`: [projects/count-characters/tutorial/README.md](../projects/count-characters/tutorial/README.md)
- `Suggested Output Repos`: one core library repo for the chosen ecosystem, language, and test framework, then one adapter repo for the chosen surface implementation
- `Notes`: start with small core logic contracts such as `count_characters(input) -> integer` and `format_character_count_message(input, count) -> string`, build the core repo first, then add adapter repos that consume that core; this project builds on `saying-hello` by introducing counting, exact input preservation, and zero/one/many message formatting

## Stage 2

### printing-quotes

- `Project`: `printing-quotes`
- `Status`: `planned`
- `Prerequisites`: `count-characters`
- `Recommended Tutorial`: [projects/printing-quotes/tutorial/README.md](../projects/printing-quotes/tutorial/README.md)
- `Suggested Output Repos`: one core library repo for the chosen ecosystem, language, and test framework, then one adapter repo for the chosen surface implementation
- `Notes`: start with a small formatting contract such as `format_attributed_quote(author, quote) -> string`, build the core repo first, then add adapter repos that consume that core; this project builds on the earlier string exercises by combining two inputs into one formatted sentence and surrounding the quote text with literal double quotation marks

## Stage 3

### mad-lib

- `Project`: `mad-lib`
- `Status`: `planned`
- `Prerequisites`: `printing-quotes`
- `Recommended Tutorial`: [projects/mad-lib/tutorial/README.md](../projects/mad-lib/tutorial/README.md)
- `Suggested Output Repos`: one core library repo for the chosen ecosystem, language, and test framework, then one adapter repo for the chosen surface implementation
- `Notes`: start with a small formatting contract such as `format_mad_lib(noun, verb, adjective, adverb) -> string`, build the core repo first, then add adapter repos that consume that core; this project builds on the earlier formatting exercises by combining four named inputs into one fixed story template

## Stage 4

### simple-math

- `Project`: `simple-math`
- `Status`: `planned`
- `Prerequisites`: `count-characters`
- `Recommended Tutorial`: [projects/simple-math/tutorial/README.md](../projects/simple-math/tutorial/README.md)
- `Suggested Output Repos`: one core library repo for the chosen ecosystem, language, and test framework, then one adapter repo for the chosen surface implementation
- `Notes`: start with small arithmetic and formatting contracts such as `calculate_simple_math(first, second)` and `format_simple_math_report(first, second, results)`, build the core repo first, then add adapter repos that consume that core; this project builds on earlier input-to-output exercises by introducing numeric parsing, arithmetic, and multi-line report formatting

## Stage 5

### retirement-calculator

- `Project`: `retirement-calculator`
- `Status`: `planned`
- `Prerequisites`: `simple-math`
- `Recommended Tutorial`: [projects/retirement-calculator/tutorial/README.md](../projects/retirement-calculator/tutorial/README.md)
- `Suggested Output Repos`: one core library repo for the chosen ecosystem, language, and test framework, then one adapter repo for the chosen surface implementation
- `Notes`: start with small calculation and formatting contracts such as `calculate_retirement(current_age, desired_retirement_age, current_year)` and `format_retirement_report(current_year, years_left, retirement_year)`, build the core repo first, then add adapter repos that consume that core; this project builds on earlier numeric exercises by combining prompted ages with the current calendar year from the runtime

## Stage 6

### area-of-a-rectangular-room

- `Project`: `area-of-a-rectangular-room`
- `Status`: `planned`
- `Prerequisites`: `simple-math`
- `Recommended Tutorial`: [projects/area-of-a-rectangular-room/tutorial/README.md](../projects/area-of-a-rectangular-room/tutorial/README.md)
- `Suggested Output Repos`: one core library repo for the chosen ecosystem, language, and test framework, then one adapter repo for the chosen surface implementation
- `Notes`: start with small calculation and formatting contracts such as `calculate_room_area(length_feet, width_feet)` and `format_room_area_report(length_feet, width_feet, area)`, build the core repo first, then add adapter repos that consume that core; this project builds on earlier numeric exercises by introducing unit conversion and fixed-precision metric output

## Stage 7

### pizza-party

- `Project`: `pizza-party`
- `Status`: `planned`
- `Prerequisites`: `simple-math`
- `Recommended Tutorial`: [projects/pizza-party/tutorial/README.md](../projects/pizza-party/tutorial/README.md)
- `Suggested Output Repos`: one core library repo for the chosen ecosystem, language, and test framework, then one adapter repo for the chosen surface implementation
- `Notes`: start with small distribution and formatting contracts such as `calculate_pizza_distribution(people, pizzas, slices_per_pizza)` and `format_pizza_distribution_report(people, pizzas, distribution)`, build the core repo first, then add adapter repos that consume that core; this project builds on earlier numeric exercises by introducing integer division, remainders, and leftover handling

## Stage 8

### paint-calculator

- `Project`: `paint-calculator`
- `Status`: `planned`
- `Prerequisites`: `area-of-a-rectangular-room`
- `Recommended Tutorial`: [projects/paint-calculator/tutorial/README.md](../projects/paint-calculator/tutorial/README.md)
- `Suggested Output Repos`: one core library repo for the chosen ecosystem, language, and test framework, then one adapter repo for the chosen surface implementation
- `Notes`: start with small calculation and formatting contracts such as `calculate_paint_requirements(length_feet, width_feet)` and `format_paint_purchase_message(square_feet, gallons_needed)`, build the core repo first, then add adapter repos that consume that core; this project builds on room-area calculation by introducing ceiling rounding with a fixed `350` square-feet-per-gallon coverage rule

## Stage 9

### self-checkout

- `Project`: `self-checkout`
- `Status`: `planned`
- `Prerequisites`: `pizza-party`
- `Recommended Tutorial`: [projects/self-checkout/tutorial/README.md](../projects/self-checkout/tutorial/README.md)
- `Suggested Output Repos`: one core library repo for the chosen ecosystem, language, and test framework, then one adapter repo for the chosen surface implementation
- `Notes`: start with small calculation and formatting contracts such as `calculate_checkout_summary(items)` and `format_checkout_report(summary)`, build the core repo first, then add adapter repos that consume that core; this project builds on earlier quantity-based numeric exercises by introducing exact cent-based money handling, percentage tax calculation, and fixed-precision currency output

## Stage 10

### currency-conversion

- `Project`: `currency-conversion`
- `Status`: `planned`
- `Prerequisites`: `self-checkout`
- `Recommended Tutorial`: [projects/currency-conversion/tutorial/README.md](../projects/currency-conversion/tutorial/README.md)
- `Suggested Output Repos`: one core library repo for the chosen ecosystem, language, and test framework, then one adapter repo for the chosen surface implementation
- `Notes`: start with small calculation and formatting contracts such as `calculate_currency_conversion(amount_euros, exchange_rate_hundredths)` and `format_currency_conversion_report(amount_euros, exchange_rate_hundredths, usd_cents)`, build the core repo first, then add adapter repos that consume that core; this project builds on earlier money exercises by introducing scaled exchange-rate input, exact cent-based conversion, and deterministic rounding to U.S. dollar cents

## Stage 11

### computing-simple-interest

- `Project`: `computing-simple-interest`
- `Status`: `planned`
- `Prerequisites`: `currency-conversion`
- `Recommended Tutorial`: [projects/computing-simple-interest/tutorial/README.md](../projects/computing-simple-interest/tutorial/README.md)
- `Suggested Output Repos`: one core library repo for the chosen ecosystem, language, and test framework, then one adapter repo for the chosen surface implementation
- `Notes`: start with small calculation and formatting contracts such as `calculate_simple_interest(principal_dollars, annual_rate_tenths_percent, years)` and `format_simple_interest_report(years, annual_rate_tenths_percent, accrued_amount_cents)`, build the core repo first, then add adapter repos that consume that core; this project builds on earlier money exercises by making rate scaling and arithmetic order explicit in a deterministic simple-interest calculation

## Stage 12

### determining-compound-interest

- `Project`: `determining-compound-interest`
- `Status`: `planned`
- `Prerequisites`: `computing-simple-interest`
- `Recommended Tutorial`: [projects/determining-compound-interest/tutorial/README.md](../projects/determining-compound-interest/tutorial/README.md)
- `Suggested Output Repos`: one core library repo for the chosen ecosystem, language, and test framework, then one adapter repo for the chosen surface implementation
- `Notes`: start with small calculation and formatting contracts such as `calculate_compound_interest(principal_dollars, annual_rate_tenths_percent, years, compounds_per_year)` and `format_compound_interest_report(principal_dollars, annual_rate_tenths_percent, years, compounds_per_year, accrued_amount_cents)`, build the core repo first, then add adapter repos that consume that core; this project builds directly on simple interest by adding exponent-based growth and compound periods while keeping deterministic cent-rounded output
