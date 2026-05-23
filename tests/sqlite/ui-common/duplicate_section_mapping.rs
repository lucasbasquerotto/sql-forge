use sql_forge::sql_forge;

mod support;

fn main() {
    let _ = sql_forge!(
        support::User,
        "SELECT id, name FROM users {#filter}",
        (
            #filter = " WHERE id = 1 ",
            #filter = " WHERE id = 2 ",
        ),
    );
}