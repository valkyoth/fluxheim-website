FROM docker.io/library/rust:1.98.0-slim-bookworm@sha256:94e9efa4033213dbb70d4f665527e7ece3944ddb7ba1dd2e43f6fd6e2490af58 AS builder

WORKDIR /build

COPY Cargo.toml Cargo.lock* rust-toolchain.toml build.rs ./
COPY src/ src/
COPY content/ content/
COPY config/ config/

RUN rustc --version | grep -F "rustc 1.97.1 " && \
    cargo build --release --locked

FROM cgr.dev/chainguard/wolfi-base:latest@sha256:02dab76bd852a70556b5b2002195c8a5fdab77d323c433bf6642aab080489795

RUN addgroup -S fluxheim && \
    adduser -S -G fluxheim fluxheim && \
    mkdir -p /srv/fluxheim-website/assets /srv/fluxheim-website/conf && \
    chown -R root:root /srv/fluxheim-website

WORKDIR /srv/fluxheim-website

COPY --from=builder /build/target/release/fluxheim-website /usr/local/bin/fluxheim-website
COPY --chown=root:root index.html download.html changelog.html cookies.html privacy.html gdpr.html ./
COPY --chown=root:root assets/ assets/
COPY --chown=root:root conf/ conf/
COPY --chown=root:root docs/ docs/
COPY --chown=root:root localized/ localized/

RUN chmod 0555 /usr/local/bin/fluxheim-website && \
    chmod -R a-w /srv/fluxheim-website

EXPOSE 8080

USER fluxheim

ENV FLUXHEIM_HSTS=enabled
ENV FLUXHEIM_OTLP=disabled
ENV OTEL_EXPORTER_OTLP_HEADERS=""
ENV OTEL_EXPORTER_OTLP_PROTOCOL=http/protobuf

CMD ["fluxheim-website"]
