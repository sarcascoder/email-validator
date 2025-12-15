#!/usr/bin/env bash

# Install script for Email Validator Backend on an OVH Debian 11 server.
# As a postinstall, this script is meant to be run once, but for convenience,
# it's actually idempotent.

# Fail early.
set -e

# You can change the default values of these variables inline here, or by
# setting them in the environment before running this script, e.g.:
# EV__BACKEND_NAME="my-own-name" ./debian11.sh

# An unique identifier for the backend.
EV__BACKEND_NAME=${EV__BACKEND_NAME:-"backend1.mycompany.com"}
# Docker image tag for your backend image.
EV_VERSION=${EV_VERSION:-"latest"}
# Optional: Send bug reports to a Sentry.io dashboard.
EV__SENTRY_DSN=${EV__SENTRY_DSN:-}
# Protect the backend from the public via a `x-email-validator-secret` header.
EV__HEADER_SECRET=${EV__HEADER_SECRET:-}
# For the "FROM" field in emails.
EV__FROM_EMAIL=${EV__FROM_EMAIL:-"hello@mycompany.com"}
# For the "EHLO" field in emails. This should ideally match the server's
# reverse DNS entry for optimal results.
EV__HELLO_NAME=${EV__HELLO_NAME:-"backend1.mycompany.com"}
# Timeout for SMTP connections in seconds.
EV__SMTP_TIMEOUT=${EV__SMTP_TIMEOUT:-"90"}
# Proxy settings.
EV__PROXY__HOST=${EV__PROXY__HOST:-}
EV__PROXY__PORT=${EV__PROXY__PORT:-}
EV__PROXY__USERNAME=${EV__PROXY__USERNAME:-}
EV__PROXY__PASSWORD=${EV__PROXY__PASSWORD:-}
# Logging. Setup to "debug" to show all logs.
RUST_LOG=${RUST_LOG:-"info"}

echo "Installing Email Validator backend $EV_VERSION on host $EV__BACKEND_NAME..."

# Install Docker
# https://docs.docker.com/engine/install/debian/
sudo apt-get update
sudo apt-get upgrade --yes
sudo apt-get install \
    ca-certificates \
    curl \
    gnupg \
    lsb-release \
    --yes
sudo mkdir -p /etc/apt/keyrings
curl -fsSL https://download.docker.com/linux/debian/gpg | sudo gpg --dearmor -o /etc/apt/keyrings/docker.gpg --yes
echo \
    "deb [arch=$(dpkg --print-architecture) signed-by=/etc/apt/keyrings/docker.gpg] https://download.docker.com/linux/debian \
    $(lsb_release -cs) stable" | sudo tee /etc/apt/sources.list.d/docker.list > /dev/null
sudo apt-get update
sudo apt-get install docker-ce docker-ce-cli containerd.io docker-compose-plugin --yes

# Create `docker` group
# https://docs.docker.com/engine/install/linux-postinstall/
getent group docker || sudo groupadd docker
sudo usermod -aG docker debian
# Reload users and groups, see
# https://superuser.com/questions/272061/reload-a-linux-users-group-assignments-without-logging-out
sudo su - $USER << EOF

# Stop all previous docker containers and images
docker stop email_validator_backend
docker rm email_validator_backend

# Run the backend
docker run -d \
    -e RUST_LOG=$RUST_LOG \
    -e EV__BACKEND_NAME=$EV__BACKEND_NAME \
    -e EV__SENTRY_DSN=$EV__SENTRY_DSN \
    -e EV__HEADER_SECRET=$EV__HEADER_SECRET \
    -e EV__FROM_EMAIL=$EV__FROM_EMAIL \
    -e EV__HELLO_NAME=$EV__HELLO_NAME \
    -e EV__SMTP_TIMEOUT=$EV__SMTP_TIMEOUT \
    -p 80:8080 \
    --name email_validator_backend \
    yourorg/email-validator-backend:$EV_VERSION

echo "Everything set. You can close this terminal."
EOF
