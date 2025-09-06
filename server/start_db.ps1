Set-StrictMode -Version Latest
$ErrorActionPreference = "Stop"

New-Item -ItemType Directory -Force -Path "./pgdata" | Out-Null
New-Item -ItemType Directory -Force -Path "./pg-run" | Out-Null

$PGDATA = Resolve-Path ./pgdata
$PGSOCK = Resolve-Path ./pg-run

postgres -D $PGDATA -k $PGSOCK
