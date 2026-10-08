#!/usr/bin/env bash
# Dry run by default. --apply creates or updates marker-owned issues and records
# their URLs durably; repeated runs resume without creating duplicates.
set -euo pipefail
exec python3 "$(dirname "$0")/file-issues.py" "$@"
