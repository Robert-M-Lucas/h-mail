#!/usr/bin/env bash

CURRENT_USER=$(id -u -n)

dropdb -U "${CURRENT_USER}" -h 127.0.0.1 -p 5432 db
createdb -U "${CURRENT_USER}" -h 127.0.0.1 -p 5432 db
diesel database setup --database-url "postgres://${CURRENT_USER}@localhost:5432/db"
