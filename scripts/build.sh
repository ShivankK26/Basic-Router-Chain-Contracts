#!/bin/bash

set -e
rustup target add wasm32-unknown-unknown
cargo wasm
# building the wasm artifacts

machine_info=$(uname -a)
apple_identifier="arm64"

echo $machine_info $apple_identifier

if echo "$machine_info" | grep -q "$apple_identifier"; then
  echo "Apple-based chipset"
  docker run --rm -v "$(pwd)":/code \
    --mount type=volume,source="voyager_cache",target=/code/target \
    --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
    cosmwasm/workspace-optimizer-arm64:0.12.13
else
  echo "Intel-based chipset"
  docker run --rm -v "$(pwd)":/code \
    --mount type=volume,source="voyager_cache",target=/code/target \
    --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
    cosmwasm/workspace-optimizer:0.12.13
fi