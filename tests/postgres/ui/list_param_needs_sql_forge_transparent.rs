use sql_forge::sql_forge;

#[derive(sqlx::FromRow)]
struct User {
    id: i64,
    name: String,
}

#[derive(sqlx::Type)]
#[sqlx(transparent)]
struct TransparentId(pub i64);

fn main() {
    let ids = vec![TransparentId(1)];
    let _ = sql_forge!(
        User,
        "SELECT id, name FROM users WHERE id IN (:ids[])",
        ( :ids = ids ),
    );
}
