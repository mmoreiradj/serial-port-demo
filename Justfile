[private]
default:
  @just --list --unsorted

musl-compile BINARY = "agent":
  #!/usr/bin/env bash
  docker run --rm \
    -v cargo-cache:/root/.cargo \
    -v $PWD:/volume \
    -w /volume \
    -t clux/muslrust \
    cargo build --release --bin {{BINARY}}

create-serial-device:
  socat -d -d pty,raw,echo=0 pty,raw,echo=0

compile-agent:
  @just musl-compile agent
  cp target/x86_64-unknown-linux-musl/release/agent .

compile-host:
  @just musl-compile host
  cp target/x86_64-unknown-linux-musl/release/host .

compile-all:
  @just compile-agent
  @just compile-host
