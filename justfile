set shell := ["bash", "-eu", "-c"]

default:
    @just --list

clean output_root="tutorials":
    rm -rf "{{output_root}}"

generate output_root="tutorials":
    cargo run --quiet --manifest-path scripts/tutorial_generator/Cargo.toml -- --output-root "{{output_root}}"

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

bootstrap-output-repos repos_root="../output-repos" owner="intrepion" sync_branch_name="" project="saying-hello":
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

    normalized_sync_branch_name='{{sync_branch_name}}'
    normalized_sync_branch_name="${normalized_sync_branch_name#\"}"
    normalized_sync_branch_name="${normalized_sync_branch_name%\"}"
    normalized_sync_branch_name="${normalized_sync_branch_name#sync_branch_name=}"

    normalized_project='{{project}}'
    normalized_project="${normalized_project#\"}"
    normalized_project="${normalized_project%\"}"
    normalized_project="${normalized_project#project=}"

    if [ -z "$normalized_sync_branch_name" ] && command -v python3 >/dev/null 2>&1; then
      normalized_sync_branch_name="$(
        python3 -c 'from datetime import UTC, datetime; print(datetime.now(UTC).strftime("%Y%m%dT%H%M%S%fZ"))'
      )"
    fi

    cargo run --quiet --manifest-path scripts/tutorial_generator/Cargo.toml -- \
      --bootstrap-output-repos \
      --repos-root "$normalized_repos_root" \
      --owner "$normalized_owner" \
      --sync-branch-name "$normalized_sync_branch_name" \
      --project "$normalized_project"

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
