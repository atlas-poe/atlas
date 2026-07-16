#!/bin/bash

# Configure git to use our hooks
git config core.hooksPath .githooks

echo "✅ Git hooks configured!"
echo ""
echo "Hooks installed:"
echo "  - commit-msg: Validates conventional commit format"
echo "  - pre-push: Runs format, clippy, and tests"
echo ""
echo "Commit message format: <type>[scope][!]: <description>"
echo "Types: feat, fix, docs, style, refactor, perf, test, build, ci, chore, revert"
