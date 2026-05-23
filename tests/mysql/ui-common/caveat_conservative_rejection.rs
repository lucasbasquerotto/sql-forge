use sql_forge::sql_forge;

fn main() {
    let include_org = false;
    let can_read_org_name = true;

    #[derive(sqlx::FromRow)]
    struct Row {
        field_1: i64,
        field_2: String,
    }

    let _ = sql_forge!(
        Row,
        r#"
        SELECT t1.id AS field_1, {#field_2}
        FROM users t1
        {#join_org}
        WHERE 1 = 1
        "#,
        (
            #join_org = match include_org {
                false => "",
                true => " JOIN organisations o ON o.id = t1.id ",
            },
            #field_2 = match include_org && can_read_org_name {
                true => "o.name AS field_2",
                false => "t1.name AS field_2",
            },
        ),
    );
}