#!/usr/bin/env bash
set -euo pipefail

PGDATA=$(realpath ./pgdata)
PGSOCK=$(realpath ./pg-run)

postgres -D "$PGDATA" -k "$PGSOCK"
