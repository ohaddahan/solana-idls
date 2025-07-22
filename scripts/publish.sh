#!/usr/bin/env bash
export _PWD="$(pwd)"
export ROOT="$(git rev-parse --show-toplevel)"
for dir in "${ROOT}"/libs/* ; do
    if [ -d "$dir" ]; then
        cd "$dir" || exit 1
        echo "Running command in $dir"
        cargo publish
        if [ $? -ne 0 ]; then
            echo "Error in $dir, stopping"
            exit 1
        fi
        cd "${ROOT}"
    fi
done