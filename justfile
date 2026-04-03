# ABOUTME: Task runner for cocoa-way Rust macOS Wayland compositor.
# ABOUTME: Single source of truth for local dev and CI build/test/lint/security recipes.

# Run the compositor
run:
    cargo run

# Build the project
build:
    cargo build

# Run all tests
test:
    cargo test

# Run clippy linter, deny all warnings
lint:
    cargo clippy -- -D warnings

# Check formatting with rustfmt
fmt:
    cargo fmt --check

# Run cargo-audit for known vulnerability scanning
audit:
    cargo audit

# Run cargo-deny for license, advisory, ban, and source checks
deny:
    cargo deny check

# Remove build artifacts
clean:
    cargo clean

# Install git hooks via pre-commit (run once after cloning)
install-hooks:
    pre-commit install --hook-type commit-msg --hook-type pre-commit

# Meta-recipe: run all CI checks in sequence
ci: fmt lint build test audit deny
