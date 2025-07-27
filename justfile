alias b := build
alias c := check
alias t := test
alias s := serve

_default:
    @just --list

build:
    cargo build

check:
    cargo check

test:
    cargo test -- --nocapture

serve:
    dx serve
