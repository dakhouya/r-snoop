FROM mcr.microsoft.com/devcontainers/rust:2-1-trixie

RUN sudo apt update && sudo apt install -y --no-install-recommends \
    libpcap-dev \
    && rm -rf /var/lib/apt/lists/*
