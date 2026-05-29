use sql_forge::db_type;
use sql_forge::sql_forge;
use std::any::TypeId;

pub type AppDb = db_type!();
pub type DbPool = sqlx::Pool<AppDb>;

type Price = i64;

#[derive(Debug, PartialEq, Eq)]
#[sql_forge::sql_forge_transparent]
struct UserId(pub i64);

fn price_new(v: i64, scale: u32) -> Price {
    v * 10i64.pow(2 - scale)
}

fn price_inc(base: &Price, v: i64, scale: u32) -> Price {
    *base + price_new(v, scale)
}

#[derive(sqlx::FromRow, Debug, PartialEq)]
struct User {
    id: i64,
    name: String,
}

#[derive(sqlx::FromRow, Debug, PartialEq)]
struct Product {
    id: i64,
    name: String,
    price: Price,
    stock: i64,
    category: String,
}

#[derive(sqlx::FromRow, Debug, PartialEq)]
struct Item {
    id: i64,
    name: String,
    price: Price,
    stock: i64,
}

#[derive(sqlx::FromRow, Debug, PartialEq)]
struct AmountResult {
    total: Option<i64>,
}

struct Filter {
    max_id: i64,
    limit: i64,
}

fn db_url() -> String {
    std::env::var("DATABASE_URL").expect("DATABASE_URL not defined")
}

#[test]
fn db_type_matches_env_db_type() {
    let env_db_type = std::env::var("ENV_DB_TYPE").expect("ENV_DB_TYPE not defined");

    let expected = match env_db_type.as_str() {
        "mysql" => TypeId::of::<sqlx::MySql>(),
        "postgres" => TypeId::of::<sqlx::Postgres>(),
        "sqlite" => TypeId::of::<sqlx::Sqlite>(),
        other => panic!("unsupported ENV_DB_TYPE: {other}"),
    };

    assert_eq!(TypeId::of::<AppDb>(), expected);
}

async fn pool() -> DbPool {
    sqlx::Pool::<AppDb>::connect(&db_url())
        .await
        .expect("cannot connect to test database")
}

#[tokio::test]
async fn basic_query_with_inline_params() {
    let pool = pool().await;

    let users: Vec<User> = sql_forge!(
        User,
        "SELECT id, name FROM users WHERE id <= :max_id AND :max_id >= id LIMIT :limit",
        ( :max_id = 3i64, :limit = 10i64 )
    )
    .fetch_all(&pool)
    .await
    .expect("basic query failed");

    assert_eq!(users.len(), 3);
    assert_eq!(users[0].name, "Alice");
    assert_eq!(users[1].name, "Bob");
    assert_eq!(users[2].name, "Charlie");
}

#[tokio::test]
async fn scalar_output() {
    let pool = pool().await;

    let count: i64 = sql_forge!(
        i64,
        "SELECT COUNT(*) FROM users WHERE id > :min_id",
        ( :min_id = 2i64 )
    )
    .fetch_one(&pool)
    .await
    .expect("scalar query failed");

    assert_eq!(count, 3);
}

#[tokio::test]
async fn struct_source_params() {
    let pool = pool().await;

    let filter = Filter {
        max_id: 3,
        limit: 2,
    };

    let users: Vec<User> = sql_forge!(
        User,
        "SELECT id, name FROM users WHERE id <= :max_id LIMIT :limit",
        filter
    )
    .fetch_all(&pool)
    .await
    .expect("struct source query failed");

    assert_eq!(users.len(), 2);
}

#[tokio::test]
async fn section_dynamic_where() {
    let pool = pool().await;

    let cat = "Electronics";

    let products: Vec<Product> = sql_forge!(
        Product,
        r#"
        SELECT id, name, price, stock, category
        FROM products
        WHERE 1 = 1
        {#filter_category}
        "#,
        (
            #filter_category = (
                " AND category = :cat ",
                ( :cat = cat ),
            ),
        )
    )
    .fetch_all(&pool)
    .await
    .expect("section query failed");

    assert!(products.len() >= 3);
    for p in &products {
        assert_eq!(p.category, "Electronics");
    }
}

#[tokio::test]
async fn section_with_local_params() {
    let pool = pool().await;

    let max_id = 4i64;

    let users: Vec<User> = sql_forge!(
        User,
        "SELECT id, name FROM users {#filter}",
        (
            #filter = (
                " WHERE id <= :max_id ",
                ( :max_id = max_id ),
            ),
        )
    )
    .fetch_all(&pool)
    .await
    .expect("section with local params failed");

    assert_eq!(users.len(), 4);
}

