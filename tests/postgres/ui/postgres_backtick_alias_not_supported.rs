use sql_forge::sql_forge;

fn main() {
    #[derive(sqlx::FromRow)]
    struct Row {
        field_name: i64,
    }

    let _ = sql_forge!(
        Row,
        "SELECT 1 AS `field_name: _`",
    );
}