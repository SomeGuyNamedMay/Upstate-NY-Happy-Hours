# --- Stage 1: Build Environment ---
FROM rust:1.80-slim AS builder

# Install system dependencies needed for building packages
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    git \
    && rm -rf /var/lib/apt/lists/*

# Install Trunk and the WebAssembly build target
RUN cargo install --locked trunk
RUN rustup target add wasm32-unknown-unknown

WORKDIR /usr/src/app

# Cache dependencies by doing a dummy build
COPY Cargo.toml Cargo.lock* ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
# Create a skeleton index.html so trunk doesn't fail
RUN echo '<html lang="en"><body></body></html>' > index.html
RUN trunk build --release

# Copy the actual project files
COPY . .

# Build the production assets
RUN trunk build --release

# --- Stage 2: Serve Environment ---
FROM nginx:alpine-slim

# Copy compiled static assets from the builder stage to Nginx directory
COPY --from=builder /usr/src/app/dist /usr/share/nginx/html

# Optional: Copy a custom nginx configuration to handle client-side routing
# COPY nginx.conf /etc/nginx/conf.d/default.conf

EXPOSE 80

CMD ["nginx", "-g", "daemon off;"]
