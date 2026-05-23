use sql_forge::sql_forge;

mod support;

fn main() {
    let _ = sql_forge!(
        support::User,
        r#"SELECT id, name FROM users WHERE name = "abc:def""#,
    );
}