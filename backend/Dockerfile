# From https://shaneutt.com/blog/rust-fast-small-docker-image-builds/

# ------------------------------------------------------------------------------
# Cargo Build Stage
# ------------------------------------------------------------------------------

FROM messense/rust-musl-cross:x86_64-musl as cargo-build

WORKDIR /usr/src/email-validator

RUN rm -f target/x86_64-unknown-linux-musl/release/deps/email_validator*

COPY . .

ENV SQLX_OFFLINE=true

RUN cargo build --bin email_validator_backend --release --target=x86_64-unknown-linux-musl

# ------------------------------------------------------------------------------
# Final Stage
# ------------------------------------------------------------------------------

FROM zenika/alpine-chrome:123

WORKDIR /home/email-validator/

USER root

# Install chromedriver
# https://github.com/Zenika/alpine-chrome/blob/master/with-chromedriver/Dockerfile
RUN apk add --no-cache chromium-chromedriver

COPY --from=cargo-build /usr/src/email-validator/target/x86_64-unknown-linux-musl/release/email_validator_backend .
COPY --from=cargo-build /usr/src/email-validator/backend/docker.sh .
COPY --from=cargo-build /usr/src/email-validator/backend/backend_config.toml .

RUN chown chrome:chrome email_validator_backend
RUN chown chrome:chrome docker.sh

# User chrome was created in zenika/alpine-chrome
USER chrome

ENV RUST_LOG=email_validator=info
ENV EV__HTTP_HOST=0.0.0.0
# Currently this Dockerfile is mainly used for single-shot verifications, so we
# disable the worker by default.
ENV EV__WORKER__ENABLE=false

EXPOSE 8080

# Remove entrypoint from parent Docker file
# https://stackoverflow.com/questions/40122152/how-to-remove-entrypoint-from-parent-image-on-dockerfile
ENTRYPOINT []

CMD ["./docker.sh"]
