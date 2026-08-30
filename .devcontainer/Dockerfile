# Devcontainer base image is Rust on Debian Bookworm
ARG BASE_IMAGE=mcr.microsoft.com/devcontainers/rust:2-1-trixie
FROM ${BASE_IMAGE}

RUN apt-get update \
    && apt-get install -y --no-install-recommends \
       # Common tools:
       git \
       curl \
       sudo \
       bash \
       ca-certificates \
       wget \
       file \
       build-essential \
       # Wayland
       libwayland-dev \
       libxkbcommon-dev \
       libegl1-mesa-dev \
       libgl1-mesa-dev \
       libx11-dev \
       libxcursor-dev \
       libxi-dev \
       libxrandr-dev \
       libxinerama-dev \
       # File dialog:
       zenity libgtk-3-0 \
       # Faster Linker needs this:
       clang \
    && rm -rf /var/lib/apt/lists/*

USER vscode

# Install cargo-binstall, see https://crates.io/crates/cargo-binstall
RUN curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash

# Use cargo-binstall to install tools:
RUN cargo binstall --no-confirm --locked \
  wild-linker \
  trunk \
  wasm-bindgen-cli

# Setup Wild linker, see https://github.com/wild-linker/wild and https://github.com/wild-linker/action/blob/dc926c45399e1842cf81e604f52bdf0ad86a0c09/action.yml
RUN CARGO_DIR="${CARGO_HOME:-$HOME/.cargo}" \
  && mkdir -p "$CARGO_DIR" \
  && echo "\n[target.x86_64-unknown-linux-gnu]\nlinker = \"clang\"\nrustflags = [\"-Clink-arg=--ld-path=wild\"]\n" >>"$CARGO_DIR/config.toml" \
  && echo "\n[target.x86_64-unknown-linux-musl]\nlinker = \"clang\"\nrustflags = [\"-Clink-arg=--ld-path=wild\"]\n" >>"$CARGO_DIR/config.toml" \
  && echo "\n[target.aarch64-unknown-linux-gnu]\nlinker = \"clang\"\nrustflags = [\"-Clink-arg=--ld-path=wild\"]\n" >>"$CARGO_DIR/config.toml" \
  && echo "\n[target.aarch64-unknown-linux-musl]\nlinker = \"clang\"\nrustflags = [\"-Clink-arg=--ld-path=wild\"]\n" >>"$CARGO_DIR/config.toml"

# Install wasm target for rust
RUN rustup target add wasm32-unknown-unknown
