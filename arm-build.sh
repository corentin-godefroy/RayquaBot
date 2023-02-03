#! /bin/bash

set -ex

image="rayquabot:latest"

docker build --platform linux/arm64 --tag "$image" .

container="$(docker create --platform linux/arm64 "$image")"

mkdir -p build
docker cp "$container":/app/target ./target-arm

docker rm "$container"