#[tokio::test]
async fn grouped_sections() {
    let pool = pool().await;

    let include_org = true;

    #[derive(sqlx::FromRow)]
    struct Row {
        #[expect(dead_code)]
        field_1: i64,
        field_2: String,
    }

    let rows: Vec<Row> = sql_forge!(
        Row,
        r#"
        SELECT t1.id AS field_1, {#field_2}
        FROM users t1
        {#join_org}
        WHERE 1 = 1
        "#,
        (
            #(join_org, field_2) = match include_org {
                true => (
                    " JOIN organisations o ON o.id = t1.id ",
                    "o.name AS field_2",
                ),
                false => ("", "t1.name AS field_2"),
            },
        )
    )
    .fetch_all(&pool)
    .await
    .expect("grouped sections query failed");

    assert_eq!(rows.len(), 3);
    assert_eq!(rows[0].field_2, "Org Alpha");
    assert_eq!(rows[1].field_2, "Org Beta");
}

#[tokio::test]
async fn grouped_sections_with_nested_matches() {
    let pool = pool().await;

    let include_org = true;
    let can_read_org_name = false;
    let use_org_label = true;

    #[derive(sqlx::FromRow)]
    struct Row {
        field_1: i64,
        field_2: Option<String>,
        field_3: Option<String>,
    }

    let rows: Vec<Row> = sql_forge!(
        Row,
        r#"
        SELECT t1.id AS field_1, {#field_2}, {#field_3}
        FROM users t1
        {#join_org}
        WHERE 1 = 1
        "#,
        (
            #(join_org, field_2, field_3) = match include_org {
                true => (
                    " JOIN organisations o ON o.id = t1.id ",
                    match can_read_org_name {
                        true => "COALESCE(o.name, '') AS field_2",
                        false => "COALESCE(t1.name, '') AS field_2",
                    },
                    match use_org_label {
                        true => "COALESCE('org', '') AS field_3",
                        false => "COALESCE('user', '') AS field_3",
                    },
                ),
                false => (
                    "",
                    "COALESCE(t1.name, '') AS field_2",
                    "COALESCE('no_join', '') AS field_3",
                ),
            },
        )
    )
    .fetch_all(&pool)
    .await
    .expect("grouped nested sections query failed");

    assert_eq!(rows.len(), 3);
    assert_eq!(rows[0].field_1, 1);
    assert_eq!(rows[0].field_2.as_deref(), Some("Alice"));
    assert_eq!(rows[0].field_3.as_deref(), Some("org"));
}

#[tokio::test]
async fn list_parameter_in_clause() {
    let pool = pool().await;

    let ids = vec![1i64, 3, 5];

    let users: Vec<User> = sql_forge!(
        User,
        "SELECT id, name FROM users WHERE id IN (:ids[])",
        ( :ids = ids )
    )
    .fetch_all(&pool)
    .await
    .expect("list param query failed");

    assert_eq!(users.len(), 3);
    assert_eq!(users[0].id, 1);
    assert_eq!(users[1].id, 3);
    assert_eq!(users[2].id, 5);
}

#[tokio::test]
async fn list_parameter_in_main_sql_with_match_filter() {
    let pool = pool().await;

    let ids = vec![UserId(1), UserId(3), UserId(4), UserId(5)];
    let min_id = Some(3i64);
    let expected_ids = [UserId(3), UserId(4), UserId(5)];

    let users: Vec<User> = sql_forge!(
        User,
        "SELECT id, name FROM users WHERE id IN (:ids[]) {#filter} ORDER BY id",
        ( :ids = ids ),
        (
            #filter = match min_id {
                Some(min_id) => (
                    " AND id >= :min_id",
                    ( :min_id = min_id ),
                ),
                None => "",
            },
        )
    )
    .fetch_all(&pool)
    .await
    .expect("list param with match filter query failed");

    assert_eq!(users.len(), expected_ids.len());
    for (user, expected_id) in users.iter().zip(expected_ids) {
        assert_eq!(user.id, expected_id.0);
    }
}

#[tokio::test]
async fn list_parameter_with_empty_guard() {
    let pool = pool().await;

    let ids: Vec<i64> = vec![];

    let users: Vec<User> = sql_forge!(
        User,
        "SELECT id, name FROM users WHERE {#filter}",
        (
            #filter = match ids.is_empty() {
                true => "1 = 0",
                false => (
                    "id IN (:ids[])",
                    ( :ids = ids ),
                ),
            },
        )
    )
    .fetch_all(&pool)
    .await
    .expect("empty list guard query failed");

    assert_eq!(users.len(), 0);
}

