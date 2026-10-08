#!/usr/bin/env bash
# Snapshot the live documentation index, every indexed page, and OpenAPI specs.
# Usage: bash experiments/issue-29/download-parallel-docs.sh [out-dir]
set -euo pipefail
exec python3 "$(dirname "$0")/download-parallel-docs.py" "${1:-docs/case-studies/issue-29/data/parallel-docs}"
