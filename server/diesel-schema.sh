#!/usr/bin/env bash

export DATABASE_URL="sqlite://data.sqlite"

echo "#![allow(clippy::all)]" > src/database/schema.rs
echo "#![allow(warnings)]" >> src/database/schema.rs

diesel print-schema >> src/database/schema.rs