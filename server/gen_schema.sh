#!/usr/bin/env bash

export DATABASE_URL="postgres://robert@localhost:5432/db"

echo "#![allow(clippy::all)]" > src/database/schema.rs
echo "#![allow(warnings)]" >> src/database/schema.rs

diesel print-schema >> src/database/diesel_interface/schema.rs