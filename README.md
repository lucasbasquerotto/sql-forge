# sql_forge

A proc-macro that combines **compile-time SQL validation** (via `sqlx::query_as!` / `sqlx::query_scalar!`) with a **runtime `QueryBuilder`** for dynamic queries.

Write SQL with named parameters and optional sections that are swapped in at runtime, while still getting sqlx's full type-checking at compile time.

---

## Installation

```toml
[dependencies]
sql-forge = "0.1"
sqlx = { version = "0.8", features = ["mysql", "runtime-tokio"] }  # or postgres / sqlite
```

Import the macro:

```rust
use sql_forge::sql_forge;
```

---

## Quick start

```rust
let users: Vec<User> = sql_forge!(
    User,
    "SELECT id, name FROM users WHERE id <= :max_id LIMIT :limit",
    ( :max_id = max_id, :limit = limit )
)
.fetch_all(&pool)
.await?;
```

---

## Full syntax

### Single result

```
sql_forge!(
    [DB,]           // optional (sqlx::MySql | sqlx::Postgres | sqlx::Sqlite)
    Model,          // return type
    SQL,            // string literal
    [params,]       // optional parameter source
    [(sections)]    // optional section map
)
```

### Multiple results

```
sql_forge!(
    [DB,]                                 // optional database type
    ( >key1 = ModelA, >key2 = ModelB ),   // result map
    SQL,                                  // string literal
    [params,]                             // optional parameter source
    [(sections)]                          // section map with {>key} matching
)
```

---

## Specifying the database type

The DB type can be given explicitly as the first argument, via the `SQL_FORGE_DB_TYPE` environment variable, or inferred from `Cargo.toml` metadata (lowest priority).

### Explicit

```rust
sql_forge!(sqlx::MySql, User, "SELECT ...", ...)
```

### Via environment variable (project-wide default)

```sh
SQL_FORGE_DB_TYPE=sqlx::MySql cargo build
```

### Via Cargo.toml (fallback)

```toml
[package.metadata.sql_forge]
db = "sqlx::MySql"
```

When any of the above is configured, the first macro argument may be the model type directly instead of the DB type.

---

## Parameters

### Named parameters (`:name`)

Placeholders are written as `:name` inside the SQL. Each `:name` is replaced by a `push_bind` call at runtime and by a `?` placeholder for compile-time validation.

### Inline map

Bind individual expressions with `( :name = expr, ... )`:

```rust
sql_forge!(
    User,
    "SELECT * FROM users WHERE id <= :max_id LIMIT :start, :limit",
    ( :max_id = filter.max_id, :start = 0u64, :limit = 100u64 )
)
```

### Struct source

Pass any struct (or local variable) whose fields match the parameter names:

```rust
struct Filter { max_id: u64, start: u64, limit: u64 }

sql_forge!(
    User,
    "SELECT * FROM users WHERE id <= :max_id LIMIT :start, :limit",
    filter   // fields max_id, start, limit are read automatically
)
```

---

## Sections (`{#name}`)

Sections are named placeholders in the SQL that are filled at runtime with a string and optional parameters.

```rust
sql_forge!(
    User,
    "SELECT * FROM users {#filter}",
    ( #filter = "WHERE id = 10" )
)
```

### Dynamic section with `match`

```rust
sql_forge!(
    User,
    "SELECT * FROM users {#join_org}",
    (
        #join_org = match include_org {
            true  => " JOIN organisations o ON o.id = users.org_id ",
            false => "",
        }
    )
)
```

### Section with local parameters

A section value can be a tuple `("sql", params)` to bind parameters that are only relevant to that section:

```rust
sql_forge!(
    User,
    "SELECT * FROM users {#filter}",
    (
        #filter = (
            " WHERE id <= :max_id AND status = :status ",
            ( :max_id = max_id, :status = "active" ),
        )
    )
)
```

The parameter source can also be a struct:

```rust
sql_forge!(
    User,
    "SELECT * FROM users {#filter}",
    (
        #filter = (
            " WHERE id <= :max_id ",
            filter_struct,   // field max_id is read from this struct
        )
    )
)
```

### Grouped sections

Multiple section placeholders can be driven by the same `match` expression. Declare them as `#(a, b)` and each arm must return a tuple with the same number of items:

