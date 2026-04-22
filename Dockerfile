# Builder Stage
FROM rust:1-slim-bookworm AS builder
WORKDIR /app

# Install system dependencies
RUN apt-get update && apt-get install -y \
    curl \
    pkg-config \
    libssl-dev \
    git \
    && rm -rf /var/lib/apt/lists/*

# Install cargo-binstall to drastically speed up CLI installations
RUN curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash

# Install Trunk and cargo-leptos via binstall (pre-compiled binaries) instead of building from source
RUN cargo binstall -y trunk cargo-leptos
RUN rustup target add wasm32-unknown-unknown

# Copy source code and build
COPY . .
# Note: Using data-wasm-opt="0" in index.html to bypass bulk-memory errors
RUN trunk build --release

# Runtime Stage
FROM nginxinc/nginx-unprivileged:alpine
COPY --from=builder /app/dist /usr/share/nginx/html

# Nginx config for SPA with security hardening
# Unprivileged Nginx uses port 8080 and drops root access
USER root
RUN mkdir -p /etc/nginx/templates && echo 'server { \
    listen ${PORT}; \
    server_tokens off; \
    add_header X-Frame-Options "SAMEORIGIN" always; \
    add_header X-Content-Type-Options "nosniff" always; \
    add_header X-XSS-Protection "1; mode=block" always; \
    add_header Referrer-Policy "strict-origin-when-cross-origin" always; \
    location / { \
        root /usr/share/nginx/html; \
        index index.html; \
        try_files $uri $uri/ /index.html; \
    } \
}' > /etc/nginx/templates/default.conf.template && \
    chown -R 101:101 /usr/share/nginx/html

ENV PORT=8080

USER 101

EXPOSE 8080
CMD ["nginx", "-g", "daemon off;"]
