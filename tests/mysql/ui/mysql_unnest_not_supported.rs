use sql_forge::sql_forge;

fn main() {
    #[derive(sqlx::FromRow)]
    struct Row {
        id: i64,
    }

    let _ = sql_forge!(
        Row,
        "SELECT UNNEST(1) AS id",
    );
}