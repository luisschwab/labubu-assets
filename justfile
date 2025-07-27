alias b := build
alias c := check
alias f := fmt
alias t := test
alias s := serve

_default:
    @just --list

build:
    cargo build

check:
    cargo check

fmt:
    cargo +nightly fmt

test:
    cargo test -- --nocapture

serve:
    dx serve
