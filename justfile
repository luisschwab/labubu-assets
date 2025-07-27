alias b := build
alias c := check
alias f := fmt
alias t := test
alias s := serve

_default:
    @just --list

build:
    rm -rf docs
    dx bundle --platform web --release --out-dir docs
    mv docs/public/* docs
    rmdir docs/public

check:
    cargo check

fmt:
    cargo +nightly fmt
    dx fmt

test:
    cargo test -- --nocapture

serve:
    dx serve
