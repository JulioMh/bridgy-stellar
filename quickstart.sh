#!/bin/bash

set -e

case "$1" in
standalone)
    echo "Using standalone network"
    ARGS="--standalone"
    ;;
local)
    echo "Using local network"
    ARGS="--local"
    ;;
futurenet)
    echo "Using Futurenet network"
    ARGS="--futurenet"
    ;;
*)
    echo "Usage: $0 standalone|futurenet"
    exit 1
    ;;
esac

shift

# Run the soroban-preview container
# Remember to do:
# make build-docker

echo "Creating docker soroban network"
(docker network inspect soroban-network -f '{{.Id}}' 2>/dev/null) \
  || docker network create soroban-network

echo "Searching for a previous soroban-preview docker container"
containerID=$(docker ps --filter="name=soroban-preview" --all --quiet)
if [[ ${containerID} ]]; then
    echo "Start removing soroban-preview container."
    docker rm --force soroban-preview
    echo "Finished removing soroban-preview container."
else
    echo "No previous soroban-preview container was found"
fi

currentDir=$(pwd)
docker run -dti \
  --volume ${currentDir}:/workspace \
  --name soroban-preview \
  -p 8001:8000 \
  --ipc=host \
  --network soroban-network \
  esteblock/soroban-preview:21.0.1

# Run the stellar quickstart image

docker run --rm -ti \
  --name stellar \
  --network soroban-network \
  -p 8000:8000 \
  stellar/quickstart:testing \
  $ARGS \
  --enable-soroban-rpc \
  "$@" # Pass through args from the CLI