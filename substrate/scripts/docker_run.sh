#!/usr/bin/env bash

set -e

echo "*** Start Substrate node template ***"

cd $(dirname ${BASH_SOURCE[0]})/..

export ABCI_APP_STATE=$(cat ./scripts/test_genesis.json)

docker-compose down --remove-orphans
docker-compose run --rm --service-ports dev $@
