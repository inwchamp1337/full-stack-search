# Build stage
FROM rust:1.83 AS builder

WORKDIR /app

# Install trunk and wasm target
RUN cargo install --locked trunk
RUN rustup target add wasm32-unknown-unknown

# Copy project files
COPY Cargo.toml Cargo.lock ./
COPY src ./src
COPY index.html ./
COPY Trunk.toml ./

# Build the project with trunk
RUN trunk build --release

# Runtime stage - Use nginx to serve static files
FROM nginx:alpine

# Copy built files from builder
COPY --from=builder /app/dist /usr/share/nginx/html

# Copy nginx configuration
COPY nginx.conf /etc/nginx/nginx.conf

EXPOSE 3000

CMD ["nginx", "-g", "daemon off;"]