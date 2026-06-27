set dotenv-load := false

# @ toggles quieting at the recipe-level. Without quiet, each commands is
#   printed to STDERR before execution.
# _ marks a recipe as private and stops it from appearing in `just --list` or
#   `just --summary`.

# Show all available recipes, also recurses inside nested justfiles.
@_default:
	just --list --unsorted
	just _section "Examples:"
	just examples/
	just _section "Docs:"
	just docs/

# Print the given text with an equal number of `=` characters below it.
@_section text:
	printf "\n{{ text }}\n"
	printf "%0.s=" $(seq 1 $(printf "%s" "{{ text }}" | wc -c))
	printf "\n"

#########
# Setup #
#########

# Install dependencies for sub-projects.
install:
	# Cargo does not need an install step.
	cargo bin --install
	just docs/install
	just examples/install

########
# Lint #
########

# Run `prek` to lint and format files.
lint hook="" *files="":
	cargo bin prek run {{ hook }} {{ if files == "" { "--all-files" } else { "--files" } }} {{ files }}

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
	cargo nextest run {{ args }}

###########
# Release #
###########

# Build the release binary.
release:
	cargo build --release

# Build a release binary for the given targets with `cross`.
cross targets:
	#!/usr/bin/env bash
	set -euo pipefail
	IFS=',' read -ra items <<< "{{ targets }}"
	for target in "${items[@]}"; do
		cargo bin cross build --release --verbose --target "$target"
	done

# Build a release binary for the given targets with `cargo` (no `cross`).
build-targets targets:
	#!/usr/bin/env bash
	set -euo pipefail
	IFS=',' read -ra items <<< "{{ targets }}"
	for target in "${items[@]}"; do
		cargo build --release --target "$target"
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
