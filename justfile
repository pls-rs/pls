set dotenv-load := false

# Show all available recipes
default:
    @just --list --unsorted

###############
# Development #
###############

# Install dependencies and set up pre-commit Git hook
install:
    poetry install
    poetry run pre-commit install

# Run pre-commit to lint and reformat all files
lint:
    poetry run pre-commit run --all-files

# Run mypy under pre-commit to typecheck code
mypy *args="--all-files":
    poetry run pre-commit run mypy {{ args }}

# Run Black under pre-commit to reformat code
black *args="--all-files":
    poetry run pre-commit run black {{ args }}

# Run unit tests using pytest
test *args:
    poetry run pytest --cov=src/ {{ args }}

# Open an IPython shell
shell:
    poetry run ipython

# Remove all Python cache files
pyclean:
    find . -type f -name '*.py[co]' -delete -o -type d -name __pycache__ -delete

##############
# Deployment #
##############

# Print the current version of `pls` from the `pyproject.toml` file
ver:
    grep 'version' pyproject.toml | cut -d'"' -f 2

# Bump the version number of the project, commit the code and tag the commit
bump level="minor":
    poetry version {{ level }}
    git add pyproject.toml
    git commit -m "Bump version to $(just ver)"
    git tag -a "$(just ver)" -m "$(just ver)"
    git push origin main
    git push origin $(just ver)

# Build the project and publish to PyPI
deploy:
    poetry build
    poetry publish
