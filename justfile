alias r:=run
alias b:=build

default: check

check:
    cargo check --all --all-features

run: build
    ./target/release/mail-rs
    
build:
    cargo build --release

fmt:
    find . -name "*.rs" -exec rustfmt {} \;