#[tokio::test]
async fn multiple_results_group() {
    let pool = pool().await;

    let category_id = 1i64;
    let min_price = price_new(10000, 2);

    let group = sql_forge!(
        (
            >amount = AmountResult,
            >list   = Item,
        ),
        r#"
        SELECT {#fields}
        FROM items
        {#joins}
        WHERE items.category_id = :category_id
        AND   items.price      >= :min_price
        {#order_limit}
        "#,
        (
            :category_id = category_id,
            :min_price   = min_price,
        ),
        (
            #(fields, joins, order_limit) = match {>amount} {
                true => (
                    "COUNT(*) AS total",
                    "",
                    "",
                ),
                false => (
                    "items.id, items.name, items.price, items.stock",
                    "JOIN categories ON categories.id = items.category_id",
                    (
                        "ORDER BY items.created_at DESC LIMIT :limit OFFSET :start",
                        ( :start = 0i64, :limit = 50i64 ),
                    ),
                ),
            },
        )
    );

    let total: AmountResult = group
        .amount
        .fetch_one(&pool)
        .await
        .expect("amount query failed");
    let items: Vec<Item> = group
        .list
        .fetch_all(&pool)
        .await
        .expect("list query failed");

    assert!(
        total.total.unwrap_or(0) >= 3,
        "expected at least 3 items in Electronics with price >= 100"
    );
    assert!(items.len() >= 3);
    assert_eq!(items[0].name, "Monitor");
    assert_eq!(items[1].name, "Headphones");
}

#[tokio::test]
async fn multiple_results_scalar_key() {
    let pool = pool().await;

    let category_id = 2i64;

    let group = sql_forge!(
        (
            >amount = scalar i64,
            >first_name = scalar String,
        ),
        r#"
        SELECT {#fields}
        FROM items
        WHERE items.category_id = :category_id
        "#,
        ( :category_id = category_id ),
        (
            #fields = match {>amount} {
                true => "COUNT(*)",
                false => "items.name",
            },
        )
    );

    let count: i64 = group
        .amount
        .fetch_one(&pool)
        .await
        .expect("count query failed");
    let first_name: String = group
        .first_name
        .fetch_one(&pool)
        .await
        .expect("first_name query failed");

    assert_eq!(count, 1);
    assert_eq!(first_name, "Rust Book");
}

#[allow(clippy::unnecessary_literal_unwrap)]
#[tokio::test]
async fn combining_features_example() {
    let pool = pool().await;

    let category = Some("Electronics");
    let price_min = Some(price_new(5000, 2));
    let price_max: Option<Price> = None;
    let in_stock_only = true;
    let order_by = Some("price_desc".to_string());
    let page: i64 = 0;
    let page_size = Some(10i64);

    let products: Vec<Product> = sql_forge!(
        Product,
        r#"
        SELECT
            p.id,
            p.name,
            p.price,
            p.stock,
            p.category
        FROM products p
        WHERE 1 = 1
        {#filter_category}
        {#filter_price_min}
        {#filter_price_max}
        {#filter_in_stock}
        {#order_by}
        {#limit}
        "#,
        (
            #filter_category = match category.is_some() {
                true => (
                    " AND p.category = :cat ",
                    ( :cat = category.unwrap() ),
                ),
                false => "",
            },
            #filter_price_min = match price_min.is_some() {
                true => (
                    " AND p.price >= :price_min ",
                    ( :price_min = price_min.unwrap() ),
                ),
                false => "",
            },
            #filter_price_max = match price_max.is_some() {
                true => (
                    " AND p.price <= :price_max ",
                    ( :price_max = price_max.unwrap() ),
                ),
                false => "",
            },
            #filter_in_stock = match in_stock_only {
                true => " AND p.stock > 0 ",
                false => "",
            },
            #order_by = match order_by.as_deref() {
                Some("price_asc") => " ORDER BY p.price ASC ",
                Some("price_desc") => " ORDER BY p.price DESC ",
                _ => " ORDER BY p.id ASC ",
            },
            #limit = match page_size.is_some() {
                true => (
                    " LIMIT :size OFFSET :offset ",
                    ( :offset = page * page_size.unwrap(), :size = page_size.unwrap() ),
                ),
                false => "",
            },
        )
    )
    .fetch_all(&pool)
    .await
    .expect("combining features query failed");

    assert!(!products.is_empty(), "expected at least one product");
    for p in &products {
        assert_eq!(p.category, "Electronics");
        assert!(p.price >= price_new(50, 0), "price should be >= 50");
        assert!(p.stock > 0, "stock should be > 0");
    }
    assert_eq!(
        products.first().map(|p| p.name.as_str()),
        Some("Tablet"),
        "expected price_desc order: Tablet (800.00) should be first"
    );
}

#[tokio::test]
async fn execute_only_query() {
    let pool = pool().await;

    sql_forge!(
        "UPDATE products SET stock = 50 WHERE id = :id",
        ( :id = 1i64 ),
    )
    .execute(&pool)
    .await
    .expect("reset stock failed");

    sql_forge!(
        r#"
        UPDATE products SET stock = stock + 1 WHERE id = :id
        "#,
        ( :id = 1i64 ),
    )
    .execute(&pool)
    .await
    .expect("execute-only query failed");

    let row: (i64,) = sqlx::query_as::<_, (i64,)>("SELECT stock FROM products WHERE id = 1")
        .fetch_one(&pool)
        .await
        .expect("readback failed");
    assert_eq!(
        row.0, 51,
        "stock should have been incremented from 50 to 51"
    );
}

#[tokio::test]
async fn execute_only_insert_update_delete() {
    let pool = pool().await;

    sql_forge!(
        "DELETE FROM products WHERE category = :category",
        ( :category = "Temporary" ),
    )
    .execute(&pool)
    .await
    .ok();

    let names = ["Temp A", "Temp B", "Temp C"];
    let base_price = price_new(9999, 2);

    for (i, name) in names.iter().enumerate() {
        sql_forge!(
            r#"
            INSERT INTO products (name, price, stock, category)
            VALUES (:name, :price, :stock, :category)
            "#,
            (
                :name = name,
                :price = price_inc(&base_price, i as i64, 2),
                :stock = 10i64,
                :category = "Temporary",
            ),
        )
        .execute(&pool)
        .await
        .expect("insert failed");
    }

    sql_forge!(
        r#"
        UPDATE products
        SET price = :new_price
        WHERE category = :category AND name = :name
        "#,
        (
            :new_price = price_new(4999, 2),
            :category = "Temporary",
            :name = "Temp B",
        ),
    )
    .execute(&pool)
    .await
    .expect("update failed");

    #[derive(sqlx::FromRow)]
    struct TempRow {
        #[expect(dead_code)]
        name: String,
        price: Price,
    }

    let rows: Vec<TempRow> = sql_forge!(
        TempRow,
        r#"
        SELECT name, price FROM products
        WHERE category = :cat
        ORDER BY id
        "#,
        ( :cat = "Temporary" ),
    )
    .fetch_all(&pool)
    .await
    .expect("select after update failed");

    assert_eq!(rows.len(), 3);
    assert_eq!(rows[0].price, price_new(9999, 2));
    assert_eq!(rows[1].price, price_new(4999, 2));
    assert_eq!(rows[2].price, price_new(10001, 2));

    sql_forge!(
        r#"
        DELETE FROM products
        WHERE category = :category
        "#,
        ( :category = "Temporary" ),
    )
    .execute(&pool)
    .await
    .expect("delete failed");

    let remaining: i64 = sql_forge!(
        i64,
        "SELECT COUNT(*) FROM products WHERE category = :cat",
        ( :cat = "Temporary" ),
    )
    .fetch_one(&pool)
    .await
    .expect("count after delete failed");

    assert_eq!(
        remaining, 0,
        "all temporary products should have been deleted"
    );
}

#[derive(sqlx::FromRow)]
struct BatchItem {
    name: String,
    price: Price,
}

#[tokio::test]
async fn execute_batch() {
    let pool = pool().await;

    sql_forge!(
        "DELETE FROM products WHERE category = :category",
        ( :category = "Batch" ),
    )
    .execute(&pool)
    .await
    .ok();

    let items = vec![
        BatchItem {
            name: "Batch A".to_string(),
            price: price_new(9999, 2),
        },
        BatchItem {
            name: "Batch B".to_string(),
            price: price_new(4999, 2),
        },
        BatchItem {
            name: "Batch C".to_string(),
            price: price_new(10001, 2),
        },
    ];

    sql_forge!(
        r#"
        INSERT INTO products (name, price, stock, category)
        VALUES {(:name, :price, 10, 'Batch')}
        "#,
        ..items
    )
    .execute(&pool)
    .await
    .expect("batch insert failed");

    let rows: Vec<BatchItem> = sql_forge!(
        BatchItem,
        r#"
        SELECT name, price FROM products
        WHERE category = :cat
        ORDER BY id
        "#,
        ( :cat = "Batch" ),
    )
    .fetch_all(&pool)
    .await
    .expect("select batch failed");

    assert_eq!(rows.len(), 3);
    assert_eq!(rows[0].name, "Batch A");
    assert_eq!(rows[0].price, price_new(9999, 2));
    assert_eq!(rows[1].name, "Batch B");
    assert_eq!(rows[1].price, price_new(4999, 2));
    assert_eq!(rows[2].name, "Batch C");
    assert_eq!(rows[2].price, price_new(10001, 2));

    sql_forge!(
        "DELETE FROM products WHERE category = :category",
        ( :category = "Batch" ),
    )
    .execute(&pool)
    .await
    .expect("delete batch failed");
}

#[tokio::test]
async fn execute_batch_with_params() {
    let pool = pool().await;

    sql_forge!(
        "DELETE FROM products WHERE category = :category",
        ( :category = "BatchWithParams" ),
    )
    .execute(&pool)
    .await
    .ok();

    let batch_items = vec![
        // BatchItem {
        //     name: "Batch A".to_string(),
        //     price: price_new(9999, 2),
        // },
        BatchItem {
            name: "Batch B".to_string(),
            price: price_new(4999, 2),
        },
        BatchItem {
            name: "Batch C".to_string(),
            price: price_new(10001, 2),
        },
    ];

    sql_forge!(
        r#"
        INSERT INTO products (name, price, stock, category)
        VALUES (:name, :price, 10, 'BatchWithParams'), {(:name, :price, 10, 'BatchWithParams')}
        "#,
        ( :name = "Batch A Param".to_string(), :price = price_new(8999, 2) ),
        ..batch_items
    )
    .execute(&pool)
    .await
    .expect("batch insert failed");

    let rows: Vec<BatchItem> = sql_forge!(
        BatchItem,
        r#"
        SELECT name, price FROM products
        WHERE category = :cat
        ORDER BY id
        "#,
        ( :cat = "BatchWithParams" ),
    )
    .fetch_all(&pool)
    .await
    .expect("select batch failed");

    assert_eq!(rows.len(), 3);
    assert_eq!(rows[0].name, "Batch A Param");
    assert_eq!(rows[0].price, price_new(8999, 2));
    assert_eq!(rows[1].name, "Batch B");
    assert_eq!(rows[1].price, price_new(4999, 2));
    assert_eq!(rows[2].name, "Batch C");
    assert_eq!(rows[2].price, price_new(10001, 2));

    sql_forge!(
        "DELETE FROM products WHERE category = :category",
        ( :category = "BatchWithParams" ),
    )
    .execute(&pool)
    .await
    .expect("delete batch failed");
}

#[derive(sqlx::FromRow)]
struct BatchFullItem {
    name: String,
    price: Price,
    stock: i64,
    category: String,
}

#[tokio::test]
async fn execute_batch_full() {
    let pool = pool().await;

    sql_forge!(
        "DELETE FROM products WHERE category = :category",
        ( :category = "BatchFull" ),
    )
    .execute(&pool)
    .await
    .ok();

    let items = vec![
        BatchFullItem {
            name: "Batch A".to_string(),
            price: price_new(9999, 2),
            stock: 10i64,
            category: "BatchFull".to_string(),
        },
        BatchFullItem {
            name: "Batch B".to_string(),
            price: price_new(4999, 2),
            stock: 10i64,
            category: "BatchFull".to_string(),
        },
    ];

    sql_forge!(
        r#"
        INSERT INTO products (name, price, stock, category)
        VALUES {(:name, :price, :stock, :category)}
        "#,
        ..items
    )
    .execute(&pool)
    .await
    .expect("batch insert failed");

    let rows: Vec<BatchFullItem> = sql_forge!(
        BatchFullItem,
        r#"
        SELECT name, price, stock, category FROM products
        WHERE category = :cat
        ORDER BY id
        "#,
        ( :cat = "BatchFull" ),
    )
    .fetch_all(&pool)
    .await
    .expect("select batch full failed");

    assert_eq!(rows.len(), 2);
    assert_eq!(rows[0].name, "Batch A");
    assert_eq!(rows[0].price, price_new(9999, 2));
    assert_eq!(rows[0].stock, 10i64);
    assert_eq!(rows[0].category, "BatchFull");
    assert_eq!(rows[1].name, "Batch B");
    assert_eq!(rows[1].price, price_new(4999, 2));
    assert_eq!(rows[1].stock, 10i64);
    assert_eq!(rows[1].category, "BatchFull");

    sql_forge!(
        "DELETE FROM products WHERE category = :category",
        ( :category = "BatchFull" ),
    )
    .execute(&pool)
    .await
    .expect("delete batch full failed");
}

#[tokio::test]
async fn section_match_bound_variable_no_warning() {
    let pool = pool().await;
    let max_price = Some(price_new(15000, 2));

    let products: Vec<Product> = sql_forge!(
        Product,
        "SELECT id, name, price, stock, category FROM products WHERE 1=1 {#filter_price} ORDER BY id",
        (
            #filter_price = match max_price {
                Some(max_price) => (
                    " AND price <= :max_price",
                    ( :max_price = max_price ),
                ),
                None => "",
            },
        )
    )
    .fetch_all(&pool)
    .await
    .expect("section match pattern query failed");

    for p in &products {
        assert!(p.price <= 15000);
    }
}

#[tokio::test]
async fn section_nested_match_outer_var_used() {
    let pool = pool().await;

    let limit_val = Some(3i64);
    let start_val = Some(0i64);

    let users: Vec<User> = sql_forge!(
        User,
        "SELECT id, name FROM users WHERE 1=1 ORDER BY id {#limit}",
        (
            #limit = match limit_val {
                Some(limit) => match start_val {
                    Some(start) => (
                        " LIMIT :limit OFFSET :start ",
                        ( :start = start, :limit = limit ),
                    ),
                    None => (
                        " LIMIT :limit ",
                        ( :limit = limit ),
                    ),
                },
                None => "",
            },
        )
    )
    .fetch_all(&pool)
    .await
    .expect("nested match query failed");

    assert!(!users.is_empty());
    assert!(users.len() <= 3);
    for (i, user) in users.iter().enumerate() {
        assert!(user.id >= i as i64);
    }
}

