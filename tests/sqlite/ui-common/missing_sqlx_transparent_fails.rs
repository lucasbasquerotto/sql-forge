use sql_forge::sql_forge;

mod support;

fn main() {
    let ids = vec![support::RawId(1)];
    let _ = sql_forge!(
        support::User,
        "SELECT id, name FROM users WHERE id IN (:ids[])",
        ( :ids = ids ),
    );
}
