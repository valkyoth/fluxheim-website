FROM docker.io/library/rust:1.97.0-slim-bookworm@sha256:6d220bf85c74e842a79da63997af8d2e74455c0b8847d8bb3a5888572334991d AS builder

WORKDIR /build

COPY Cargo.toml Cargo.lock* rust-toolchain.toml build.rs ./
COPY src/ src/
COPY content/ content/
COPY config/ config/

# The official 1.97.1 image was not yet published when this toolchain shipped.
# rustup reads rust-toolchain.toml, installs 1.97.1, and verifies the compiler
# before Cargo builds the release binary.
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
