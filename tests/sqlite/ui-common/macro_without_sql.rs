use sql_forge::sql_forge;

mod support;

fn main() {
    let _ = sql_forge!(support::User);
}