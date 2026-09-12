FROM docker.io/library/rust:1.98.1-slim-bookworm@sha256:ebd900bae66fd508b466cef82d64a83a5fb34682e4c8b2797a42908bddc95a57 AS builder

WORKDIR /build

COPY Cargo.toml Cargo.lock* rust-toolchain.toml build.rs ./
COPY src/ src/
COPY content/ content/
COPY config/ config/
COPY index.html download.html changelog.html cookies.html privacy.html gdpr.html ./
COPY conf/ conf/
COPY docs/ docs/
COPY localized/ localized/

RUN rustc --version | grep -F "rustc 1.98.1 " && \
    cargo build --release --locked

FROM cgr.dev/chainguard/wolfi-base:latest@sha256:65e1acb87a2bf356b92c5f70f3980f03b4bb51dfd483c834e01557525f15c1d9

RUN addgroup -S fluxheim && \
    adduser -S -G fluxheim fluxheim && \
    mkdir -p /srv/fluxheim-website/assets && \
    chown -R root:root /srv/fluxheim-website

WORKDIR /srv/fluxheim-website

COPY --from=builder /build/target/release/fluxheim-website /usr/local/bin/fluxheim-website
COPY --chown=root:root assets/ assets/

RUN chmod 0555 /usr/local/bin/fluxheim-website && \
    chmod -R a-w /srv/fluxheim-website

EXPOSE 8080

USER fluxheim

ENV FLUXHEIM_HSTS=enabled
ENV FLUXHEIM_OTLP=disabled
ENV FLUXHEIM_WEBSITE_BIND=0.0.0.0
ENV OTEL_EXPORTER_OTLP_HEADERS=""
ENV OTEL_EXPORTER_OTLP_PROTOCOL=http/protobuf

CMD ["fluxheim-website"]
