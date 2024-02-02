# syntax=docker/dockerfile:1

# Dockerfile reference guide at
# https://docs.docker.com/go/dockerfile-reference/
# See also https://docs.docker.com/language/rust/

ARG RUST_IMAGE_LABEL
FROM rust:${RUST_IMAGE_LABEL} as base
## Repo for the Rust images: https://github.com/rust-lang/docker-rust/tree/master
## The full Rust image depends on https://github.com/docker-library/buildpack-deps

SHELL ["bash", "-c"]

## Install linker and other utilities
# RUN --mount=type=cache,target=/var/cache/apt,sharing=locked \
#     --mount=type=cache,target=/var/lib/apt,sharing=locked \
#     apt-get update \
#     && export DEBIAN_FRONTEND=noninteractive \
#     && apt-get install -y --no-install-recommends mold clang \
#     && apt-get autoremove -y \
#     && apt-get clean -y \
#     && rm -rf /var/lib/apt/lists/*
## Alternatively, use `lld` instead of the `mold` linker
## Configuration in .cargo/config.toml
## If using the `slim` Rust image, you may need to install: curl pkg-config libssl-dev

## Update Rust if needed (note: `rustup update stable && rustup default stable` is required for Rust 1.74 -> Rust 1.75 somehow)
RUN rustup update

## Install clippy if needed
RUN rustup component add clippy

## Install nightly fmt
RUN rustup toolchain install nightly \
    && rustup component add rustfmt --toolchain nightly

## Install cargo binstall (binary install)
## Install just and nextest from the binary release
RUN curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash \
    && cargo binstall --no-confirm just \
    && cargo binstall --no-confirm --secure cargo-nextest

## TODO Install cargo check tools
# RUN cargo install --locked cargo-deny || true
# RUN cargo install --locked cargo-outdated || true
# RUN cargo install --locked cargo-udeps || true
# RUN cargo install --locked cargo-audit || true
# RUN cargo install --locked cargo-pants || true

WORKDIR /code

## --------------------------------
## Stage for development
FROM base as development

## Additional packages for developement only
# RUN apt-get update && export DEBIAN_FRONTEND=noninteractive && apt-get install -y firefox-esr
# locales xdg-utils desktop-file-utils \
# nano \

# RUN cargo binstall --no-confirm ripgrep

## https://code.visualstudio.com/remote/advancedcontainers/persist-bash-history
RUN SNIPPET="export PROMPT_COMMAND='history -a' && export HISTFILE=/command_history/.bash_history" \
    && echo "$SNIPPET" >> "/root/.bashrc"

## Git config
ARG EMAIL
ARG GIT_AUTHOR_NAME

RUN git config --global user.email ${EMAIL} && git config --global user.name ${GIT_AUTHOR_NAME}

## https://code.visualstudio.com/remote/advancedcontainers/start-processes
COPY --chmod=755 .devcontainer/dev.sh ./.devcontainer/
ENTRYPOINT [ "./.devcontainer/dev.sh" ]

## Sleep to keep the container running.
## The command is passed to `dev.sh`.
CMD [ "sleep", "infinity" ]

## --------------------------------
## Stage for Continuous Integration
FROM base as ci

## Copy the code in the container (in WORKDIR), excluding what is in .dockerignore
COPY . .

ENV CARGO_TARGET_DIR=./target/

## Make sure the start scripts have permission to execute
RUN chmod +x ./.devcontainer/ci.sh

ENTRYPOINT [ "./.devcontainer/ci.sh" ]
