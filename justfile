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