#[cfg(sql_forge_db_mysql)]
#[tokio::test]
async fn execute_only_with_explicit_mysql_db() {
    let pool = pool().await;

    sql_forge!(sqlx::MySql, "SELECT 1",)
        .execute(&pool)
        .await
        .expect("execute-only with explicit MySql db failed");
}

#[cfg(sql_forge_db_postgres)]
#[tokio::test]
async fn execute_only_with_explicit_postgres_db() {
    let pool = pool().await;

    sql_forge!(sqlx::Postgres, "SELECT 1",)
        .execute(&pool)
        .await
        .expect("execute-only with explicit Postgres db failed");
}

#[cfg(sql_forge_db_sqlite)]
#[tokio::test]
async fn execute_only_with_explicit_sqlite_db() {
    let pool = pool().await;

    sql_forge!(sqlx::Sqlite, "SELECT 1",)
        .execute(&pool)
        .await
        .expect("execute-only with explicit Sqlite db failed");
}

#[test]
fn compile_fail() {
    let db_type = std::env::var("ENV_DB_TYPE").expect("ENV_DB_TYPE not defined");
    let pattern = format!("tests/{db_type}/tmp-ui/*.rs");
    let tests = trybuild::TestCases::new();
    tests.compile_fail(&pattern);
}

#[test]
fn compile_fail_specific() {
    let db_type = std::env::var("ENV_DB_TYPE").expect("ENV_DB_TYPE not defined");
    let pattern = format!("tests/{db_type}/ui/*.rs");
    let tests = trybuild::TestCases::new();
    tests.compile_fail(&pattern);
}
