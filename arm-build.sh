#! /bin/bash

set -ex

image="rayquabot:latest"

docker buildx build --platform linux/arm64 --tag "$image" . --load

container="$(docker create --platform linux/arm64 "$image")"

mkdir -p build
docker cp "$container":/app/target ./target-arm

docker rm "$container"

