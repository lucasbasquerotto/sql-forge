#!/bin/bash
set -euo pipefail

export ENV_DB_TYPE="postgres"
export DATABASE_URL="postgres://postgres:root@pg:5432/sql_forge_test"
export SQL_FORGE_DB_TYPE="sqlx::Postgres"
export CARGO_TARGET_DIR="target/$ENV_DB_TYPE"
export ENV_DIR="tests/$ENV_DB_TYPE"
export SQLX_OFFLINE_DIR="/app/$ENV_DIR/.sqlx"
export TRYBUILD=overwrite

exec "${@}"