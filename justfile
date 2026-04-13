set shell := ["bash", "-eu", "-c"]

bootstrap_ecosystem := ""
bootstrap_language := ""
bootstrap_testing := ""
bootstrap_mocking := ""
bootstrap_storage := ""
bootstrap_surface := ""
bootstrap_target := ""
bootstrap_framework := ""
bootstrap_protocol := ""

default:
    @just --list

clean output_root="tutorials":
    rm -rf "{{output_root}}"

generate output_root="tutorials":
    cargo run --quiet --manifest-path scripts/tutorial_generator/Cargo.toml -- --output-root "{{output_root}}"

check-generator:
    cargo check --manifest-path scripts/tutorial_generator/Cargo.toml

test-generator:
    cargo test --manifest-path scripts/tutorial_generator/Cargo.toml

regenerate output_root="tutorials":
    just clean "{{output_root}}"
    just generate "{{output_root}}"

check output_root="tutorials":
    #!/usr/bin/env bash
    set -euo pipefail
    temp_dir="$(mktemp -d)"
    trap 'rm -rf "$temp_dir"' EXIT
    cargo run --quiet --manifest-path scripts/tutorial_generator/Cargo.toml -- --output-root "$temp_dir"
    if [ ! -d "{{output_root}}" ]; then
      echo "Generated tutorials are missing at {{output_root}}. Run 'just generate' first."
      exit 1
    fi
    diff -ru "{{output_root}}" "$temp_dir"

check-all output_root="tutorials":
    just check-generator
    just test-generator
    just check "{{output_root}}"

create-output-repos owner="intrepion":
    #!/usr/bin/env bash
    set -euo pipefail
    normalized_owner='{{owner}}'
    normalized_owner="${normalized_owner#\"}"
    normalized_owner="${normalized_owner%\"}"
    normalized_owner="${normalized_owner#owner=}"

    cargo run --quiet --manifest-path scripts/tutorial_generator/Cargo.toml -- \
      --create-output-repos \
      --owner "$normalized_owner"

clone-output-repos repos_root="../output-repos" owner="intrepion":
    #!/usr/bin/env bash
    set -euo pipefail
    normalized_repos_root='{{repos_root}}'
    normalized_repos_root="${normalized_repos_root#\"}"
    normalized_repos_root="${normalized_repos_root%\"}"
    normalized_repos_root="${normalized_repos_root#repos_root=}"

    normalized_owner='{{owner}}'
    normalized_owner="${normalized_owner#\"}"
    normalized_owner="${normalized_owner%\"}"
    normalized_owner="${normalized_owner#owner=}"

    cargo run --quiet --manifest-path scripts/tutorial_generator/Cargo.toml -- \
      --clone-output-repos \
      --repos-root "$normalized_repos_root" \
      --owner "$normalized_owner"

[private]
_bs owner="intrepion" project="saying-hello":
    #!/usr/bin/env bash
    set -euo pipefail
    normalized_owner='{{owner}}'
    normalized_owner="${normalized_owner#\"}"
    normalized_owner="${normalized_owner%\"}"
    normalized_owner="${normalized_owner#owner=}"

    normalized_project='{{project}}'
    normalized_project="${normalized_project#\"}"
    normalized_project="${normalized_project%\"}"
    normalized_project="${normalized_project#project=}"

    normalized_repos_root='../output-repos'
    normalized_sync_branch_name=''

    normalized_ecosystem='{{bootstrap_ecosystem}}'
    normalized_ecosystem="${normalized_ecosystem#\"}"
    normalized_ecosystem="${normalized_ecosystem%\"}"
    normalized_ecosystem="${normalized_ecosystem#ecosystem=}"

    normalized_language='{{bootstrap_language}}'
    normalized_language="${normalized_language#\"}"
    normalized_language="${normalized_language%\"}"
    normalized_language="${normalized_language#language=}"

    normalized_testing='{{bootstrap_testing}}'
    normalized_testing="${normalized_testing#\"}"
    normalized_testing="${normalized_testing%\"}"
    normalized_testing="${normalized_testing#testing=}"

    normalized_mocking='{{bootstrap_mocking}}'
    normalized_mocking="${normalized_mocking#\"}"
    normalized_mocking="${normalized_mocking%\"}"
    normalized_mocking="${normalized_mocking#mocking=}"

    normalized_storage='{{bootstrap_storage}}'
    normalized_storage="${normalized_storage#\"}"
    normalized_storage="${normalized_storage%\"}"
    normalized_storage="${normalized_storage#storage=}"

    normalized_surface='{{bootstrap_surface}}'
    normalized_surface="${normalized_surface#\"}"
    normalized_surface="${normalized_surface%\"}"
    normalized_surface="${normalized_surface#surface=}"

    normalized_target='{{bootstrap_target}}'
    normalized_target="${normalized_target#\"}"
    normalized_target="${normalized_target%\"}"
    normalized_target="${normalized_target#target=}"

    normalized_framework='{{bootstrap_framework}}'
    normalized_framework="${normalized_framework#\"}"
    normalized_framework="${normalized_framework%\"}"
    normalized_framework="${normalized_framework#framework=}"

    normalized_protocol='{{bootstrap_protocol}}'
    normalized_protocol="${normalized_protocol#\"}"
    normalized_protocol="${normalized_protocol%\"}"
    normalized_protocol="${normalized_protocol#protocol=}"

    if [ -z "$normalized_sync_branch_name" ] && command -v python3 >/dev/null 2>&1; then
      normalized_sync_branch_name="$(
        python3 -c 'from datetime import UTC, datetime; print(datetime.now(UTC).strftime("%Y%m%dT%H%M%S%fZ"))'
      )"
    fi

    args=(
      cargo run --quiet --manifest-path scripts/tutorial_generator/Cargo.toml --
      --bootstrap-output-repos
      --repos-root "$normalized_repos_root"
      --owner "$normalized_owner"
      --sync-branch-name "$normalized_sync_branch_name"
      --project "$normalized_project"
    )

    if [ -n "$normalized_ecosystem" ]; then
      args+=(--ecosystem "$normalized_ecosystem")
    fi
    if [ -n "$normalized_language" ]; then
      args+=(--language "$normalized_language")
    fi
    if [ -n "$normalized_testing" ]; then
      args+=(--testing "$normalized_testing")
    fi
    if [ -n "$normalized_mocking" ]; then
      args+=(--mocking "$normalized_mocking")
    fi
    if [ -n "$normalized_storage" ]; then
      args+=(--storage "$normalized_storage")
    fi
    if [ -n "$normalized_surface" ]; then
      args+=(--surface "$normalized_surface")
    fi
    if [ -n "$normalized_target" ]; then
      args+=(--target "$normalized_target")
    fi
    if [ -n "$normalized_framework" ]; then
      args+=(--framework "$normalized_framework")
    fi
    if [ -n "$normalized_protocol" ]; then
      args+=(--protocol "$normalized_protocol")
    fi

    "${args[@]}"

