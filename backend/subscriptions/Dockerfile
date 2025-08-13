#### BASE STAGE
#### Contains base image
FROM alpine:3.22.1 AS base

WORKDIR /app

#### BASE-MOON STAGE
#### Installs moon.
FROM base AS base-moon

# Install dependencies
RUN apk add --no-cache curl bash libgcc musl-dev build-base sccache

# sccache
ENV RUSTC_WRAPPER=/usr/bin/sccache

# Install moon binary
RUN curl -fsSL https://moonrepo.dev/install/moon.sh | bash
ENV PATH="/root/.moon/bin:$PATH"

#### SKELETON STAGE
#### Scaffolds repository skeleton structures.

FROM base-moon AS skeleton

# Copy entire repository and scaffold
COPY . .
RUN moon docker scaffold subscriptions

#### BUILD STAGE
#### Builds the project.

FROM base-moon AS build

# Copy workspace configs
COPY --from=skeleton /app/.moon/docker/workspace .

# Install dependencies
RUN --mount=type=cache,target=/root/.cache/sccache,sharing=locked \
    moon docker setup

# Copy project sources
COPY --from=skeleton /app/.moon/docker/sources .

# Build the project
RUN --mount=type=cache,target=/root/.cache/sccache,sharing=locked \
    moon run subscriptions:build -- --release

#### START STAGE
#### Runs the project.

FROM base AS start

# Copy built sources
COPY --from=build /app/target/release/subscriptions .

EXPOSE 8080

CMD ["./subscriptions"]
