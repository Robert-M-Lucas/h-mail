#!/usr/bin/env bash

dropdb -U robert -h 127.0.0.1 -p 5432 db
createdb -U robert -h 127.0.0.1 -p 5432 db
diesel database setup --database-url "postgres://robert@localhost:5432/db"
