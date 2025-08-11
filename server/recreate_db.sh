#!/usr/bin/env bash

rm data.sqlite
diesel setup --database-url "sqlite://data.sqlite"