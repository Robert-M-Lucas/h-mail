#!/usr/bin/env bash

dropdb -U robert -h 127.0.0.1 -p 5432 db
createdb -U robert -h 127.0.0.1 -p 5432 db
diesel database reset --database-url "postgres://robert@localhost:5432/db"
