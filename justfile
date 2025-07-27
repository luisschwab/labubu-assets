alias b := build
alias t := test
alias s := serve

_default:
    @just --list

build:
    cargo build

test:
    cargo test -- --nocapture

serve:
    dx serve