```rust
sql_forge!(
    User,
    "SELECT * FROM users {#join_org} {#filter_org}",
    (
        #(join_org, filter_org) = match include_org {
            true => (
                " JOIN organisations o ON o.id = users.org_id ",
                (
                    " AND o.active = :active ",
                    ( :active = true ),
                ),
            ),
            false => ("", ""),
        }
    )
)
```

---

## Scalar output

When the model type is a Rust primitive (`i8`, `i16`, `i32`, `i64`, `u8` … `u64`, `f32`, `f64`, `bool`, `String`), `sql_forge!` automatically uses `query_scalar!` for validation and `build_query_scalar` for execution:

```rust
let count: i64 = sql_forge!(
    i64,
    "SELECT COUNT(*) FROM users WHERE active = 1",
)
.fetch_one(&pool)
.await?;
```

For non-primitive types that SQLx can treat as scalars (e.g. tuple structs with a single field annotated with `#[sqlx(transparent)]`, like wrapped IDs), use the `scalar` keyword before the model name:

```rust
let user_id: UserId = sql_forge!(
    scalar UserId,
    "SELECT id FROM users WHERE email = :email",
    ( :email = "user@example.com" ),
)
.fetch_one(&pool)
.await?;
```

The `scalar` keyword is only required for non-primitive scalar types.

---

## `IN (...)` with a list parameter

Append `[]` to the placeholder name in the SQL to mark it as a list parameter:

```rust
let ids = vec![1i32, 2, 3];

let users: Vec<User> = sql_forge!(
    User,
    "SELECT * FROM users WHERE id IN (:ids[])",
    ( :ids = ids )
)
.fetch_all(&pool)
.await?;
```

Without the `[]` suffix, the parameter is treated as a single value.

### Empty lists

`sql_forge!` does not rewrite empty list predicates. If a list parameter used with `[]` is empty, runtime SQL becomes `IN ()` / `NOT IN ()`, and the database will raise a syntax error.

Handle empty lists explicitly in your own logic. For example, return early, or use a section with `match` to emit alternative SQL:

```rust
let users: Vec<User> = sql_forge!(
    User,
    "SELECT id, name FROM users WHERE {#filter}",
    (
        #filter = match ids.is_empty() {
            true  => "1 = 0",
                false => (
                    "id IN (:ids[])",
                    ( :ids = ids ),
                ),
        }
    )
)
.fetch_all(&pool)
.await?;
```

---

## Execution

The macro expands to an `EnhancedQuery` value. Call `.fetch_all(executor)`, `.fetch_one(executor)`, or `.fetch_optional(executor)` directly on it:

```rust
let result = sql_forge!(User, "...", ...)
    .fetch_all(&pool)
    .await?;
```

You can also store the query and execute it later:

```rust
let query = sql_forge!(User, "...", ...);
// ... some code ...
let result = query.fetch_one(&pool).await?;
```

### Returning a query from a function

Use `impl EnhancedQuery<Model>` as the return type to build a query in one place and execute it elsewhere. The macro return type is unnameable, so `impl Trait` is the only option.

```rust
use sql_forge::{sql_forge, db_type, EnhancedQuery};

pub type AppDb = db_type!(); // or explicitly sqlx::MySql, sqlx::Postgres, etc.

fn users_by_ids_query(ids: Vec<i32>) -> impl EnhancedQuery<User, Db = AppDb> {
    sql_forge!(
        User,
        "SELECT id, name FROM users WHERE id IN (:ids)",
        ( :ids = ids )
    )
}

// Later, at call site:
let query = users_by_ids_query(vec![1, 2, 3]);
let users = query.fetch_all(&pool).await?;
```

### Execute-only (no model)

When the model type is omitted entirely, the macro produces a value implementing `EnhancedQueryExecute`. Only `.execute(executor)` is available and there is no return type to deserialize into. This is useful for `INSERT`, `UPDATE`, `DELETE`, and other DML statements.

```rust
use sql_forge::EnhancedQueryExecute;

let result = sql_forge!(
    "UPDATE products SET stock = stock + 1 WHERE id = :id",
    ( :id = 42i64 ),
)
.execute(&pool)
.await?;
```

Sections work the same way:

```rust
let rows_affected = sql_forge!(
    "UPDATE products SET price = :new_price {#filter}",
    ( :new_price = 10.0 ),
    (
        #filter = (
            " WHERE category = :cat ",
            ( :cat = "Electronics" ),
        ),
    ),
)
.execute(&pool)
.await?;
```

