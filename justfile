set windows-shell := ["pwsh.exe", "-NoLogo", "-Command"]

default:
    @just --list

fmt:
    cargo sort-derives
    cargo fmt
    taplo fmt
    rumdl fmt .

clippy:
    cargo clippy --all-targets --all-features --locked -- -D warnings

check:
    cargo check --all-targets --all-features --locked

test:
    cargo test --all-features --locked

cov:
    cargo llvm-cov --all-features --all-targets

test-publish:
    cargo publish --dry-run --allow-dirty
