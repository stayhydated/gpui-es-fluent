set windows-shell := ["pwsh.exe", "-NoLogo", "-Command"]

default:
    @just --list

fmt:
    cargo sort-derives
    cargo fmt
    taplo fmt
    rumdl fmt .

# Run with the versioned CI quality tools installed and Docker available.
quality:
    taplo fmt --check
    rumdl check .
    typos
    cargo sort-derives --check
    docker run --rm -v "{{justfile_directory()}}:/repo" -w /repo rhysd/actionlint:1.7.12
    git ls-files -z '*.sh' '*.bash' | xargs -0 --no-run-if-empty shellcheck

clippy:
    cargo clippy --workspace --all-targets --all-features --locked -- -D warnings

check:
    cargo check --workspace --all-targets --all-features --locked

test:
    cargo test --workspace --all-features --locked

cov:
    cargo llvm-cov --workspace --all-features --all-targets --locked \
        --exclude gpui-es-fluent-demo \
        --exclude xtask \
        --exclude web

test-publish:
    cargo publish --workspace --dry-run --locked --allow-dirty

book:
    mdbook serve book

gpui-demo-build:
    cargo xtask build gpui-demo

web-build: gpui-demo-build
    cargo xtask build book
    cargo xtask build llms-txt
    cargo xtask build web

web: web-build
    dx serve --package web

web-preview: web-build
    cargo xtask preview web
