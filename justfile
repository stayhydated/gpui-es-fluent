set windows-shell := ["pwsh.exe", "-NoLogo", "-Command"]

default:
    @just --list

fmt:
    cargo sort-derives
    cargo fmt
    taplo fmt
    rumdl fmt .

# Run with the pinned CI quality tools installed.
quality:
    taplo fmt --check
    rumdl check .
    typos
    cargo sort-derives --check
    actionlint
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
