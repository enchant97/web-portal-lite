# syntax=docker/dockerfile:1.4

FROM blackdex/rust-musl:x86_64-musl-stable as builder

    WORKDIR /src

    COPY Cargo.toml .
    COPY web-portal-lite web-portal-lite
    COPY web-portal-lite-cli web-portal-lite-cli
    COPY web-portal-lite-core web-portal-lite-core

    RUN --mount=type=cache,target=/root/.cargo/registry \
        --mount=type=cache,target=/src/target,sharing=private \
        cargo install --path web-portal-lite-cli --root /out \
        && cargo install --path web-portal-lite --root /out

FROM scratch as build-content

    WORKDIR /app

    COPY --from=builder --link /out/bin/web-portal-lite /out/bin/web-portal-lite-cli ./
    COPY Rocket.toml LICENSE.txt THIRD-PARTY.txt ./

    WORKDIR /src/web-portal-lite

    COPY web-portal-lite/static ./static
    COPY templates ./templates

FROM scratch

    WORKDIR /app

    EXPOSE 8000
    ENV ROCKET_CONFIG_PATH=/app/config.yml
    ENV ROCKET_ICONS_PATH=/app/icons
    ENV ROCKET_ADDRESS=0.0.0.0
    ENV ROCKET_PORT=8000
    ENV ROCKET_TEMPLATE_DIR=/src/web-portal-lite/templates

    COPY --from=build-content --link /src /src
    COPY --from=build-content --link /app /app

    CMD ["./web-portal-lite"]
