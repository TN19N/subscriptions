#### BASE STAGE
#### Contains base image
FROM lukemathwalker/cargo-chef:0.1.72-rust-1.89.0-alpine3.20 AS base
WORKDIR /app
# Install dependencies
RUN apk add --no-cache sccache build-base
ENV RUSTC_WRAPPER=sccache

#### SKELETON STAGE
#### Scaffolds repository skeleton structures.
FROM base AS planner
COPY . .
# Compute a lock-like file for our project
RUN --mount=type=cache,target=/root/.cache/sccache,sharing=locked \
    cargo chef prepare  --recipe-path recipe.json

#### BUILD STAGE
#### Builds the project.
FROM base AS builder
COPY --from=planner /app/recipe.json recipe.json
# Build our project dependencies, not our application!
RUN --mount=type=cache,target=/root/.cache/sccache,sharing=locked \
    cargo chef cook --release --recipe-path recipe.json
# Up to this point, if our dependency tree stays the same,
# all layers should be cached.
COPY . .
# Build our project
RUN --mount=type=cache,target=/root/.cache/sccache,sharing=locked \
    cargo build --release

#### START STAGE
#### start the project.
FROM alpine:3.22 AS start
# Copy built sources
COPY --from=builder /app/target/release/subscriptions .
CMD ["./subscriptions"]
