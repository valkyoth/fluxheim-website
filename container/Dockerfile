# Stage 1: Build fluxheim v1.6.0 from source
FROM rust:1.96-slim-bookworm AS builder

RUN apt-get update && apt-get install -y --no-install-recommends \
    git \
    cmake \
    make \
    g++ \
    pkg-config \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /build

RUN git clone --depth=1 --branch v1.6.0 \
    https://github.com/valkyoth/fluxheim.git .

RUN cargo build --release --locked --no-default-features \
    --features profile-static-site

# Stage 2: Package the local website content
FROM alpine:3 AS site

WORKDIR /site

COPY index.html download.html changelog.html ./
COPY assets/ assets/
COPY docs/ docs/
COPY conf/ conf/

# Stage 3: Hardened Wolfi runtime
# Wolfi uses glibc, so Debian-built binaries copy in without recompilation.
FROM cgr.dev/chainguard/wolfi-base

RUN apk add --no-cache ca-certificates-bundle

RUN addgroup -S fluxheim && \
    adduser -S -G fluxheim fluxheim && \
    mkdir -p /etc/fluxheim /srv/sites/fluxheim && \
    chown fluxheim:fluxheim /etc/fluxheim /srv/sites/fluxheim

COPY --from=builder /build/target/release/fluxheim /usr/local/bin/fluxheim

COPY --from=site --chown=fluxheim:fluxheim /site/index.html /site/download.html /site/changelog.html /srv/sites/fluxheim/
COPY --from=site --chown=fluxheim:fluxheim /site/assets/ /srv/sites/fluxheim/assets/
COPY --from=site --chown=fluxheim:fluxheim /site/docs/ /srv/sites/fluxheim/docs/
COPY --from=site --chown=fluxheim:fluxheim /site/conf/fluxheim.toml /etc/fluxheim/fluxheim.toml

EXPOSE 8080

USER fluxheim

CMD ["fluxheim", "--config", "/etc/fluxheim/fluxheim.toml"]
