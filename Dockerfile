# Build stage
FROM rust:alpine as builder

WORKDIR /usr/src/app

# Install dependencies
RUN apk add --no-cache pkgconfig openssl-dev musl-dev

# Copy project files
COPY Cargo.toml Cargo.lock ./
COPY src ./src
# Copy www directory (needed for bundle feature)
COPY www ./www

# This allows us to cache dependencies
RUN mkdir -p .cargo && \
    cargo vendor > .cargo/config

# Build arguments
ARG FEATURES=bundle,compression
ARG BUILD_MODE=release

# Build the application
RUN if [ "$BUILD_MODE" = "release" ] ; then \
    if [ -n "$FEATURES" ] ; then \
    cargo build --release --features $FEATURES ; \
    else \
    cargo build --release ; \
    fi \
    else \
    if [ -n "$FEATURES" ] ; then \
    cargo build --features $FEATURES ; \
    else \
    cargo build ; \
    fi \
    fi

# Copy the binary to a clean location
RUN if [ "$BUILD_MODE" = "release" ] ; then \
    cp target/release/simple-spa-server /usr/local/bin/ ; \
    else \
    cp target/debug/simple-spa-server /usr/local/bin/ ; \
    fi

# If bundle feature is enabled, clear the www directory contents
# since they're already embedded in the binary
RUN if [[ "$FEATURES" == *"bundle"* ]]; then \
    echo "Bundle feature enabled, cleaning www directory contents" && \
    find ./www -mindepth 1 -delete; \
    fi

# Runtime stage
FROM alpine:latest

# Install runtime dependencies
RUN apk add --no-cache ca-certificates

# Copy the binary from builder stage
COPY --from=builder /usr/local/bin/simple-spa-server /usr/local/bin/

# Create app directory
WORKDIR /app

# Copy www directory (will be empty if bundle feature was enabled)
COPY --from=builder --chown=nobody:nobody /usr/src/app/www ./www

# Expose the default port
EXPOSE 8080

# Run the server with appropriate arguments based on bundle feature
ENTRYPOINT ["simple-spa-server"]
CMD ["--serve-dir", "/app/www", "--listen", ":8080"]
