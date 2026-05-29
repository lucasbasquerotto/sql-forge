use sql_forge::sql_forge;

mod support;

fn main() {
    let batch_items = vec![
        support::BatchNamePrice {
            name: "Batch A".to_string(),
            price: 9999,
        },
        support::BatchNamePrice {
            name: "Batch B".to_string(),
            price: 4999,
        },
    ];

    let _ = sql_forge!(
        r#"
        INSERT INTO products (name, price, stock, category)
        VALUES (:name, :price, :stock, 'Test'), {(:name, :price, :stock, 'Test')}
        "#,
        ( :name = "Test".to_string(), :price = 100, :stock = 10 ),
        ..batch_items
    );
}
