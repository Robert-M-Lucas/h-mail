#!/usr/bin/env bash
set -euo pipefail

PGDATA=$(realpath ./pgdata)
PGSOCK=$(realpath ./pg-run)

mkdir -p "$PGDATA" "$PGSOCK"

postgres -D "$PGDATA" -k "$PGSOCK"
