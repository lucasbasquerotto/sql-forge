#!/bin/bash
set -euo pipefail

export ENV_DB_TYPE="sqlite"
export DATABASE_URL="sqlite:/app/sql_forge_test.db"
export SQL_FORGE_DB_TYPE="sqlx::Sqlite"
export CARGO_TARGET_DIR="target/$ENV_DB_TYPE"
export ENV_DIR="tests/$ENV_DB_TYPE"
export SQLX_OFFLINE_DIR="/app/$ENV_DIR/.sqlx"
export TRYBUILD=overwrite

exec "${@}"