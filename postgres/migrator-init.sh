#!/bin/bash

SQL_DIR="${SQL_DIR:-./tests/data/schema}"
MIGRATIONS_DIR="${MIGRATIONS_DIR:-./tests/data/migrations}"

set -e

echo 'CREATE DATABASE social_net' | psql postgres://postgres:postgres@postgres
echo 'database social_net created'

echo 'GRANT ALL PRIVILEGES ON DATABASE social_net TO postgres' | psql postgres://postgres:postgres@postgres
echo 'postgres user granted'

find $SQL_DIR/ -name apply.sql | sort -V | xargs -I{} psql postgres://postgres:postgres@postgres/social_net -f {} -v ON_ERROR_STOP=1
echo 'release migrations upped'

find $MIGRATIONS_DIR/ -name apply.sql | sort -V | xargs -I{} psql postgres://postgres:postgres@postgres/social_net -f {} -v ON_ERROR_STOP=1
echo 'social_net migrations upped'
