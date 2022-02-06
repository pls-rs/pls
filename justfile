set dotenv-load := false

# Show all available recipes
default:
    @just --list --unsorted

# Install dependencies
install:
    poetry install

# Setup pre-commit as a Git hook
precommit:
    poetry run pre-commit install

# Run pre-commit to lint and reformat all files
lint:
    poetry run pre-commit run --all-files

# Run unit tests using pytest
test:
    poetry run pytest

# Open an IPython shell
shell:
    poetry run ipython
