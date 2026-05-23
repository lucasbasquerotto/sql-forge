use sql_forge::sql_forge;

mod support;

fn main() {
    let include_org = true;

    let _ = sql_forge!(
        support::User,
        "SELECT id, name FROM users {#join_org} {#filter_org}",
        (
            #(join_org, filter_org) = match include_org {
                true => ("", "", ""),
                false => ("", ""),
            },
        ),
    );
}