set dotenv-load := false

# Show all available recipes
# Show all available recipes, also recurses inside nested justfiles.
@_default:
    just --list --unsorted
    printf "\nExamples:\n"
    printf   "=========\n"
    just examples/
    printf "\nDocs:\n"
    printf   "=====\n"
    just docs/

#########
# Setup #
#########

# Install dependencies for sub-projects.
install:
    # Cargo does not need an install step.
    just docs/install
    just examples/install

########
# Lint #
########

# Install `prek`, if it does not already exist.
get-prek:
    [ -x "$(command -v prek)" ] || cargo install prek
    prek install

# Run `prek` to lint and format files.
lint hook="" *files="":
    prek run {{ hook }} {{ if files == "" { "--all-files" } else { "--files" } }} {{ files }}

###########
# Recipes #
###########

# Run the program.
run *args:
    cargo run -- {{ args }}

# Run the program with debug logging.
debug *args:
    env RUST_LOG=debug just run {{ args }}

# Run tests.
test *args:
    cargo test {{ args }}

###########
# Release #
###########

# Build the release binary.
release:
    cargo build --release

# Install `cross`, if it does not already exist.
get-cross:
    [ -x "$(command -v cross)" ] || cargo install cross

# Build a release binary for the given targets with `cross`.
cross targets:
    #!/usr/bin/env bash
    set -euo pipefail
    IFS=',' read -ra items <<< "{{ targets }}"
    for target in "${items[@]}"; do
        cross build --release --verbose --target "$target"
    done

# Combine the given binaries into a universal binary.
lipo output inputs:
    #!/usr/bin/env bash
    set -euo pipefail
    IFS=',' read -ra items <<< "{{ inputs }}"
    paths=()
    for target in "${items[@]}"; do
        paths+=("target/$target/release/pls")
    done
    out_path="target/{{ output }}/release/pls"
    mkdir -p "$(dirname "$out_path")"
    lipo -create -output "$out_path" "${paths[@]}"

###########
# Aliases #
###########

alias i := install

alias l := lint

alias r := run
alias d := debug
alias t := test

alias R := release
