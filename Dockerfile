ARG RUST_VERSION=1.75.0
ARG APP_NAME=rs-tree

FROM rust:${RUST_VERSION}-alpine AS build
ARG APP_NAME
WORKDIR /app

RUN apk add --no-cache musl-dev openssl-dev openssl-libs-static git

ARG SQLX_OFFLINE=true 

RUN --mount=type=bind,source=src,target=src \
    --mount=type=bind,source=Cargo.toml,target=Cargo.toml \
    --mount=type=bind,source=Cargo.lock,target=Cargo.lock \
    --mount=type=bind,source=build.rs,target=build.rs \
    --mount=type=bind,source=README.md,target=README.md \
    --mount=type=bind,source=.git,target=.git \
    --mount=type=bind,source=migrations,target=migrations \
    --mount=type=bind,source=.sqlx,target=.sqlx \
    --mount=type=cache,target=/app/target/ \
    --mount=type=cache,target=/usr/local/cargo/registry/ \
    <<EOF
set -e
cargo build --locked --profile=dev-docker
cp ./target/dev-docker/$APP_NAME /bin/server
EOF

FROM alpine:3.18 AS final

ARG UID=10001
RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    appuser
USER appuser

COPY --from=build /bin/server /bin/

CMD ["/bin/server"]