The parameter source can also be a struct:

```rust
struct UpdateFilter { id: i64, new_price: f64 }

let filter = UpdateFilter { id: 1, new_price: 15.0 };

let result = sql_forge!(
    "UPDATE products SET price = :new_price WHERE id = :id",
    filter,
)
.execute(&pool)
.await?;
```

---

## Multiple results (`EnhancedQueryGroup`)

A single SQL template can produce **multiple independent queries** that share the same base SQL, parameters, and dynamic sections. This avoids duplicating the query structure when you need different result shapes (e.g., a count and a paginated list) from the same filters.

### Result map syntax

Replace the single model type with a map of `>key = Type` entries:

```rust
sql_forge!(
    (
        >amount = ModelAmount,
        >list   = ModelItem,
    ),
    "SELECT {#fields} FROM items {#joins} WHERE ...",
    ...
)
```

Each key becomes a field on the returned group object, holding its own `EnhancedQuery<Type, Db = DB>`.

### Conditional sections with `{>key}`

In the section map, the special expression `{>key_name}` evaluates to `true` when generating the query for that specific key, and `false` otherwise. Use it with `match` to select different SQL per result:

```rust
(
    #(fields, joins) = match {>amount} {
        true => (
            "COUNT(*) AS total",
            "",
        ),
        false => (
            "items.id, items.name, items.price",
            "JOIN categories ON categories.id = items.category_id",
        ),
    },
)
```

### Complete example

```rust
use sql_forge::{sql_forge, EnhancedQuery, EnhancedQueryGroup, EnhancedQueryGroupGet};
use crate::models::general::ListAndAmount;

pub type AppDb = sqlx::MySql;

struct AmountResult {
    total: i64,
}

fn build_item_query(
    category_id: i32,
    min_price: f64,
) -> ListAndAmount<
    impl EnhancedQuery<AmountResult, Db = AppDb>,
    impl EnhancedQuery<Item, Db = AppDb>,
> {
    let group = sql_forge!(
        (
            >amount = AmountResult,
            >list   = Item,
        ),
        r#"-- sql
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
                    r#"-- sql
                    COUNT(*) AS total
                    "#,
                    "",
                    "",
                ),
                false => (
                    r#"-- sql
                    items.id,
                    items.name,
                    items.price,
                    items.stock
                    "#,
                    "JOIN categories ON categories.id = items.category_id",
                    (
                        r#"-- sql
                        ORDER BY items.created_at DESC
                        LIMIT :start, :limit
                        "#,
                        (
                            :start = 0i64,
                            :limit = 50i64,
                        ),
                    ),
                ),
            },
        )
    );

    ListAndAmount {
        amount: group.amount,
        list: group.list,
    }
}
```

### Accessing individual queries

The group struct exposes each key as a field. Pass the field to any SQLx executor method:

```rust
let q = build_item_query(10, 5.0);

// Execute the count query
let total: AmountResult = q.amount.fetch_one(&pool).await?;

// Execute the list query
let items: Vec<Item> = q.list.fetch_all(&pool).await?;
```

### Scalar keys

Keys can also use `scalar Type` for primitive/scalar output:

```rust
sql_forge!(
    (
        >amount = scalar i64,
        >list   = Item,
    ),
    "SELECT {#fields} FROM items WHERE ...",
    ...
)
```

When the key type is marked as `scalar`, the macro generates a `query_scalar!` validator instead of `query_as!`. This is equivalent to using a standalone `scalar i64` in a single-result query.

### Return type pattern

The typical return type for a function producing two queries uses `ListAndAmount<A, L>` (or a custom struct):

```rust
use crate::models::general::ListAndAmount;

fn query_items(...)
    -> ListAndAmount<
        impl EnhancedQuery<AmountResult, Db = AppDb>,
        impl EnhancedQuery<Item, Db = AppDb>,
    >
{ ... }
```

The group struct (`__EnhancedQueryGroup`) generated by the macro is unnameable, so a wrapper like `ListAndAmount` is the recommended way to return individual queries from a function. You can also call `.into_parts()` on the group result to destructure it into a tuple of the individual queries.

---

## Compile-time validation

