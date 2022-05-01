#!/usr/bin/env bash

set -x
set -eo pipefail
# check if psql and sqlx-cli are installed
if ! [ -x "$(command -v psql)" ]; then
  echo >&2 "Error: psql is not installed"
  exit 1
fi

if ! [ -x "$(command -v sqlx)" ]; then
  echo >&2 "Error: sqlx is not installed"
  echo >&2 "Use:"
  echo >&2 " cargo install --version=0.5.13 sqlx-cli --no-default-features --features postgres"
  echo >&2 "to install it."
  exit 1
fi

# check if a custom use has been set, otherwise default to "postgres"
DB_USER="${POSTGRES_USER:=postgres}"
# check if a custom password has been set, otherwise default to "password"
DB_PASSWORD="${POSTGRES_PASSWORD:=password}"
# check if a custom database name has been set, otherwise default to "newsletter"
DB_NAME="${POSTGRES_DB:=newsletter}"
# check if a custom port has been set, otherwise default to "5432"
DB_PORT="${POSTGRES_PORT:=5432}"

# launch postgres using docker
# Allow to skip docker initialization if it's already running
if [ "${SKIP_DOCKER}" != "true" ];
then
docker run \
  -e POSTGRES_USER="${DB_USER}" \
  -e POSTGRES_PASSWORD="${DB_PASSWORD}" \
  -e POSTGRES_DB="${DB_NAME}" \
  -p "${DB_PORT}:5432" \
  --name newsletter-db \
  -d postgres \
  postgres -N 1000
fi

# wait for postgres to be ready
export PGPASSWORD="${DB_PASSWORD}"
until psql -h "localhost" -U "${DB_USER}" -p "${DB_PORT}" -d "postgres" -c '\q'; do
  >&2 echo "Postgres is unavailable - sleeping"
  sleep 1
done

>&2 echo "Postgres is up and running on port ${DB_PORT}!"

export DATABASE_URL=postgres://${DB_USER}:${DB_PASSWORD}@localhost:${DB_PORT}/${DB_NAME}
sqlx database create
sqlx migrate run

>&2 echo "Postgres has been migrated, ready to go!"
