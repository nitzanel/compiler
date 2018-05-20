#!/bin/bash
set -eu

errors=0

cd "${0%/*}/.."

echo "Running tests"
if result=$(cargo test --color always 2>&1); then
    echo "passed 'cargo test'"
else
    echo "$result"
    errors=1
fi

if [ "$errors" != 0 ]; then
    echo "Failed "
    exit 1
else
    echo "OK"
fi
