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
test *args:
    poetry run pytest {{ args }}

mypy *args="--all-files":
    poetry run pre-commit run mypy {{ args }}

# Open an IPython shell
shell:
    poetry run ipython

# Print the current version of `pls` from the `pyproject.toml` file
ver:
    grep 'version' pyproject.toml | cut -d'"' -f 2

# Bump the version number of the project, commit the code and tag the commit
bump level="minor":
    poetry version {{ level }}
    git add pyproject.toml
    git commit -m "Bump version to $(just ver)"
    git tag -a "$(just ver)" -m "$(just ver)"

# Build the project and publish to PyPI
deploy:
    poetry build
    poetry publish
