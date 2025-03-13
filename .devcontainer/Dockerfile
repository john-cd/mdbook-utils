# syntax=docker/dockerfile:1

# Dockerfile reference guide at
# https://docs.docker.com/go/dockerfile-reference/
# See also https://docs.docker.com/language/rust/

ARG RUST_IMAGE_LABEL
FROM rust:${RUST_IMAGE_LABEL} as base
## Repo for the Rust images: https://github.com/rust-lang/docker-rust/tree/master
## The full Rust image depends on https://github.com/docker-library/buildpack-deps

SHELL ["bash", "-c"]

# Set non-interactive mode for APT
ENV DEBIAN_FRONTEND=noninteractive

## Install utilities
RUN apt-get update \
    && apt-get install -y --no-install-recommends jq \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*
## To cache:
## --mount=type=cache,target=/var/cache/apt,sharing=locked \
## --mount=type=cache,target=/var/lib/apt,sharing=locked \

## Update Rust if needed
## Install nightly fmt
RUN rustup update \
    && rustup toolchain install nightly \
    && rustup component add rustfmt --toolchain nightly

ARG MDBOOK_VERSION

## Install cargo binstall (binary install)
## Install just, mdbook, etc... from the binary release
RUN curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash \
    && cargo binstall --no-confirm just \
    && cargo binstall --no-confirm --version ${MDBOOK_VERSION} mdbook
#    && cargo binstall --no-confirm --secure cargo-nextest

WORKDIR /code

## --------------------------------
## Stage for development
FROM base as development

## Optional packages for development only
# RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
# && apt-get install -y firefox-esr
#                       locales \
#                       xdg-utils \
#                       desktop-file-utils \
#                       nano

## Optional cargo installs for development only
# RUN cargo binstall --no-confirm ripgrep
# RUN cargo install --locked cargo-deny || true
# RUN cargo install --locked cargo-outdated || true
# RUN cargo install --locked cargo-udeps || true
# RUN cargo install --locked cargo-audit || true
# RUN cargo install --locked cargo-pants || true
RUN cargo install cargo-bump

## rustup completions
RUN mkdir -p ~/.local/share/bash-completion/completions/ \
    && rustup completions bash > ~/.local/share/bash-completion/completions/rustup

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
## Stage for Continuous Integration / Deployment
FROM base as ci

## Copy the code in the container (in WORKDIR), excluding what is in .dockerignore
COPY . .

ENV CARGO_TARGET_DIR=./target/

## Make sure the start scripts have permission to execute
RUN chmod +x ./.devcontainer/ci.sh ./.devcontainer/publish.sh

## `ci.sh` builds / tests the code and builds / tests the user guide.
## The user guide HTML that has been built is exported
## via the bind mount in `compose-ci.yaml` back to the CI host,
## so that it could be deployed to GitHub Pages.
ENTRYPOINT [ "./.devcontainer/ci.sh" ]
