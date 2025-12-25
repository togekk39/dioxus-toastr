FROM archlinux:base-devel

# Install necessary tools
RUN pacman -Syu --noconfirm \
    curl \
    git \
    pkg-config \
    openssl \
    openssh \
    rustup \
    base-devel \
    procps-ng \
    mold \
    sccache

RUN pacman -Scc --noconfirm

# Add environment variables (persistent for all RUN and CMD steps)
ENV RUSTC_WRAPPER="sccache"
ENV SCCACHE_DIR="~/.cache/sccache"
ENV CHROME_BINARY="/usr/bin/chromium"
ENV PATH="/root/.cargo/bin:${PATH}"
# ENV RUST_LOG="error"

# Setup Rust
RUN rustup default nightly && \
    rustup target add wasm32-unknown-unknown && \
    cargo install wasm-pack && \
    cargo install dioxus-cli

# Set working directory (will be overridden in docker-compose)
WORKDIR /app

# Copy minimal files (actual source code will be volume mounted)
COPY Cargo.toml Cargo.lock ./
COPY src ./src

# Pre-fetch dependencies to layer cache
RUN cargo fetch && cargo update

CMD ["bash"]
