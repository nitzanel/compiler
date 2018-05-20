#!/bin/bash

GIT_DIR=$(git rev-parse --git-dir)

echo "Installing hooks..."
rm $GIT_DIR/hooks/pre-commit 2>/dev/null
ln -s ../../scripts/pre-commit.sh $GIT_DIR/hooks/pre-commit
echo "Done!"
