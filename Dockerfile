# syntax=docker/dockerfile:1.4

FROM rust:1.66-bullseye as builder

    WORKDIR /app

    COPY Cargo.toml Cargo.lock ./
    COPY src src

    RUN --mount=type=cache,target=/root/.cargo/registry \
        --mount=type=cache,target=/app/target,sharing=private \
        cargo install --locked --path . --root /out

FROM scratch as build-content

    WORKDIR /app

    COPY --from=builder --link /out/bin/web-portal-lite ./

    COPY Rocket.toml LICENSE.txt THIRD-PARTY.txt ./
    COPY static ./static
    COPY templates ./templates

FROM gcr.io/distroless/cc-debian11

    WORKDIR /app

    EXPOSE 8000
    ENV ROCKET_CONFIG_PATH=/app/config.yml
    ENV ROCKET_ICONS_PATH=/app/icons
    ENV ROCKET_ADDRESS=0.0.0.0
    ENV ROCKET_PORT=8000
    ENV ROCKET_TEMPLATE_DIR=/app/templates

    COPY --from=build-content --link /app /app

    ENTRYPOINT ["./web-portal-lite"]
    CMD ["serve"]
