# Utility Scripts

This directory contains utility scripts for development, testing, and deployment.

## Available Scripts

### setup.sh

Sets up the development environment.

```bash
./scripts/setup.sh
```

**What it does:**
- Checks for Rust installation
- Installs Diesel CLI if not present
- Creates .env file from .env.example
- Runs database migrations
- Builds the project

### test.sh

Runs all tests and quality checks.

```bash
./scripts/test.sh
```

**What it does:**
- Runs unit and integration tests
- Generates code coverage report (if tarpaulin is installed)
- Runs clippy linter
- Checks code formatting

### docker-build.sh

Builds the Docker image.

```bash
./scripts/docker-build.sh [tag]
```

**Arguments:**
- `tag` (optional): Docker image tag (default: latest)

**Example:**
```bash
./scripts/docker-build.sh v1.0.0
```

### clean.sh

Cleans up build artifacts and temporary files.

```bash
./scripts/clean.sh
```

**What it does:**
- Removes cargo build artifacts
- Optionally removes database file
- Removes SQLite journal files
- Removes log files
- Removes coverage reports

## Making Scripts Executable

Before running these scripts, make them executable:

```bash
chmod +x scripts/*.sh
```

## Requirements

- Bash shell
- Rust and Cargo
- Docker (for docker-build.sh)

## Notes

- All scripts are designed to be run from the project root directory
- Scripts will exit immediately if any command fails (set -e)
- The setup.sh script is particularly useful for new developers