Under the hood the macro emits a never-called closure containing one or more `sqlx::query_as!` / `sqlx::query_scalar!` invocations. Rather than generating the full cartesian product of all section variants (which would explode for many sections), it uses a **smart cycling strategy**:

- Let each section have _m_ possible variants (match arms).
- Find _n_max_, the largest _m_ across all sections.
- Generate exactly _n_max_ validator queries.
- Query _i_ (0-based) uses variant `i % m` for each section.

For example, with two sections having 3 and 10 variants respectively, 10 validator queries are generated, instead of 30; the first section cycles `(0, 1, 2, 0, 1, 2, 0, 1, 2, 0)` while the second uses each of its 10 variants once. This ensures every variant of the widest section appears in at least one validator query, and every other section is exercised as many times as its own variant count allows, without combinatorial growth.

List parameters are validated using index access to the first element (`.as_slice()[0]`). The validator closure is never called at runtime, it exists solely to drive `query_as!`/`query_scalar!` compile-time type checking. This means that `IN (:list[])` would be validated as `IN (?)` using the first list element in a closure that is never called, used only for compile-time validation (the runtime query will use the full list with a QueryBuilder `push_bind`).

---

## Combining features

This example uses the `WHERE 1 = 1` idiom so that every optional filter can start with `AND` without needing to track whether it is the first condition. Modern database engines (MySQL, PostgreSQL, SQLite) constant-fold `1 = 1` away at planning time, so there is no runtime performance cost.

```rust
let rows: Vec<Product> = sql_forge!(
    sqlx::MySql,
    Product,
    r#"
        SELECT
            p.id,
            p.name,
            p.price,
            IF(p.stock > 0, 1, 0) AS in_stock,
            IF(p.stock >= :overflow, 1, 0) AS overflow,
            p.stock
        FROM products p
        WHERE 1 = 1
        {#filter_category}
        {#filter_price_min}
        {#filter_price_max}
        {#filter_in_stock}
        {#order_by}
        {#limit}
    "#,
    ( :overflow = 1000 ),
    (
        #filter_category = match &category {
            Some(cat) => ( " AND p.category = :cat ", ( :cat = cat ) ),
            None      => "",
        },
        #filter_price_min = match price_min {
            Some(min) => ( " AND p.price >= :price_min ", ( :price_min = min ) ),
            None      => "",
        },
        #filter_price_max = match price_max {
            Some(max) => ( " AND p.price <= :price_max ", ( :price_max = max ) ),
            None      => "",
        },
        #filter_in_stock = match in_stock_only {
            true  => " AND p.stock > 0 ",
            false => "",
        },
        #order_by = match order_by.as_deref() {
            Some("price_asc")  => " ORDER BY p.price ASC ",
            Some("price_desc") => " ORDER BY p.price DESC ",
            _                  => " ORDER BY p.id ASC ",
        },
        #limit = match page_size {
            Some(size) => ( " LIMIT :offset, :size ", ( :offset = page * size, :size = size ) ),
            None       => "",
        },
    )
)
.fetch_all(&pool)
.await?;
```

---

## Caveats

### String literals containing `:`

The macro scans the SQL template text for `:parameter` tokens to locate bind parameter placeholders.
A colon that appears **inside a SQL string literal** in the template (e.g. `"abc:def"`) is
indistinguishable from a parameter placeholder at the text level and will cause a parse error
or an unexpected parameter name.

**Workaround:** pass the value as a bind parameter and use the `:parameter` placeholder in the
template instead of embedding the literal directly.

| ❌ Inline literal (breaks)                 | ✅ Bind parameter (correct)                       |
| ------------------------------------------ | ------------------------------------------------- |
| `WHERE name = "abc:def"` in the SQL string | `WHERE name = :name` with `( :name = "abc:def" )` |

```rust
// ❌ will fail, as the macro sees ":def" as a parameter placeholder
// sql_forge!(User, r#"SELECT ... WHERE name = "abc:def""#);

// ✅ bind the value that contains ":" as a parameter
let _query = sql_forge!(
    User,
    r#"SELECT id, name FROM users WHERE name = :name"#,
    ( :name = "abc:def" )
);
```

---

### String literals containing `{#`

The macro also scans the SQL template text for `{#section}` tokens to locate dynamic section slots.
A `{#` sequence that appears **inside a SQL string literal** in the template (e.g. `"abc{#def"`)
will be treated as a section slot, causing a parse error or a missing-section error.

