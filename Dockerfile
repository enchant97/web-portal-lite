# syntax=docker/dockerfile:1.4

FROM blackdex/rust-musl:x86_64-musl-stable as builder

    WORKDIR /usr/src/app

    COPY Cargo.toml Cargo.toml
    COPY web-portal-lite web-portal-lite
    COPY web-portal-lite-cli web-portal-lite-cli
    COPY web-portal-lite-core web-portal-lite-core

    RUN cargo build --release

FROM scratch

    WORKDIR /app

    COPY --from=builder --link /usr/src/app/target/x86_64-unknown-linux-musl/release/web_portal_lite .
    COPY --from=builder --link /usr/src/app/target/x86_64-unknown-linux-musl/release/web_portal_lite_cli .

    COPY Rocket.toml .
    COPY web-portal-lite/static /usr/src/app/web-portal-lite/static
    COPY templates /usr/src/app/web-portal-lite/templates

    COPY LICENSE.txt .
    COPY THIRD-PARTY.txt .

    EXPOSE 8000
    ENV ROCKET_CONFIG_PATH=/app/config.yml
    ENV ROCKET_ICONS_PATH=/app/icons
    ENV ROCKET_ADDRESS=0.0.0.0
    ENV ROCKET_PORT=8000
    ENV ROCKET_TEMPLATE_DIR=/usr/src/app/web-portal-lite/templates

    CMD ["./web_portal_lite"]
