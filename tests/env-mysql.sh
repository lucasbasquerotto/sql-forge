#!/bin/bash
set -euo pipefail

export ENV_DB_TYPE="mysql"
export DATABASE_URL="mysql://root:root@mysql:3306/sql_forge_test"
export SQL_FORGE_DB_TYPE="sqlx::MySql"
export CARGO_TARGET_DIR="target/$ENV_DB_TYPE"
export ENV_DIR="tests/$ENV_DB_TYPE"
export SQLX_OFFLINE_DIR="/app/$ENV_DIR/.sqlx"
export TRYBUILD=overwrite

exec "${@}"