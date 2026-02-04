FROM rust:1.93-bookworm

# Install Tauri dependencies with webkit2gtk-4.1
RUN apt-get update && apt-get install -y \
    libwebkit2gtk-4.1-dev \
    build-essential \
    curl \
    wget \
    file \
    libssl-dev \
    libgtk-3-dev \
    libayatana-appindicator3-dev \
    librsvg2-dev \
    pkg-config \
    xdg-utils \
    && rm -rf /var/lib/apt/lists/*

# Set PKG_CONFIG_PATH for the build
ENV PKG_CONFIG_PATH=/usr/lib/x86_64-linux-gnu/pkgconfig:/usr/share/pkgconfig

# Install wasm target first
RUN rustup target add wasm32-unknown-unknown

# Install Tauri CLI, trunk, and wasm-bindgen-cli
RUN cargo install tauri-cli trunk wasm-bindgen-cli && \
    cp /usr/local/cargo/bin/trunk /usr/local/bin/ && \
    cp /usr/local/cargo/bin/wasm-bindgen /usr/local/bin/ && \
    cp /usr/local/cargo/bin/cargo-tauri /usr/local/bin/

WORKDIR /app
