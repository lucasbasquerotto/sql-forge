use sql_forge::sql_forge;

mod support;

fn main() {
    let _ = sql_forge!(
        (
            >user = support::User,
            >amount = support::AmountResult,
        ),
        "SELECT id, name FROM users LIMIT 1",
    );
}