bs-dotnet owner="intrepion" project="saying-hello":
    just --set bootstrap_ecosystem dotnet \
      --set bootstrap_language csharp \
      --set bootstrap_testing xunit \
      --set bootstrap_mocking nsubstitute \
      --set bootstrap_storage no-storage \
      --set bootstrap_surface command-line \
      --set bootstrap_target all \
      --set bootstrap_framework no-framework \
      _bs "{{owner}}" "{{project}}"

bs-echo owner="intrepion" project="saying-hello":
    just --set bootstrap_ecosystem go \
      --set bootstrap_language go \
      --set bootstrap_testing testify \
      --set bootstrap_mocking testify-mock \
      --set bootstrap_storage no-storage \
      --set bootstrap_surface web \
      --set bootstrap_target api \
      --set bootstrap_framework echo \
      --set bootstrap_protocol http-json \
      _bs "{{owner}}" "{{project}}"

bs-astro owner="intrepion" project="saying-hello":
    just --set bootstrap_ecosystem javascript \
      --set bootstrap_language typescript \
      --set bootstrap_testing vitest \
      --set bootstrap_mocking vitest-built-in \
      --set bootstrap_storage no-storage \
      --set bootstrap_surface web \
      --set bootstrap_target front-end \
      --set bootstrap_framework astro \
      --set bootstrap_protocol http-json \
      _bs "{{owner}}" "{{project}}"

bs-flutter-local owner="intrepion" project="saying-hello":
    just --set bootstrap_ecosystem dart \
      --set bootstrap_language dart \
      --set bootstrap_testing test \
      --set bootstrap_mocking mocktail \
      --set bootstrap_storage no-storage \
      --set bootstrap_surface client \
      --set bootstrap_target all \
      --set bootstrap_framework flutter \
      --set bootstrap_protocol none \
      _bs "{{owner}}" "{{project}}"

bs-flutter-http-json owner="intrepion" project="saying-hello":
    just --set bootstrap_ecosystem dart \
      --set bootstrap_language dart \
      --set bootstrap_testing test \
      --set bootstrap_mocking mocktail \
      --set bootstrap_storage no-storage \
      --set bootstrap_surface client \
      --set bootstrap_target all \
      --set bootstrap_framework flutter \
      --set bootstrap_protocol http-json \
      _bs "{{owner}}" "{{project}}"

cleanup-output-repos apply="false" repos_root="../output-repos" owner="intrepion":
    #!/usr/bin/env bash
    set -euo pipefail
    normalized_apply='{{apply}}'
    normalized_apply="${normalized_apply#\"}"
    normalized_apply="${normalized_apply%\"}"
    normalized_apply="${normalized_apply#apply=}"

    normalized_repos_root='{{repos_root}}'
    normalized_repos_root="${normalized_repos_root#\"}"
    normalized_repos_root="${normalized_repos_root%\"}"
    normalized_repos_root="${normalized_repos_root#repos_root=}"

    normalized_owner='{{owner}}'
    normalized_owner="${normalized_owner#\"}"
    normalized_owner="${normalized_owner%\"}"
    normalized_owner="${normalized_owner#owner=}"

    args=(
      cargo run --quiet --manifest-path scripts/tutorial_generator/Cargo.toml --
      --cleanup-output-repos
      --repos-root "$normalized_repos_root"
      --owner "$normalized_owner"
    )
    if [ "$normalized_apply" = "true" ] || [ "$normalized_apply" = "1" ] || [ "$normalized_apply" = "yes" ]; then
      args+=(--apply)
    fi
    "${args[@]}"

cleanup-output-repos-apply repos_root="../output-repos" owner="intrepion":
    just cleanup-output-repos true "{{repos_root}}" "{{owner}}"
