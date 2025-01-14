FROM mcr.microsoft.com/devcontainers/base:debian-12

ENV DEBIAN_FRONTEND=noninteractive

SHELL ["/bin/bash", "-c"]

RUN apt update && \
    apt install -y curl && \
    echo 'deb http://download.opensuse.org/repositories/shells:/fish:/release:/3/Debian_12/ /' | tee /etc/apt/sources.list.d/shells:fish:release:3.list && \
    curl -fsSL https://download.opensuse.org/repositories/shells:fish:release:3/Debian_12/Release.key | gpg --dearmor | tee /etc/apt/trusted.gpg.d/shells_fish_release_3.gpg > /dev/null && \
    apt update && \
    apt install -y \
        git \
        xdg-utils \
        fish && \
    apt clean && \
    rm -rf /var/lib/apt/lists/*

USER vscode
ENV HOME="/home/vscode"
ENV FISH_CONFIG_DIR="${HOME}/.config/fish"
RUN mkdir -p "${FISH_CONFIG_DIR}"

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="${HOME}/.cargo/bin:${PATH}"
ENV RUST_VERSION="1.83.0"
RUN rustup install "${RUST_VERSION}" && \
    rustup default "${RUST_VERSION}"

ENV FISH_CONFIG_FILE="${FISH_CONFIG_DIR}/config.fish"
RUN echo "alias ll='ls -alF'" >> "${FISH_CONFIG_FILE}"
RUN echo "functions --erase fish_greeting" >> "${FISH_CONFIG_FILE}"

WORKDIR /work
