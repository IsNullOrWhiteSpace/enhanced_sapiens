#!/usr/bin/env bash

set -e

IMAGE_NAME="sapiens_bot"

test -e .env || (echo "Please create a .env file with the required environment variables" && exit 1)

ARGS=("$@")

# test if docker cli is installed
if ! command -v docker > /dev/null 2>&1; then
    echo "Docker is not installed. Please install it first."
    exit 1
fi

# test if docker is installed and running
if ! docker info > /dev/null 2