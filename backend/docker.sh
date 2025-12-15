#!/bin/ash

# This is the Dockerfile's entrypoint script.
# https://docs.docker.com/config/containers/multi-service_container/

chromedriver &
./email_validator_backend
