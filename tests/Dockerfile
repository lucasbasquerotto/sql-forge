FROM rust:1.95.0

RUN apt-get update && \
    apt-get install -y --no-install-recommends \
        default-libmysqlclient-dev \
        libpq-dev \
        pkg-config \
        sqlite3 \
    && rm -rf /var/lib/apt/lists/*

RUN cargo install sqlx-cli --version 0.8.6 && \
    rustup component add rustfmt clippy

RUN cargo install cargo-expand

COPY tests/sql-forge-with-db.sh /usr/local/bin/sql-forge-with-db
COPY tests/sql-forge.sh  /usr/local/bin/sql-forge
COPY tests/env-mysql.sh  /usr/local/bin/env-mysql
COPY tests/env-postgres.sh  /usr/local/bin/env-postgres
COPY tests/env-sqlite.sh  /usr/local/bin/env-sqlite

RUN chmod +x \
    /usr/local/bin/sql-forge-with-db \
    /usr/local/bin/sql-forge \
    /usr/local/bin/env-mysql \
    /usr/local/bin/env-postgres \
    /usr/local/bin/env-sqlite

WORKDIR /app
