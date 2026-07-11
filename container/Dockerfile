FROM docker.io/library/rust:1.96.1-slim-bookworm@sha256:e18a79fc84dfcfc3ab5ba72290398a644c135c97eaa881447fddc354ee4701a3 AS builder

WORKDIR /build

COPY Cargo.toml Cargo.lock* rust-toolchain.toml build.rs ./
COPY src/ src/
COPY content/ content/
COPY config/ config/

RUN cargo build --release --locked

FROM cgr.dev/chainguard/wolfi-base:latest@sha256:02dab76bd852a70556b5b2002195c8a5fdab77d323c433bf6642aab080489795

RUN apk add --no-cache ca-certificates-bundle

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
