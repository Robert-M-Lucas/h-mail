#!/usr/bin/env bash

CURRENT_USER=$(id -u -n)

export DATABASE_URL="postgres://${CURRENT_USER}@localhost:5432/db"

echo "#![allow(clippy::all)]" > src/database/schema.rs
echo "#![allow(warnings)]" >> src/database/schema.rs

diesel print-schema >> src/database/diesel_interface/schema.rs