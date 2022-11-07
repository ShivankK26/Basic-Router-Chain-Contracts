#!/bin/bash

# exit when any command fails
set -e

file_name=$(echo $1 | tr "-" _)

# storing on chain the wasm code
result=$(yes 12345678 | routerd tx wasm store artifacts/${file_name}.wasm \
        --from genesis --chain-id "router-1" --fees 10000000000000000router --gas \
        20000000 --output json --broadcast-mode block --yes )

code_id=$(echo $result | jq -r '.logs[0].events[-1].attributes[0].value')
if [ -z "$code_id" ]; then
        echo "The code id is not set";
        exit 1;
fi

echo $1 $2  $code_id
sleep 20

# instantiating the wasm code
yes 12345678 | routerd tx wasm instantiate $code_id $2 --label=$2 \
        --from=genesis --chain-id="router-1" --yes --fees=1000000000000000router \
        --gas=2000000 --no-admin --output json

echo "Contract Deployed"
sleep 20
routerd query wasm list-contract-by-code $code_id --output json | jq -r '.contracts[0]'