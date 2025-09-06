#!/usr/bin/env bash

PGDATA=$(realpath ./pgdata)
PGSOCK=$(realpath ./pg-run)

mkdir -p "$PGDATA" "$PGSOCK"
initdb -D "$PGDATA"