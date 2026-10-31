[default]
_default:
    @just --list

# Install pinned tools
[group('configuration')]
deps:
    mise install

# Build a debug binary
[group('build')]
build:
    cargo build --locked

# Build an optimized binary at target/release/wt
[group('build')]
release:
    cargo build --release --locked

# Run the integration tests (need git and zsh on PATH)
[group('tests')]
test:
    cargo test --locked

# Run every gate
[group('checks')]
check: test
