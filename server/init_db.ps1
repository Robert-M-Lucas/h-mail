New-Item -ItemType Directory -Force -Path "./pgdata" | Out-Null
New-Item -ItemType Directory -Force -Path "./pg-run" | Out-Null

$PGDATA = (Resolve-Path "./pgdata").Path

initdb -D $PGDATA