**Workaround:** pass the value as a bind parameter and use the `:parameter` placeholder in the
template instead.

| ❌ Inline literal (breaks)                  | ✅ Bind parameter (correct)                        |
| ------------------------------------------- | -------------------------------------------------- |
| `WHERE name = "abc{#def"` in the SQL string | `WHERE name = :name` with `( :name = "abc{#def" )` |

```rust
// ❌ will fail, as the macro sees "{#def" as a section slot
// sql_forge!(User, r#"SELECT ... WHERE name = "abc{#def""#);

// ✅ bind the value that contains "{#" as a parameter
let _query = sql_forge!(
    User,
    r#"SELECT id, name FROM users WHERE name = :name"#,
    ( :name = "abc{#def" )
);
```

---

### Incomplete cross-section validation

The validator intentionally does **not** expand the full cartesian product of section variants, because doing so grows exponentially and becomes impractical for real queries. Instead, it uses the cycling strategy described above.

That tradeoff leads to the first class of behavior below. The second class is an inherent property of compile-time validation and occurs independently of cycling:

1. **Missed required validation (cycling limitation):**
   A problematic combination can be skipped by the cycle, so code that should fail may compile.
2. **Conservative rejection (always present):**
   A query can be flagged even when runtime control flow would make it safe, because compile-time validation does not reason about runtime conditions. This happens with or without cycling.

In both situations, the recommended fix is the same: **group dependent sections** under one `match` using `#(a, b, ...)` so related SQL fragments always move together.

Even when a non-grouped version happens to work today, grouping is safer and less fragile under future maintenance (similar to choosing stricter safety constraints that prevent subtle bugs).

**Case 1: may compile, but should fail (cycling can miss it)**

```rust
// ⚠ Not recommended: dependent sections are independent.
// With 2x2 variants, cycling checks (0,0) and (1,1), so (0,1) may be skipped.
sql_forge!(
    Row,
    r#"
        SELECT t1.id AS field_1, {#field_2}
        FROM t1
        {#join_t2}
        WHERE 1 = 1
    "#,
    (
        #join_t2 = match include_t2 {
            true => " JOIN t2 ON t2.t1_id = t1.id ",
            false => "",
        },
        #field_2 = match needs_t2_field {
            true => "t2.name AS field_2", // requires join_t2 = true
            false => "t1.name AS field_2",
        },
    )
);
```

**Recommended grouped version for Case 1**

```rust
sql_forge!(
    Row,
    r#"
        SELECT t1.id AS field_1, {#field_2}
        FROM t1
        {#join_t2}
        WHERE 1 = 1
    "#,
    (
        #(join_t2, field_2) = match include_t2 {
            true => (
                " JOIN t2 ON t2.t1_id = t1.id ",
                "t2.name AS field_2",
            ),
            false => ("", "t1.name AS field_2"),
        }
    )
);
```

**Case 2: may be rejected, even though runtime logic would work (conservative validation)**

```rust
// ⚠ Not recommended: runtime implies safety, but compile-time still checks all variants.
sql_forge!(
    Row,
    r#"
        SELECT t1.id AS field_1, {#field_2}
        FROM t1
        {#join_t2}
        WHERE 1 = 1
    "#,
    (
        #join_t2 = match include_t2 {
            false => "",
            true => " JOIN t2 ON t2.t1_id = t1.id ",
        },
        #field_2 = match include_t2 && can_read_t2 {
            true => "t2.secret AS field_2", // only reachable when include_t2 = true
            false => "t1.name AS field_2",
        },
    )
);
```

**Recommended grouped version for Case 2**

```rust
sql_forge!(
    Row,
    r#"
        SELECT t1.id AS field_1, {#field_2}
        FROM t1
        {#join_t2}
        WHERE 1 = 1
    "#,
    (
        #(join_t2, field_2) = match include_t2 {
            true => (
                " JOIN t2 ON t2.t1_id = t1.id ",
                match can_read_t2 {
                    true => "t2.secret AS field_2",
                    false => "t1.name AS field_2",
                },
            ),
            false => ("", "t1.name AS field_2"),
        }
    )
);
```

Grouping keeps related fragments synchronized, avoids skipped problematic combinations, and reduces fragile query shapes during maintenance.
