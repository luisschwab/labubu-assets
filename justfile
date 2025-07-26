_default:
    @just --list

build:
    cargo build

test:
    cargo test

serve:
    dx serve
