#![feature(prelude_import)]
extern crate std;
#[prelude_import]
use std::prelude::rust_2021::*;
use sql_forge::db_type;
use sql_forge::sql_forge;
use std::any::TypeId;
pub type AppDb = sqlx::Sqlite;
pub type DbPool = sqlx::Pool<AppDb>;
type Price = i64;
#[sqlx(transparent)]
struct UserId(pub i64);
#[automatically_derived]
impl<'q, DB: ::sqlx::Database> ::sqlx::encode::Encode<'q, DB> for UserId
where
    i64: ::sqlx::encode::Encode<'q, DB>,
{
    fn encode_by_ref(
        &self,
        buf: &mut <DB as ::sqlx::database::Database>::ArgumentBuffer,
    ) -> ::std::result::Result<::sqlx::encode::IsNull, ::sqlx::error::BoxDynError> {
        <i64 as ::sqlx::encode::Encode<'q, DB>>::encode_by_ref(&self.0, buf)
    }
    fn produces(&self) -> Option<DB::TypeInfo> {
        <i64 as ::sqlx::encode::Encode<'q, DB>>::produces(&self.0)
    }
    fn size_hint(&self) -> usize {
        <i64 as ::sqlx::encode::Encode<'q, DB>>::size_hint(&self.0)
    }
}
#[automatically_derived]
impl<'r, DB: ::sqlx::Database> ::sqlx::decode::Decode<'r, DB> for UserId
where
    i64: ::sqlx::decode::Decode<'r, DB>,
{
    fn decode(
        value: <DB as ::sqlx::database::Database>::ValueRef<'r>,
    ) -> ::std::result::Result<
        Self,
        ::std::boxed::Box<
            dyn ::std::error::Error + 'static + ::std::marker::Send + ::std::marker::Sync,
        >,
    > {
        <i64 as ::sqlx::decode::Decode<'r, DB>>::decode(value).map(|val| Self { 0: val })
    }
}
#[automatically_derived]
impl<DB: ::sqlx::Database> ::sqlx::Type<DB> for UserId
where
    i64: ::sqlx::Type<DB>,
{
    fn type_info() -> DB::TypeInfo {
        <i64 as ::sqlx::Type<DB>>::type_info()
    }
    fn compatible(ty: &DB::TypeInfo) -> ::std::primitive::bool {
        <i64 as ::sqlx::Type<DB>>::compatible(ty)
    }
}
#[automatically_derived]
impl ::sqlx::postgres::PgHasArrayType for UserId
where
    i64: ::sqlx::postgres::PgHasArrayType,
{
    fn array_type_info() -> ::sqlx::postgres::PgTypeInfo {
        <i64 as ::sqlx::postgres::PgHasArrayType>::array_type_info()
    }
}
impl sql_forge::SqlForgeValidatorValue<i64> for UserId {
    fn sql_forge_validator_value(&self) -> i64 {
        self.0.clone()
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for UserId {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_tuple_field1_finish(f, "UserId", &&self.0)
    }
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for UserId {}
#[automatically_derived]
impl ::core::cmp::PartialEq for UserId {
    #[inline]
    fn eq(&self, other: &UserId) -> bool {
        self.0 == other.0
    }
}
#[automatically_derived]
impl ::core::cmp::Eq for UserId {
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_fields_are_eq(&self) {
        let _: ::core::cmp::AssertParamIsEq<i64>;
    }
}
fn price_new(v: i64, scale: u32) -> Price {
    v * 10i64.pow(2 - scale)
}
fn price_inc(base: &Price, v: i64, scale: u32) -> Price {
    *base + price_new(v, scale)
}
struct User {
    id: i64,
    name: String,
}
#[automatically_derived]
impl<'a, R: ::sqlx::Row> ::sqlx::FromRow<'a, R> for User
where
    &'a ::std::primitive::str: ::sqlx::ColumnIndex<R>,
    i64: ::sqlx::decode::Decode<'a, R::Database>,
    i64: ::sqlx::types::Type<R::Database>,
    String: ::sqlx::decode::Decode<'a, R::Database>,
    String: ::sqlx::types::Type<R::Database>,
{
    fn from_row(__row: &'a R) -> ::sqlx::Result<Self> {
        let id: i64 = __row.try_get("id")?;
        let name: String = __row.try_get("name")?;
        ::std::result::Result::Ok(User { id, name })
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for User {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field2_finish(
            f,
            "User",
            "id",
            &self.id,
            "name",
            &&self.name,
        )
    }
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for User {}
#[automatically_derived]
impl ::core::cmp::PartialEq for User {
    #[inline]
    fn eq(&self, other: &User) -> bool {
        self.id == other.id && self.name == other.name
    }
}
struct Product {
    id: i64,
    name: String,
    price: Price,
    stock: i64,
    category: String,
}
#[automatically_derived]
impl<'a, R: ::sqlx::Row> ::sqlx::FromRow<'a, R> for Product
where
    &'a ::std::primitive::str: ::sqlx::ColumnIndex<R>,
    i64: ::sqlx::decode::Decode<'a, R::Database>,
    i64: ::sqlx::types::Type<R::Database>,
    String: ::sqlx::decode::Decode<'a, R::Database>,
    String: ::sqlx::types::Type<R::Database>,
    Price: ::sqlx::decode::Decode<'a, R::Database>,
    Price: ::sqlx::types::Type<R::Database>,
    i64: ::sqlx::decode::Decode<'a, R::Database>,
    i64: ::sqlx::types::Type<R::Database>,
    String: ::sqlx::decode::Decode<'a, R::Database>,
    String: ::sqlx::types::Type<R::Database>,
{
    fn from_row(__row: &'a R) -> ::sqlx::Result<Self> {
        let id: i64 = __row.try_get("id")?;
        let name: String = __row.try_get("name")?;
        let price: Price = __row.try_get("price")?;
        let stock: i64 = __row.try_get("stock")?;
        let category: String = __row.try_get("category")?;
        ::std::result::Result::Ok(Product {
            id,
            name,
            price,
            stock,
            category,
        })
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for Product {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field5_finish(
            f,
            "Product",
            "id",
            &self.id,
            "name",
            &self.name,
            "price",
            &self.price,
            "stock",
            &self.stock,
            "category",
            &&self.category,
        )
    }
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for Product {}
#[automatically_derived]
impl ::core::cmp::PartialEq for Product {
    #[inline]
    fn eq(&self, other: &Product) -> bool {
        self.id == other.id && self.stock == other.stock && self.name == other.name
            && self.price == other.price && self.category == other.category
    }
}
struct Item {
    id: i64,
    name: String,
    price: Price,
    stock: i64,
}
#[automatically_derived]
impl<'a, R: ::sqlx::Row> ::sqlx::FromRow<'a, R> for Item
where
    &'a ::std::primitive::str: ::sqlx::ColumnIndex<R>,
    i64: ::sqlx::decode::Decode<'a, R::Database>,
    i64: ::sqlx::types::Type<R::Database>,
    String: ::sqlx::decode::Decode<'a, R::Database>,
    String: ::sqlx::types::Type<R::Database>,
    Price: ::sqlx::decode::Decode<'a, R::Database>,
    Price: ::sqlx::types::Type<R::Database>,
    i64: ::sqlx::decode::Decode<'a, R::Database>,
    i64: ::sqlx::types::Type<R::Database>,
{
    fn from_row(__row: &'a R) -> ::sqlx::Result<Self> {
        let id: i64 = __row.try_get("id")?;
        let name: String = __row.try_get("name")?;
        let price: Price = __row.try_get("price")?;
        let stock: i64 = __row.try_get("stock")?;
        ::std::result::Result::Ok(Item { id, name, price, stock })
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for Item {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field4_finish(
            f,
            "Item",
            "id",
            &self.id,
            "name",
            &self.name,
            "price",
            &self.price,
            "stock",
            &&self.stock,
        )
    }
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for Item {}
#[automatically_derived]
impl ::core::cmp::PartialEq for Item {
    #[inline]
    fn eq(&self, other: &Item) -> bool {
        self.id == other.id && self.stock == other.stock && self.name == other.name
            && self.price == other.price
    }
}
struct AmountResult {
    total: Option<i64>,
}
#[automatically_derived]
impl<'a, R: ::sqlx::Row> ::sqlx::FromRow<'a, R> for AmountResult
where
    &'a ::std::primitive::str: ::sqlx::ColumnIndex<R>,
    Option<i64>: ::sqlx::decode::Decode<'a, R::Database>,
    Option<i64>: ::sqlx::types::Type<R::Database>,
{
    fn from_row(__row: &'a R) -> ::sqlx::Result<Self> {
        let total: Option<i64> = __row.try_get("total")?;
        ::std::result::Result::Ok(AmountResult { total })
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for AmountResult {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field1_finish(
            f,
            "AmountResult",
            "total",
            &&self.total,
        )
    }
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for AmountResult {}
#[automatically_derived]
impl ::core::cmp::PartialEq for AmountResult {
    #[inline]
    fn eq(&self, other: &AmountResult) -> bool {
        self.total == other.total
    }
}
struct Filter {
    max_id: i64,
    limit: i64,
}
fn db_url() -> String {
    std::env::var("DATABASE_URL").expect("DATABASE_URL not defined")
}
extern crate test;
#[rustc_test_marker = "db_type_matches_env_db_type"]
#[doc(hidden)]
pub const db_type_matches_env_db_type: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("db_type_matches_env_db_type"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "tests/tests.rs",
        start_line: 60usize,
        start_col: 4usize,
        end_line: 60usize,
        end_col: 31usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(db_type_matches_env_db_type()),
    ),
};
fn db_type_matches_env_db_type() {
    let env_db_type = std::env::var("ENV_DB_TYPE").expect("ENV_DB_TYPE not defined");
    let expected = match env_db_type.as_str() {
        "mysql" => TypeId::of::<sqlx::MySql>(),
        "postgres" => TypeId::of::<sqlx::Postgres>(),
        "sqlite" => TypeId::of::<sqlx::Sqlite>(),
        other => {
            ::core::panicking::panic_fmt(
                format_args!("unsupported ENV_DB_TYPE: {0}", other),
            );
        }
    };
    match (&TypeId::of::<AppDb>(), &expected) {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(
                    kind,
                    &*left_val,
                    &*right_val,
                    ::core::option::Option::None,
                );
            }
        }
    };
}
async fn pool() -> DbPool {
    sqlx::Pool::<AppDb>::connect(&db_url())
        .await
        .expect("cannot connect to test database")
}
extern crate test;
#[rustc_test_marker = "basic_query_with_inline_params"]
#[doc(hidden)]
pub const basic_query_with_inline_params: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("basic_query_with_inline_params"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "tests/tests.rs",
        start_line: 80usize,
        start_col: 10usize,
        end_line: 80usize,
        end_col: 40usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(basic_query_with_inline_params()),
    ),
};
fn basic_query_with_inline_params() {
    let body = async {
        let pool = pool().await;
        let users: Vec<User> = {
            let _sql_forge_validator = || {
                let __sql_forge_top_level_max_id = &(3i64);
                let __sql_forge_top_level_limit = &(10i64);
                {
                    type __SqlForgeModel = User;
                    {
                        let __sql_forge_validator_arg_0 = __sql_forge_top_level_max_id;
                        let __sql_forge_validator_arg_1 = __sql_forge_top_level_max_id;
                        let __sql_forge_validator_arg_2 = __sql_forge_top_level_limit;
                        let _ = {
                            {
                                #[allow(clippy::all)]
                                {
                                    use ::sqlx::Arguments as _;
                                    let arg0 = &(__sql_forge_validator_arg_0);
                                    let arg1 = &(__sql_forge_validator_arg_1);
                                    let arg2 = &(__sql_forge_validator_arg_2);
                                    let mut query_args = <sqlx::sqlite::Sqlite as ::sqlx::database::Database>::Arguments::default();
                                    query_args
                                        .reserve(
                                            3usize,
                                            0
                                                + ::sqlx::encode::Encode::<
                                                    sqlx::sqlite::Sqlite,
                                                >::size_hint(arg0)
                                                + ::sqlx::encode::Encode::<
                                                    sqlx::sqlite::Sqlite,
                                                >::size_hint(arg1)
                                                + ::sqlx::encode::Encode::<
                                                    sqlx::sqlite::Sqlite,
                                                >::size_hint(arg2),
                                        );
                                    let query_args = ::core::result::Result::<
                                        _,
                                        ::sqlx::error::BoxDynError,
                                    >::Ok(query_args)
                                        .and_then(move |mut query_args| {
                                            query_args.add(arg0).map(move |()| query_args)
                                        })
                                        .and_then(move |mut query_args| {
                                            query_args.add(arg1).map(move |()| query_args)
                                        })
                                        .and_then(move |mut query_args| {
                                            query_args.add(arg2).map(move |()| query_args)
                                        });
                                    ::sqlx::__query_with_result::<
                                        sqlx::sqlite::Sqlite,
                                        _,
                                    >(
                                            "SELECT id, name FROM users WHERE id <= ? AND ? >= id LIMIT ?",
                                            query_args,
                                        )
                                        .try_map(|row: sqlx::sqlite::SqliteRow| {
                                            use ::sqlx::Row as _;
                                            #[allow(non_snake_case)]
                                            let sqlx_query_as_id = row
                                                .try_get_unchecked::<i64, _>(0usize)?
                                                .into();
                                            #[allow(non_snake_case)]
                                            let sqlx_query_as_name = row
                                                .try_get_unchecked::<String, _>(1usize)?
                                                .into();
                                            ::std::result::Result::Ok(__SqlForgeModel {
                                                id: sqlx_query_as_id,
                                                name: sqlx_query_as_name,
                                            })
                                        })
                                }
                            }
                        };
                    }
                }
            };
            struct __SqlForgeQuery_single {
                inner: sqlx::QueryBuilder<sqlx::Sqlite>,
            }
            impl __SqlForgeQuery_single {
                async fn fetch_all<'e, E>(
                    mut self,
                    executor: E,
                ) -> Result<Vec<User>, sqlx::Error>
                where
                    E: sqlx::Executor<'e, Database = sqlx::Sqlite>,
                {
                    self.inner.build_query_as::<User>().fetch_all(executor).await
                }
                async fn fetch_one<'e, E>(
                    mut self,
                    executor: E,
                ) -> Result<User, sqlx::Error>
                where
                    E: sqlx::Executor<'e, Database = sqlx::Sqlite>,
                {
                    self.inner.build_query_as::<User>().fetch_one(executor).await
                }
                async fn fetch_optional<'e, E>(
                    mut self,
                    executor: E,
                ) -> Result<Option<User>, sqlx::Error>
                where
                    E: sqlx::Executor<'e, Database = sqlx::Sqlite>,
                {
                    self.inner.build_query_as::<User>().fetch_optional(executor).await
                }
                async fn execute<'e, E>(
                    mut self,
                    executor: E,
                ) -> Result<<sqlx::Sqlite as sqlx::Database>::QueryResult, sqlx::Error>
                where
                    E: sqlx::Executor<'e, Database = sqlx::Sqlite>,
                {
                    self.inner.build().execute(executor).await
                }
            }
            impl sql_forge::SqlForgeQuery<User> for __SqlForgeQuery_single {
                type Db = sqlx::Sqlite;
                fn fetch_all<'e, E>(
                    self,
                    executor: E,
                ) -> impl std::future::Future<
                    Output = Result<Vec<User>, sqlx::Error>,
                > + Send + 'e
                where
                    Self: Sized + 'e,
                    E: sqlx::Executor<'e, Database = sqlx::Sqlite> + Send + 'e,
                    sqlx::Sqlite: 'e,
                {
                    __SqlForgeQuery_single::fetch_all(self, executor)
                }
                fn fetch_one<'e, E>(
                    self,
                    executor: E,
                ) -> impl std::future::Future<
                    Output = Result<User, sqlx::Error>,
                > + Send + 'e
                where
                    Self: Sized + 'e,
                    E: sqlx::Executor<'e, Database = sqlx::Sqlite> + Send + 'e,
                    sqlx::Sqlite: 'e,
                {
                    __SqlForgeQuery_single::fetch_one(self, executor)
                }
                fn fetch_optional<'e, E>(
                    self,
                    executor: E,
                ) -> impl std::future::Future<
                    Output = Result<Option<User>, sqlx::Error>,
                > + Send + 'e
                where
                    Self: Sized + 'e,
                    E: sqlx::Executor<'e, Database = sqlx::Sqlite> + Send + 'e,
                    sqlx::Sqlite: 'e,
                {
                    __SqlForgeQuery_single::fetch_optional(self, executor)
                }
                fn execute<'e, E>(
                    self,
                    executor: E,
                ) -> impl std::future::Future<
                    Output = Result<
                        <sqlx::Sqlite as sqlx::Database>::QueryResult,
                        sqlx::Error,
                    >,
                > + Send + 'e
                where
                    Self: Sized + 'e,
                    E: sqlx::Executor<'e, Database = sqlx::Sqlite> + Send + 'e,
                    sqlx::Sqlite: 'e,
                {
                    __SqlForgeQuery_single::execute(self, executor)
                }
            }
            let __sql_forge_runtime_max_id = 3i64;
            let __sql_forge_runtime_limit = 10i64;
            let mut __builder: sqlx::QueryBuilder<sqlx::Sqlite> = sqlx::QueryBuilder::new(
                "",
            );
            __builder.push("SELECT id, name FROM users WHERE id <= ");
            __builder.push_bind(__sql_forge_runtime_max_id);
            __builder.push(" AND ");
            __builder.push_bind(__sql_forge_runtime_max_id);
            __builder.push(" >= id LIMIT ");
            __builder.push_bind(__sql_forge_runtime_limit);
            let __sql_forge_value_single = __SqlForgeQuery_single {
                inner: __builder,
            };
            __sql_forge_value_single
        }
            .fetch_all(&pool)
            .await
            .expect("basic query failed");
        match (&users.len(), &3) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::None,
                    );
                }
            }
        };
        match (&users[0].name, &"Alice") {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::None,
                    );
                }
            }
        };
        match (&users[1].name, &"Bob") {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::None,
                    );
                }
            }
        };
        match (&users[2].name, &"Charlie") {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::None,
                    );
                }
            }
        };
    };
    let mut body = body;
    #[allow(unused_mut)]
    let mut body = unsafe { ::tokio::macros::support::Pin::new_unchecked(&mut body) };
    let body: ::core::pin::Pin<&mut dyn ::core::future::Future<Output = ()>> = body;
    #[allow(
        clippy::expect_used,
        clippy::diverging_sub_expression,
        clippy::needless_return,
        clippy::unwrap_in_result
    )]
    {
        use tokio::runtime::Builder;
        return Builder::new_current_thread()
            .enable_all()
            .build()
            .expect("Failed building the Runtime")
            .block_on(body);
    }
}
extern crate test;
#[rustc_test_marker = "scalar_output"]
#[doc(hidden)]
pub const scalar_output: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("scalar_output"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "tests/tests.rs",
        start_line: 99usize,
        start_col: 10usize,
        end_line: 99usize,
        end_col: 23usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(scalar_output()),
    ),
};
fn scalar_output() {
    let body = async {
        let pool = pool().await;
        let count: i64 = {
            let _sql_forge_validator = || {
                let __sql_forge_top_level_min_id = &(2i64);
                {
                    {
                        let __sql_forge_validator_arg_0 = __sql_forge_top_level_min_id;
                        let _ = {
                            #[allow(clippy::all)]
                            {
                                use ::sqlx::Arguments as _;
                                let arg0 = &(__sql_forge_validator_arg_0);
                                let mut query_args = <sqlx::sqlite::Sqlite as ::sqlx::database::Database>::Arguments::default();
                                query_args
                                    .reserve(
                                        1usize,
                                        0
                                            + ::sqlx::encode::Encode::<
                                                sqlx::sqlite::Sqlite,
                                            >::size_hint(arg0),
                                    );
                                let query_args = ::core::result::Result::<
                                    _,
                                    ::sqlx::error::BoxDynError,
                                >::Ok(query_args)
                                    .and_then(move |mut query_args| {
                                        query_args.add(arg0).map(move |()| query_args)
                                    });
                                ::sqlx::__query_scalar_with_result::<
                                    sqlx::sqlite::Sqlite,
                                    i64,
                                    _,
                                >("SELECT COUNT(*) FROM users WHERE id > ?", query_args)
                            }
                        };
                    }
                }
            };
            struct __SqlForgeQuery_single {
                inner: sqlx::QueryBuilder<sqlx::Sqlite>,
            }
            impl __SqlForgeQuery_single {
                async fn fetch_all<'e, E>(
                    mut self,
                    executor: E,
                ) -> Result<Vec<i64>, sqlx::Error>
                where
                    E: sqlx::Executor<'e, Database = sqlx::Sqlite>,
                {
                    self.inner.build_query_scalar::<i64>().fetch_all(executor).await
                }
                async fn fetch_one<'e, E>(
                    mut self,
                    executor: E,
                ) -> Result<i64, sqlx::Error>
                where
                    E: sqlx::Executor<'e, Database = sqlx::Sqlite>,
                {
                    self.inner.build_query_scalar::<i64>().fetch_one(executor).await
                }
                async fn fetch_optional<'e, E>(
                    mut self,
                    executor: E,
                ) -> Result<Option<i64>, sqlx::Error>
                where
                    E: sqlx::Executor<'e, Database = sqlx::Sqlite>,
                {
                    self.inner.build_query_scalar::<i64>().fetch_optional(executor).await
                }
                async fn execute<'e, E>(
                    mut self,
                    executor: E,
                ) -> Result<<sqlx::Sqlite as sqlx::Database>::QueryResult, sqlx::Error>
                where
                    E: sqlx::Executor<'e, Database = sqlx::Sqlite>,
                {
                    self.inner.build().execute(executor).await
                }
            }
            impl sql_forge::SqlForgeQuery<i64> for __SqlForgeQuery_single {
                type Db = sqlx::Sqlite;
                fn fetch_all<'e, E>(
                    self,
                    executor: E,
                ) -> impl std::future::Future<
                    Output = Result<Vec<i64>, sqlx::Error>,
                > + Send + 'e
                where
                    Self: Sized + 'e,
                    E: sqlx::Executor<'e, Database = sqlx::Sqlite> + Send + 'e,
                    sqlx::Sqlite: 'e,
                {
                    __SqlForgeQuery_single::fetch_all(self, executor)
                }
                fn fetch_one<'e, E>(
                    self,
                    executor: E,
                ) -> impl std::future::Future<
                    Output = Result<i64, sqlx::Error>,
                > + Send + 'e
                where
                    Self: Sized + 'e,
                    E: sqlx::Executor<'e, Database = sqlx::Sqlite> + Send + 'e,
                    sqlx::Sqlite: 'e,
                {
                    __SqlForgeQuery_single::fetch_one(self, executor)
                }
                fn fetch_optional<'e, E>(
                    self,
                    executor: E,
                ) -> impl std::future::Future<
                    Output = Result<Option<i64>, sqlx::Error>,
                > + Send + 'e
                where
                    Self: Sized + 'e,
                    E: sqlx::Executor<'e, Database = sqlx::Sqlite> + Send + 'e,
                    sqlx::Sqlite: 'e,
                {
                    __SqlForgeQuery_single::fetch_optional(self, executor)
                }
                fn execute<'e, E>(
                    self,
                    executor: E,
                ) -> impl std::future::Future<
                    Output = Result<
                        <sqlx::Sqlite as sqlx::Database>::QueryResult,
                        sqlx::Error,
                    >,
                > + Send + 'e
                where
                    Self: Sized + 'e,
                    E: sqlx::Executor<'e, Database = sqlx::Sqlite> + Send + 'e,
                    sqlx::Sqlite: 'e,
                {
                    __SqlForgeQuery_single::execute(self, executor)
                }
            }
            let __sql_forge_runtime_min_id = 2i64;
            let mut __builder: sqlx::QueryBuilder<sqlx::Sqlite> = sqlx::QueryBuilder::new(
                "",
            );
            __builder.push("SELECT COUNT(*) FROM users WHERE id > ");
            __builder.push_bind(__sql_forge_runtime_min_id);
            let __sql_forge_value_single = __SqlForgeQuery_single {
                inner: __builder,
            };
            __sql_forge_value_single
        }
            .fetch_one(&pool)
            .await
            .expect("scalar query failed");
        match (&count, &3) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::None,
                    );
                }
            }
        };
    };
    let mut body = body;
    #[allow(unused_mut)]
    let mut body = unsafe { ::tokio::macros::support::Pin::new_unchecked(&mut body) };
    let body: ::core::pin::Pin<&mut dyn ::core::future::Future<Output = ()>> = body;
    #[allow(
        clippy::expect_used,
        clippy::diverging_sub_expression,
        clippy::needless_return,
        clippy::unwrap_in_result
    )]
    {
        use tokio::runtime::Builder;
        return Builder::new_current_thread()
            .enable_all()
            .build()
            .expect("Failed building the Runtime")
            .block_on(body);
    }
}
extern crate test;
#[rustc_test_marker = "struct_source_params"]
#[doc(hidden)]
pub const struct_source_params: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("struct_source_params"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "tests/tests.rs",
        start_line: 115usize,
        start_col: 10usize,
        end_line: 115usize,
        end_col: 30usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(struct_source_params()),
    ),
};
fn struct_source_params() {
    let body = async {
        let pool = pool().await;
        let filter = Filter { max_id: 3, limit: 2 };
        let users: Vec<User> = {
            let _sql_forge_validator = || {
                let __sql_forge_source_top_level = &(filter);
                let __sql_forge_top_level_max_id = &__sql_forge_source_top_level.max_id;
                let __sql_forge_top_level_limit = &__sql_forge_source_top_level.limit;
                {
                    type __SqlForgeModel = User;
                    {
                        let __sql_forge_validator_arg_0 = __sql_forge_top_level_max_id;
                        let __sql_forge_validator_arg_1 = __sql_forge_top_level_limit;
                        let _ = {
                            {
                                #[allow(clippy::all)]
                                {
                                    use ::sqlx::Arguments as _;
                                    let arg0 = &(__sql_forge_validator_arg_0);
                                    let arg1 = &(__sql_forge_validator_arg_1);
                                    let mut query_args = <sqlx::sqlite::Sqlite as ::sqlx::database::Database>::Arguments::default();
                                    query_args
                                        .reserve(
                                            2usize,
                                            0
                                                + ::sqlx::encode::Encode::<
                                                    sqlx::sqlite::Sqlite,
                                                >::size_hint(arg0)
                                                + ::sqlx::encode::Encode::<
                                                    sqlx::sqlite::Sqlite,
                                                >::size_hint(arg1),
                                        );
                                    let query_args = ::core::result::Result::<
                                        _,
                                        ::sqlx::error::BoxDynError,
                                    >::Ok(query_args)
                                        .and_then(move |mut query_args| {
                                            query_args.add(arg0).map(move |()| query_args)
                                        })
                                        .and_then(move |mut query_args| {
                                            query_args.add(arg1).map(move |()| query_args)
                                        });
                                    ::sqlx::__query_with_result::<
                                        sqlx::sqlite::Sqlite,
                                        _,
                                    >(
                                            "SELECT id, name FROM users WHERE id <= ? LIMIT ?",
                                            query_args,
                                        )
                                        .try_map(|row: sqlx::sqlite::SqliteRow| {
                                            use ::sqlx::Row as _;
                                            #[allow(non_snake_case)]
                                            let sqlx_query_as_id = row
                                                .try_get_unchecked::<i64, _>(0usize)?
                                                .into();
                                            #[allow(non_snake_case)]
                                            let sqlx_query_as_name = row
                                                .try_get_unchecked::<String, _>(1usize)?
                                                .into();
                                            ::std::result::Result::Ok(__SqlForgeModel {
                                                id: sqlx_query_as_id,
                                                name: sqlx_query_as_name,
                                            })
                                        })
                                }
                            }
                        };
                    }
                }
            };
            struct __SqlForgeQuery_single {
                inner: sqlx::QueryBuilder<sqlx::Sqlite>,
            }
            impl __SqlForgeQuery_single {
                async fn fetch_all<'e, E>(
                    mut self,
                    executor: E,
                ) -> Result<Vec<User>, sqlx::Error>
                where
                    E: sqlx::Executor<'e, Database = sqlx::Sqlite>,
                {
                    self.inner.build_query_as::<User>().fetch_all(executor).await
                }
                async fn fetch_one<'e, E>(
                    mut self,
                    executor: E,
                ) -> Result<User, sqlx::Error>
                where
                    E: sqlx::Executor<'e, Database = sqlx::Sqlite>,
                {
                    self.inner.build_query_as::<User>().fetch_one(executor).await
                }
                async fn fetch_optional<'e, E>(
                    mut self,
                    executor: E,
                ) -> Result<Option<User>, sqlx::Error>
                where
                    E: sqlx::Executor<'e, Database = sqlx::Sqlite>,
                {
                    self.inner.build_query_as::<User>().fetch_optional(executor).await
                }
                async fn execute<'e, E>(
                    mut self,
                    executor: E,
                ) -> Result<<sqlx::Sqlite as sqlx::Database>::QueryResult, sqlx::Error>
                where
                    E: sqlx::Executor<'e, Database = sqlx::Sqlite>,
                {
                    self.inner.build().execute(executor).await
                }
            }
            impl sql_forge::SqlForgeQuery<User> for __SqlForgeQuery_single {
                type Db = sqlx::Sqlite;
                fn fetch_all<'e, E>(
                    self,
                    executor: E,
                ) -> impl std::future::Future<
                    Output = Result<Vec<User>, sqlx::Error>,
                > + Send + 'e
                where
                    Self: Sized + 'e,
                    E: sqlx::Executor<'e, Database = sqlx::Sqlite> + Send + 'e,
                    sqlx::Sqlite: 'e,
                {
                    __SqlForgeQuery_single::fetch_all(self, executor)
                }
                fn fetch_one<'e, E>(
                    self,
                    executor: E,
                ) -> impl std::future::Future<
                    Output = Result<User, sqlx::Error>,
                > + Send + 'e
                where
                    Self: Sized + 'e,
                    E: sqlx::Executor<'e, Database = sqlx::Sqlite> + Send + 'e,
                    sqlx::Sqlite: 'e,
                {
                    __SqlForgeQuery_single::fetch_one(self, executor)
                }
                fn fetch_optional<'e, E>(
                    self,
                    executor: E,
                ) -> impl std::future::Future<
                    Output = Result<Option<User>, sqlx::Error>,
                > + Send + 'e
                where
                    Self: Sized + 'e,
                    E: sqlx::Executor<'e, Database = sqlx::Sqlite> + Send + 'e,
                    sqlx::Sqlite: 'e,
                {
                    __SqlForgeQuery_single::fetch_optional(self, executor)
                }
                fn execute<'e, E>(
                    self,
                    executor: E,
                ) -> impl std::future::Future<
                    Output = Result<
                        <sqlx::Sqlite as sqlx::Database>::QueryResult,
                        sqlx::Error,
                    >,
                > + Send + 'e
                where
                    Self: Sized + 'e,
                    E: sqlx::Executor<'e, Database = sqlx::Sqlite> + Send + 'e,
                    sqlx::Sqlite: 'e,
                {
                    __SqlForgeQuery_single::execute(self, executor)
                }
            }
            let __sql_forge_source_runtime = &(filter);
            let __sql_forge_runtime_max_id = __sql_forge_source_runtime.max_id;
            let __sql_forge_runtime_limit = __sql_forge_source_runtime.limit;
            let mut __builder: sqlx::QueryBuilder<sqlx::Sqlite> = sqlx::QueryBuilder::new(
                "",
            );
            __builder.push("SELECT id, name FROM users WHERE id <= ");
            __builder.push_bind(__sql_forge_runtime_max_id);
            __builder.push(" LIMIT ");
            __builder.push_bind(__sql_forge_runtime_limit);
            let __sql_forge_value_single = __SqlForgeQuery_single {
                inner: __builder,
            };
            __sql_forge_value_single
        }
            .fetch_all(&pool)
            .await
            .expect("struct source query failed");
        match (&users.len(), &2) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::None,
                    );
                }
            }
        };
    };
    let mut body = body;
    #[allow(unused_mut)]
    let mut body = unsafe { ::tokio::macros::support::Pin::new_unchecked(&mut body) };
    let body: ::core::pin::Pin<&mut dyn ::core::future::Future<Output = ()>> = body;
    #[allow(
        clippy::expect_used,
        clippy::diverging_sub_expression,
        clippy::needless_return,
        clippy::unwrap_in_result
    )]
    {
        use tokio::runtime::Builder;
        return Builder::new_current_thread()
            .enable_all()
            .build()
            .expect("Failed building the Runtime")
            .block_on(body);
    }
}
extern crate test;
#[rustc_test_marker = "section_dynamic_where"]
#[doc(hidden)]
pub const section_dynamic_where: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("section_dynamic_where"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "tests/tests.rs",
        start_line: 136usize,
        start_col: 10usize,
        end_line: 136usize,
        end_col: 31usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(section_dynamic_where()),
    ),
};
fn section_dynamic_where() {
    let body = async {
        let pool = pool().await;
        let cat = "Electronics";
        let products: Vec<Product> = {
            let _sql_forge_validator = || {
                {
                    type __SqlForgeModel = Product;
                    {
                        let __sql_forge_section_case_single_0_filter_category_cat = &(cat);
                        let __sql_forge_validator_arg_0 = __sql_forge_section_case_single_0_filter_category_cat;
                        let _ = {
                            {
                                #[allow(clippy::all)]
                                {
                                    use ::sqlx::Arguments as _;
                                    let arg0 = &(__sql_forge_validator_arg_0);
                                    let mut query_args = <sqlx::sqlite::Sqlite as ::sqlx::database::Database>::Arguments::default();
                                    query_args
                                        .reserve(
                                            1usize,
                                            0
                                                + ::sqlx::encode::Encode::<
                                                    sqlx::sqlite::Sqlite,
                                                >::size_hint(arg0),
                                        );
                                    let query_args = ::core::result::Result::<
                                        _,
                                        ::sqlx::error::BoxDynError,
                                    >::Ok(query_args)
                                        .and_then(move |mut query_args| {
                                            query_args.add(arg0).map(move |()| query_args)
                                        });
                                    ::sqlx::__query_with_result::<
                                        sqlx::sqlite::Sqlite,
                                        _,
                                    >(
                                            "\n        SELECT id, name, price, stock, category\n        FROM products\n        WHERE 1 = 1\n         AND category = ? \n        ",
                                            query_args,
                                        )
                                        .try_map(|row: sqlx::sqlite::SqliteRow| {
                                            use ::sqlx::Row as _;
                                            #[allow(non_snake_case)]
                                            let sqlx_query_as_id = row
                                                .try_get_unchecked::<i64, _>(0usize)?
                                                .into();
                                            #[allow(non_snake_case)]
                                            let sqlx_query_as_name = row
                                                .try_get_unchecked::<String, _>(1usize)?
                                                .into();
                                            #[allow(non_snake_case)]
                                            let sqlx_query_as_price = row
                                                .try_get_unchecked::<i64, _>(2usize)?
                                                .into();
                                            #[allow(non_snake_case)]
                                            let sqlx_query_as_stock = row
                                                .try_get_unchecked::<i64, _>(3usize)?
                                                .into();
                                            #[allow(non_snake_case)]
                                            let sqlx_query_as_category = row
                                                .try_get_unchecked::<String, _>(4usize)?
                                                .into();
                                            ::std::result::Result::Ok(__SqlForgeModel {
                                                id: sqlx_query_as_id,
                                                name: sqlx_query_as_name,
                                                price: sqlx_query_as_price,
                                                stock: sqlx_query_as_stock,
                                                category: sqlx_query_as_category,
                                            })
                                        })
                                }
                            }
                        };
                    }
                }
            };
            struct __SqlForgeQuery_single {
                inner: sqlx::QueryBuilder<sqlx::Sqlite>,
            }
            impl __SqlForgeQuery_single {
                async fn fetch_all<'e, E>(
                    mut self,
                    executor: E,
                ) -> Result<Vec<Product>, sqlx::Error>
                where
                    E: sqlx::Executor<'e, Database = sqlx::Sqlite>,
                {
                    self.inner.build_query_as::<Product>().fetch_all(executor).await
                }
                async fn fetch_one<'e, E>(
                    mut self,
                    executor: E,
                ) -> Result<Product, sqlx::Error>
                where
                    E: sqlx::Executor<'e, Database = sqlx::Sqlite>,
                {
                    self.inner.build_query_as::<Product>().fetch_one(executor).await
                }
                async fn fetch_optional<'e, E>(
                    mut self,
                    executor: E,
                ) -> Result<Option<Product>, sqlx::Error>
                where
                    E: sqlx::Executor<'e, Database = sqlx::Sqlite>,
                {
                    self.inner.build_query_as::<Product>().fetch_optional(executor).await
                }
                async fn execute<'e, E>(
                    mut self,
                    executor: E,
                ) -> Result<<sqlx::Sqlite as sqlx::Database>::QueryResult, sqlx::Error>
                where
                    E: sqlx::Executor<'e, Database = sqlx::Sqlite>,
                {
                    self.inner.build().execute(executor).await
                }
            }
            impl sql_forge::SqlForgeQuery<Product> for __SqlForgeQuery_single {
                type Db = sqlx::Sqlite;
                fn fetch_all<'e, E>(
                    self,
                    executor: E,
                ) -> impl std::future::Future<
                    Output = Result<Vec<Product>, sqlx::Error>,
                > + Send + 'e
                where
                    Self: Sized + 'e,
                    E: sqlx::Executor<'e, Database = sqlx::Sqlite> + Send + 'e,
                    sqlx::Sqlite: 'e,
                {
                    __SqlForgeQuery_single::fetch_all(self, executor)
                }
                fn fetch_one<'e, E>(
                    self,
                    executor: E,
                ) -> impl std::future::Future<
                    Output = Result<Product, sqlx::Error>,
                > + Send + 'e
                where
                    Self: Sized + 'e,
                    E: sqlx::Executor<'e, Database = sqlx::Sqlite> + Send + 'e,
                    sqlx::Sqlite: 'e,
                {
                    __SqlForgeQuery_single::fetch_one(self, executor)
                }
                fn fetch_optional<'e, E>(
                    self,
                    executor: E,
                ) -> impl std::future::Future<
                    Output = Result<Option<Product>, sqlx::Error>,
                > + Send + 'e
                where
                    Self: Sized + 'e,
                    E: sqlx::Executor<'e, Database = sqlx::Sqlite> + Send + 'e,
                    sqlx::Sqlite: 'e,
                {
                    __SqlForgeQuery_single::fetch_optional(self, executor)
                }
                fn execute<'e, E>(
                    self,
                    executor: E,
                ) -> impl std::future::Future<
                    Output = Result<
                        <sqlx::Sqlite as sqlx::Database>::QueryResult,
                        sqlx::Error,
                    >,
                > + Send + 'e
                where
                    Self: Sized + 'e,
                    E: sqlx::Executor<'e, Database = sqlx::Sqlite> + Send + 'e,
                    sqlx::Sqlite: 'e,
                {
                    __SqlForgeQuery_single::execute(self, executor)
                }
            }
            let mut __builder: sqlx::QueryBuilder<sqlx::Sqlite> = sqlx::QueryBuilder::new(
                "",
            );
            __builder
                .push(
                    "\n        SELECT id, name, price, stock, category\n        FROM products\n        WHERE 1 = 1\n        ",
                );
            {
                let __sql_forge_section_filter_category_cat = cat;
                __builder.push(" AND category = ");
                __builder.push_bind(__sql_forge_section_filter_category_cat);
                __builder.push(" ");
            }
            __builder.push("\n        ");
            let __sql_forge_value_single = __SqlForgeQuery_single {
                inner: __builder,
            };
            __sql_forge_value_single
        }
            .fetch_all(&pool)
            .await
            .expect("section query failed");
        if !(products.len() >= 3) {
            ::core::panicking::panic("assertion failed: products.len() >= 3")
        }
        for p in &products {
            match (&p.category, &"Electronics") {
                (left_val, right_val) => {
                    if !(*left_val == *right_val) {
                        let kind = ::core::panicking::AssertKind::Eq;
                        ::core::panicking::assert_failed(
                            kind,
                            &*left_val,
                            &*right_val,
                            ::core::option::Option::None,
                        );
                    }
                }
            };
        }
    };
    let mut body = body;
    #[allow(unused_mut)]
    let mut body = unsafe { ::tokio::macros::support::Pin::new_unchecked(&mut body) };
    let body: ::core::pin::Pin<&mut dyn ::core::future::Future<Output = ()>> = body;
    #[allow(
        clippy::expect_used,
        clippy::diverging_sub_expression,
        clippy::needless_return,
        clippy::unwrap_in_result
    )]
    {
        use tokio::runtime::Builder;
        return Builder::new_current_thread()
            .enable_all()
            .build()
            .expect("Failed building the Runtime")
            .block_on(body);
    }
}
extern crate test;
#[rustc_test_marker = "section_with_local_params"]
#[doc(hidden)]
pub const section_with_local_params: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("section_with_local_params"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "tests/tests.rs",
        start_line: 167usize,
        start_col: 10usize,
        end_line: 167usize,
        end_col: 35usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(section_with_local_params()),
    ),
};
fn section_with_local_params() {
    let body = async {
        let pool = pool().await;
        let max_id = 4i64;
        let users: Vec<User> = {
            let _sql_forge_validator = || {
                {
                    type __SqlForgeModel = User;
                    {
                        let __sql_forge_section_case_single_0_filter_max_id = &(max_id);
                        let __sql_forge_validator_arg_0 = __sql_forge_section_case_single_0_filter_max_id;
                        let _ = {
                            {
                                #[allow(clippy::all)]
                                {
                                    use ::sqlx::Arguments as _;
                                    let arg0 = &(__sql_forge_validator_arg_0);
                                    let mut query_args = <sqlx::sqlite::Sqlite as ::sqlx::database::Database>::Arguments::default();
                                    query_args
                                        .reserve(
                                            1usize,
                                            0
                                                + ::sqlx::encode::Encode::<
                                                    sqlx::sqlite::Sqlite,
                                                >::size_hint(arg0),
                                        );
                                    let query_args = ::core::result::Result::<
                                        _,
                                        ::sqlx::error::BoxDynError,
                                    >::Ok(query_args)
                                        .and_then(move |mut query_args| {
                                            query_args.add(arg0).map(move |()| query_args)
                                        });
                                    ::sqlx::__query_with_result::<
                                        sqlx::sqlite::Sqlite,
                                        _,
                                    >("SELECT id, name FROM users  WHERE id <= ? ", query_args)
                                        .try_map(|row: sqlx::sqlite::SqliteRow| {
                                            use ::sqlx::Row as _;
                                            #[allow(non_snake_case)]
                                            let sqlx_query_as_id = row
                                                .try_get_unchecked::<i64, _>(0usize)?
                                                .into();
                                            #[allow(non_snake_case)]
                                            let sqlx_query_as_name = row
                                                .try_get_unchecked::<String, _>(1usize)?
                                                .into();
                                            ::std::result::Result::Ok(__SqlForgeModel {
                                                id: sqlx_query_as_id,
                                                name: sqlx_query_as_name,
                                            })
                                        })
                                }
                            }
                        };
                    }
                }
            };
            struct __SqlForgeQuery_single {
                inner: sqlx::QueryBuilder<sqlx::Sqlite>,
            }
            impl __SqlForgeQuery_single {
                async fn fetch_all<'e, E>(
                    mut self,
                    executor: E,
                ) -> Result<Vec<User>, sqlx::Error>
                where
                    E: sqlx::Executor<'e, Database = sqlx::Sqlite>,
                {
                    self.inner.build_query_as::<User>().fetch_all(executor).await
                }
                async fn fetch_one<'e, E>(
                    mut self,
                    executor: E,
                ) -> Result<User, sqlx::Error>
                where
                    E: sqlx::Executor<'e, Database = sqlx::Sqlite>,
                {
                    self.inner.build_query_as::<User>().fetch_one(executor).await
                }
                async fn fetch_optional<'e, E>(
                    mut self,
                    executor: E,
                ) -> Result<Option<User>, sqlx::Error>
                where
                    E: sqlx::Executor<'e, Database = sqlx::Sqlite>,
                {
                    self.inner.build_query_as::<User>().fetch_optional(executor).await
                }
                async fn execute<'e, E>(
                    mut self,
                    executor: E,
                ) -> Result<<sqlx::Sqlite as sqlx::Database>::QueryResult, sqlx::Error>
                where
                    E: sqlx::Executor<'e, Database = sqlx::Sqlite>,
                {
                    self.inner.build().execute(executor).await
                }
            }
            impl sql_forge::SqlForgeQuery<User> for __SqlForgeQuery_single {
                type Db = sqlx::Sqlite;
                fn fetch_all<'e, E>(
                    self,
                    executor: E,
                ) -> impl std::future::Future<
                    Output = Result<Vec<User>, sqlx::Error>,
                > + Send + 'e
                where
                    Self: Sized + 'e,
                    E: sqlx::Executor<'e, Database = sqlx::Sqlite> + Send + 'e,
                    sqlx::Sqlite: 'e,
                {
                    __SqlForgeQuery_single::fetch_all(self, executor)
                }
                fn fetch_one<'e, E>(
                    self,
                    executor: E,
                ) -> impl std::future::Future<
                    Output = Result<User, sqlx::Error>,
                > + Send + 'e
                where
                    Self: Sized + 'e,
                    E: sqlx::Executor<'e, Database = sqlx::Sqlite> + Send + 'e,
                    sqlx::Sqlite: 'e,
                {
                    __SqlForgeQuery_single::fetch_one(self, executor)
                }
                fn fetch_optional<'e, E>(
                    self,
                    executor: E,
                ) -> impl std::future::Future<
                    Output = Result<Option<User>, sqlx::Error>,
                > + Send + 'e
                where
                    Self: Sized + 'e,
                    E: sqlx::Executor<'e, Database = sqlx::Sqlite> + Send + 'e,
                    sqlx::Sqlite: 'e,
                {
                    __SqlForgeQuery_single::fetch_optional(self, executor)
                }
                fn execute<'e, E>(
                    self,
                    executor: E,
                ) -> impl std::future::Future<
                    Output = Result<
                        <sqlx::Sqlite as sqlx::Database>::QueryResult,
                        sqlx::Error,
                    >,
                > + Send + 'e
                where
                    Self: Sized + 'e,
                    E: sqlx::Executor<'e, Database = sqlx::Sqlite> + Send + 'e,
                    sqlx::Sqlite: 'e,
                {
                    __SqlForgeQuery_single::execute(self, executor)
                }
            }
            let mut __builder: sqlx::QueryBuilder<sqlx::Sqlite> = sqlx::QueryBuilder::new(
                "",
            );
            __builder.push("SELECT id, name FROM users ");
            {
                let __sql_forge_section_filter_max_id = max_id;
                __builder.push(" WHERE id <= ");
                __builder.push_bind(__sql_forge_section_filter_max_id);
                __builder.push(" ");
            }
            let __sql_forge_value_single = __SqlForgeQuery_single {
                inner: __builder,
            };
            __sql_forge_value_single
        }
            .fetch_all(&pool)
            .await
            .expect("section with local params failed");
        match (&users.len(), &4) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::None,
                    );
                }
            }
        };
    };
    let mut body = body;
    #[allow(unused_mut)]
    let mut body = unsafe { ::tokio::macros::support::Pin::new_unchecked(&mut body) };
    let body: ::core::pin::Pin<&mut dyn ::core::future::Future<Output = ()>> = body;
    #[allow(
        clippy::expect_used,
        clippy::diverging_sub_expression,
        clippy::needless_return,
        clippy::unwrap_in_result
    )]
    {
        use tokio::runtime::Builder;
        return Builder::new_current_thread()
            .enable_all()
            .build()
            .expect("Failed building the Runtime")
            .block_on(body);
    }
}
extern crate test;
#[rustc_test_marker = "grouped_sections"]
#[doc(hidden)]
pub const grouped_sections: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("grouped_sections"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "tests/tests.rs",
        start_line: 190usize,
        start_col: 10usize,
        end_line: 190usize,
        end_col: 26usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(grouped_sections()),
    ),
};
fn grouped_sections() {
    let body = async {
        let pool = pool().await;
        let include_org = true;
        struct Row {
            #[expect(dead_code)]
            field_1: i64,
            field_2: String,
        }
        #[automatically_derived]
        impl<'a, R: ::sqlx::Row> ::sqlx::FromRow<'a, R> for Row
        where
            &'a ::std::primitive::str: ::sqlx::ColumnIndex<R>,
            i64: ::sqlx::decode::Decode<'a, R::Database>,
            i64: ::sqlx::types::Type<R::Database>,
            String: ::sqlx::decode::Decode<'a, R::Database>,
            String: ::sqlx::types::Type<R::Database>,
        {
            fn from_row(__row: &'a R) -> ::sqlx::Result<Self> {
                let field_1: i64 = __row.try_get("field_1")?;
                let field_2: String = __row.try_get("field_2")?;
                ::std::result::Result::Ok(Row { field_1, field_2 })
            }
        }
        let rows: Vec<Row> = {
            let _sql_forge_validator = || {
                {
                    type __SqlForgeModel = Row;
                    {
                        let _ = {
                            {
                                #[allow(clippy::all)]
                                {
                                    use ::sqlx::Arguments as _;
                                    let query_args = ::core::result::Result::<
                                        _,
                                        ::sqlx::error::BoxDynError,
                                    >::Ok(
                                        <sqlx::sqlite::Sqlite as ::sqlx::database::Database>::Arguments::default(),
                                    );
                                    ::sqlx::__query_with_result::<
                                        sqlx::sqlite::Sqlite,
                                        _,
                                    >(
                                            "\n        SELECT t1.id AS field_1, o.name AS field_2\n        FROM users t1\n         JOIN organisations o ON o.id = t1.id \n        WHERE 1 = 1\n        ",
                                            query_args,
                                        )
                                        .try_map(|row: sqlx::sqlite::SqliteRow| {
                                            use ::sqlx::Row as _;
                                            #[allow(non_snake_case)]
                                            let sqlx_query_as_field_1 = row
                                                .try_get_unchecked::<i64, _>(0usize)?
                                                .into();
                                            #[allow(non_snake_case)]
                                            let sqlx_query_as_field_2 = row
                                                .try_get_unchecked::<String, _>(1usize)?
                                                .into();
                                            ::std::result::Result::Ok(__SqlForgeModel {
                                                field_1: sqlx_query_as_field_1,
                                                field_2: sqlx_query_as_field_2,
                                            })
                                        })
                                }
                            }
                        };
                    }
                    {
                        let _ = {
                            {
                                #[allow(clippy::all)]
                                {
                                    use ::sqlx::Arguments as _;
                                    let query_args = ::core::result::Result::<
                                        _,
                                        ::sqlx::error::BoxDynError,
                                    >::Ok(
                                        <sqlx::sqlite::Sqlite as ::sqlx::database::Database>::Arguments::default(),
                                    );
                                    ::sqlx::__query_with_result::<
                                        sqlx::sqlite::Sqlite,
                                        _,
                                    >(
                                            "\n        SELECT t1.id AS field_1, t1.name AS field_2\n        FROM users t1\n        \n        WHERE 1 = 1\n        ",
                                            query_args,
                                        )
                                        .try_map(|row: sqlx::sqlite::SqliteRow| {
                                            use ::sqlx::Row as _;
                                            #[allow(non_snake_case)]
                                            let sqlx_query_as_field_1 = row
                                                .try_get_unchecked::<i64, _>(0usize)?
                                                .into();
                                            #[allow(non_snake_case)]
                                            let sqlx_query_as_field_2 = row
                                                .try_get_unchecked::<String, _>(1usize)?
                                                .into();
                                            ::std::result::Result::Ok(__SqlForgeModel {
                                                field_1: sqlx_query_as_field_1,
                                                field_2: sqlx_query_as_field_2,
                                            })
                                        })
                                }
                            }
                        };
                    }
                }
            };
            struct __SqlForgeQuery_single {
                inner: sqlx::QueryBuilder<sqlx::Sqlite>,
            }
            impl __SqlForgeQuery_single {
                async fn fetch_all<'e, E>(
                    mut self,
                    executor: E,
                ) -> Result<Vec<Row>, sqlx::Error>
                where
                    E: sqlx::Executor<'e, Database = sqlx::Sqlite>,
                {
                    self.inner.build_query_as::<Row>().fetch_all(executor).await
                }
                async fn fetch_one<'e, E>(
                    mut self,
                    executor: E,
                ) -> Result<Row, sqlx::Error>
                where
                    E: sqlx::Executor<'e, Database = sqlx::Sqlite>,
                {
                    self.inner.build_query_as::<Row>().fetch_one(executor).await
                }
                async fn fetch_optional<'e, E>(
                    mut self,
                    executor: E,
                ) -> Result<Option<Row>, sqlx::Error>
                where
                    E: sqlx::Executor<'e, Database = sqlx::Sqlite>,
                {
                    self.inner.build_query_as::<Row>().fetch_optional(executor).await
                }
                async fn execute<'e, E>(
                    mut self,
                    executor: E,
                ) -> Result<<sqlx::Sqlite as sqlx::Database>::QueryResult, sqlx::Error>
                where
                    E: sqlx::Executor<'e, Database = sqlx::Sqlite>,
                {
                    self.inner.build().execute(executor).await
                }
            }
            impl sql_forge::SqlForgeQuery<Row> for __SqlForgeQuery_single {
                type Db = sqlx::Sqlite;
                fn fetch_all<'e, E>(
                    self,
                    executor: E,
                ) -> impl std::future::Future<
                    Output = Result<Vec<Row>, sqlx::Error>,
                > + Send + 'e
                where
                    Self: Sized + 'e,
                    E: sqlx::Executor<'e, Database = sqlx::Sqlite> + Send + 'e,
                    sqlx::Sqlite: 'e,
                {
                    __SqlForgeQuery_single::fetch_all(self, executor)
                }
                fn fetch_one<'e, E>(
                    self,
                    executor: E,
                ) -> impl std::future::Future<
                    Output = Result<Row, sqlx::Error>,
                > + Send + 'e
                where
                    Self: Sized + 'e,
                    E: sqlx::Executor<'e, Database = sqlx::Sqlite> + Send + 'e,
                    sqlx::Sqlite: 'e,
                {
                    __SqlForgeQuery_single::fetch_one(self, executor)
                }
                fn fetch_optional<'e, E>(
                    self,
                    executor: E,
                ) -> impl std::future::Future<
                    Output = Result<Option<Row>, sqlx::Error>,
                > + Send + 'e
                where
                    Self: Sized + 'e,
                    E: sqlx::Executor<'e, Database = sqlx::Sqlite> + Send + 'e,
                    sqlx::Sqlite: 'e,
                {
                    __SqlForgeQuery_single::fetch_optional(self, executor)
                }
                fn execute<'e, E>(
                    self,
                    executor: E,
                ) -> impl std::future::Future<
                    Output = Result<
                        <sqlx::Sqlite as sqlx::Database>::QueryResult,
                        sqlx::Error,
                    >,
                > + Send + 'e
                where
                    Self: Sized + 'e,
                    E: sqlx::Executor<'e, Database = sqlx::Sqlite> + Send + 'e,
                    sqlx::Sqlite: 'e,
                {
                    __SqlForgeQuery_single::execute(self, executor)
                }
            }
            let mut __builder: sqlx::QueryBuilder<sqlx::Sqlite> = sqlx::QueryBuilder::new(
                "",
            );
            __builder.push("\n        SELECT t1.id AS field_1, ");
            match include_org {
                true => {
                    __builder.push("o.name AS field_2");
                }
                false => {
                    __builder.push("t1.name AS field_2");
                }
            }
            __builder.push("\n        FROM users t1\n        ");
            match include_org {
                true => {
                    __builder.push(" JOIN organisations o ON o.id = t1.id ");
                }
                false => {}
            }
            __builder.push("\n        WHERE 1 = 1\n        ");
            let __sql_forge_value_single = __SqlForgeQuery_single {
                inner: __builder,
            };
            __sql_forge_value_single
        }
            .fetch_all(&pool)
            .await
            .expect("grouped sections query failed");
        match (&rows.len(), &3) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::None,
                    );
                }
            }
        };
        match (&rows[0].field_2, &"Org Alpha") {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::None,
                    );
                }
            }
        };
        match (&rows[1].field_2, &"Org Beta") {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::None,
                    );
                }
            }
        };
    };
    let mut body = body;
    #[allow(unused_mut)]
    let mut body = unsafe { ::tokio::macros::support::Pin::new_unchecked(&mut body) };
    let body: ::core::pin::Pin<&mut dyn ::core::future::Future<Output = ()>> = body;
    #[allow(
        clippy::expect_used,
        clippy::diverging_sub_expression,
        clippy::needless_return,
        clippy::unwrap_in_result
    )]
    {
        use tokio::runtime::Builder;
        return Builder::new_current_thread()
            .enable_all()
            .build()
            .expect("Failed building the Runtime")
            .block_on(body);
    }
}
extern crate test;
#[rustc_test_marker = "grouped_sections_with_nested_matches"]
#[doc(hidden)]
pub const grouped_sections_with_nested_matches: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("grouped_sections_with_nested_matches"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "tests/tests.rs",
        start_line: 230usize,
        start_col: 10usize,
        end_line: 230usize,
        end_col: 46usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(grouped_sections_with_nested_matches()),
    ),
};
fn grouped_sections_with_nested_matches() {
    let body = async {
        let pool = pool().await;
        let include_org = true;
        let can_read_org_name = false;
        let use_org_label = true;
        struct Row {
            field_1: i64,
            field_2: Option<String>,
            field_3: Option<String>,
        }
        #[automatically_derived]
        impl<'a, R: ::sqlx::Row> ::sqlx::FromRow<'a, R> for Row
        where
            &'a ::std::primitive::str: ::sqlx::ColumnIndex<R>,
            i64: ::sqlx::decode::Decode<'a, R::Database>,
            i64: ::sqlx::types::Type<R::Database>,
            Option<String>: ::sqlx::decode::Decode<'a, R::Database>,
            Option<String>: ::sqlx::types::Type<R::Database>,
            Option<String>: ::sqlx::decode::Decode<'a, R::Database>,
            Option<String>: ::sqlx::types::Type<R::Database>,
        {
            fn from_row(__row: &'a R) -> ::sqlx::Result<Self> {
                let field_1: i64 = __row.try_get("field_1")?;
                let field_2: Option<String> = __row.try_get("field_2")?;
                let field_3: Option<String> = __row.try_get("field_3")?;
                ::std::result::Result::Ok(Row { field_1, field_2, field_3 })
            }
        }
        let rows: Vec<Row> = {
            let _sql_forge_validator = || {
                {
                    type __SqlForgeModel = Row;
                    {
                        let _ = {
                            {
                                #[allow(clippy::all)]
                                {
                                    use ::sqlx::Arguments as _;
                                    let query_args = ::core::result::Result::<
                                        _,
                                        ::sqlx::error::BoxDynError,
                                    >::Ok(
                                        <sqlx::sqlite::Sqlite as ::sqlx::database::Database>::Arguments::default(),
                                    );
                                    ::sqlx::__query_with_result::<
                                        sqlx::sqlite::Sqlite,
                                        _,
                                    >(
                                            "\n        SELECT t1.id AS field_1, COALESCE(o.name, '') AS field_2, COALESCE('org', '') AS field_3\n        FROM users t1\n         JOIN organisations o ON o.id = t1.id \n        WHERE 1 = 1\n        ",
                                            query_args,
                                        )
                                        .try_map(|row: sqlx::sqlite::SqliteRow| {
                                            use ::sqlx::Row as _;
                                            #[allow(non_snake_case)]
                                            let sqlx_query_as_field_1 = row
                                                .try_get_unchecked::<i64, _>(0usize)?
                                                .into();
                                            #[allow(non_snake_case)]
                                            let sqlx_query_as_field_2 = row
                                                .try_get_unchecked::<String, _>(1usize)?
                                                .into();
                                            #[allow(non_snake_case)]
                                            let sqlx_query_as_field_3 = row
                                                .try_get_unchecked::<String, _>(2usize)?
                                                .into();
                                            ::std::result::Result::Ok(__SqlForgeModel {
                                                field_1: sqlx_query_as_field_1,
                                                field_2: sqlx_query_as_field_2,
                                                field_3: sqlx_query_as_field_3,
                                            })
                                        })
                                }
                            }
                        };
                    }
                    {
                        let _ = {
                            {
                                #[allow(clippy::all)]
                                {
                                    use ::sqlx::Arguments as _;
                                    let query_args = ::core::result::Result::<
                                        _,
                                        ::sqlx::error::BoxDynError,
                                    >::Ok(
                                        <sqlx::sqlite::Sqlite as ::sqlx::database::Database>::Arguments::default(),
                                    );
                                    ::sqlx::__query_with_result::<
                                        sqlx::sqlite::Sqlite,
                                        _,
                                    >(
                                            "\n        SELECT t1.id AS field_1, COALESCE(t1.name, '') AS field_2, COALESCE('user', '') AS field_3\n        FROM users t1\n         JOIN organisations o ON o.id = t1.id \n        WHERE 1 = 1\n        ",
                                            query_args,
                                        )
                                        .try_map(|row: sqlx::sqlite::SqliteRow| {
                                            use ::sqlx::Row as _;
                                            #[allow(non_snake_case)]
                                            let sqlx_query_as_field_1 = row
                                                .try_get_unchecked::<i64, _>(0usize)?
                                                .into();
                                            #[allow(non_snake_case)]
                                            let sqlx_query_as_field_2 = row
                                                .try_get_unchecked::<String, _>(1usize)?
                                                .into();
                                            #[allow(non_snake_case)]
                                            let sqlx_query_as_field_3 = row
                                                .try_get_unchecked::<String, _>(2usize)?
                                                .into();
                                            ::std::result::Result::Ok(__SqlForgeModel {
                                                field_1: sqlx_query_as_field_1,
                                                field_2: sqlx_query_as_field_2,
                                                field_3: sqlx_query_as_field_3,
                                            })
                                        })
                                }
                            }
                        };
                    }
                    {
                        let _ = {
                            {
                                #[allow(clippy::all)]
                                {
                                    use ::sqlx::Arguments as _;
                                    let query_args = ::core::result::Result::<
                                        _,
                                        ::sqlx::error::BoxDynError,
                                    >::Ok(
                                        <sqlx::sqlite::Sqlite as ::sqlx::database::Database>::Arguments::default(),
                                    );
                                    ::sqlx::__query_with_result::<
                                        sqlx::sqlite::Sqlite,
                                        _,
                                    >(
                                            "\n        SELECT t1.id AS field_1, COALESCE(t1.name, '') AS field_2, COALESCE('no_join', '') AS field_3\n        FROM users t1\n        \n        WHERE 1 = 1\n        ",
                                            query_args,
                                        )
                                        .try_map(|row: sqlx::sqlite::SqliteRow| {
                                            use ::sqlx::Row as _;
                                            #[allow(non_snake_case)]
                                            let sqlx_query_as_field_1 = row
                                                .try_get_unchecked::<i64, _>(0usize)?
                                                .into();
                                            #[allow(non_snake_case)]
                                            let sqlx_query_as_field_2 = row
                                                .try_get_unchecked::<String, _>(1usize)?
                                                .into();
                                            #[allow(non_snake_case)]
                                            let sqlx_query_as_field_3 = row
                                                .try_get_unchecked::<String, _>(2usize)?
                                                .into();
                                            ::std::result::Result::Ok(__SqlForgeModel {
                                                field_1: sqlx_query_as_field_1,
                                                field_2: sqlx_query_as_field_2,
                                                field_3: sqlx_query_as_field_3,
                                            })
                                        })
                                }
                            }
                        };
                    }
                }
            };
            struct __SqlForgeQuery_single {
                inner: sqlx::QueryBuilder<sqlx::Sqlite>,
            }
            impl __SqlForgeQuery_single {
                async fn fetch_all<'e, E>(
                    mut self,
                    executor: E,
                ) -> Result<Vec<Row>, sqlx::Error>
                where
                    E: sqlx::Executor<'e, Database = sqlx::Sqlite>,
                {
                    self.inner.build_query_as::<Row>().fetch_all(executor).await
                }
                async fn fetch_one<'e, E>(
                    mut self,
                    executor: E,
                ) -> Result<Row, sqlx::Error>
                where
                    E: sqlx::Executor<'e, Database = sqlx::Sqlite>,
                {
                    self.inner.build_query_as::<Row>().fetch_one(executor).await
                }
                async fn fetch_optional<'e, E>(
                    mut self,
                    executor: E,
                ) -> Result<Option<Row>, sqlx::Error>
                where
                    E: sqlx::Executor<'e, Database = sqlx::Sqlite>,
                {
                    self.inner.build_query_as::<Row>().fetch_optional(executor).await
                }
                async fn execute<'e, E>(
                    mut self,
                    executor: E,
                ) -> Result<<sqlx::Sqlite as sqlx::Database>::QueryResult, sqlx::Error>
                where
                    E: sqlx::Executor<'e, Database = sqlx::Sqlite>,
                {
                    self.inner.build().execute(executor).await
                }
            }
            impl sql_forge::SqlForgeQuery<Row> for __SqlForgeQuery_single {
                type Db = sqlx::Sqlite;
                fn fetch_all<'e, E>(
                    self,
                    executor: E,
                ) -> impl std::future::Future<
                    Output = Result<Vec<Row>, sqlx::Error>,
                > + Send + 'e
                where
                    Self: Sized + 'e,
                    E: sqlx::Executor<'e, Database = sqlx::Sqlite> + Send + 'e,
                    sqlx::Sqlite: 'e,
                {
                    __SqlForgeQuery_single::fetch_all(self, executor)
                }
                fn fetch_one<'e, E>(
                    self,
                    executor: E,
                ) -> impl std::future::Future<
                    Output = Result<Row, sqlx::Error>,
                > + Send + 'e
                where
                    Self: Sized + 'e,
                    E: sqlx::Executor<'e, Database = sqlx::Sqlite> + Send + 'e,
                    sqlx::Sqlite: 'e,
                {
                    __SqlForgeQuery_single::fetch_one(self, executor)
                }
                fn fetch_optional<'e, E>(
                    self,
                    executor: E,
                ) -> impl std::future::Future<
                    Output = Result<Option<Row>, sqlx::Error>,
                > + Send + 'e
                where
                    Self: Sized + 'e,
                    E: sqlx::Executor<'e, Database = sqlx::Sqlite> + Send + 'e,
                    sqlx::Sqlite: 'e,
                {
                    __SqlForgeQuery_single::fetch_optional(self, executor)
                }
                fn execute<'e, E>(
                    self,
                    executor: E,
                ) -> impl std::future::Future<
                    Output = Result<
                        <sqlx::Sqlite as sqlx::Database>::QueryResult,
                        sqlx::Error,
                    >,
                > + Send + 'e
                where
                    Self: Sized + 'e,
                    E: sqlx::Executor<'e, Database = sqlx::Sqlite> + Send + 'e,
                    sqlx::Sqlite: 'e,
                {
                    __SqlForgeQuery_single::execute(self, executor)
                }
            }
            let mut __builder: sqlx::QueryBuilder<sqlx::Sqlite> = sqlx::QueryBuilder::new(
                "",
            );
            __builder.push("\n        SELECT t1.id AS field_1, ");
            match include_org {
                true => {
                    match can_read_org_name {
                        true => {
                            __builder.push("COALESCE(o.name, '') AS field_2");
                        }
                        false => {
                            __builder.push("COALESCE(t1.name, '') AS field_2");
                        }
                    }
                }
                false => {
                    __builder.push("COALESCE(t1.name, '') AS field_2");
                }
            }
            __builder.push(", ");
            match include_org {
                true => {
                    match use_org_label {
                        true => {
                            __builder.push("COALESCE('org', '') AS field_3");
                        }
                        false => {
                            __builder.push("COALESCE('user', '') AS field_3");
                        }
                    }
                }
                false => {
                    __builder.push("COALESCE('no_join', '') AS field_3");
                }
            }
            __builder.push("\n        FROM users t1\n        ");
            match include_org {
                true => {
                    __builder.push(" JOIN organisations o ON o.id = t1.id ");
                }
                false => {}
            }
            __builder.push("\n        WHERE 1 = 1\n        ");
            let __sql_forge_value_single = __SqlForgeQuery_single {
                inner: __builder,
            };
            __sql_forge_value_single
        }
            .fetch_all(&pool)
            .await
            .expect("grouped nested sections query failed");
        match (&rows.len(), &3) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::None,
                    );
                }
            }
        };
        match (&rows[0].field_1, &1) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::None,
                    );
                }
            }
        };
        match (&rows[0].field_2.as_deref(), &Some("Alice")) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::None,
                    );
                }
            }
        };
        match (&rows[0].field_3.as_deref(), &Some("org")) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::None,
                    );
                }
            }
        };
    };
    let mut body = body;
    #[allow(unused_mut)]
    let mut body = unsafe { ::tokio::macros::support::Pin::new_unchecked(&mut body) };
    let body: ::core::pin::Pin<&mut dyn ::core::future::Future<Output = ()>> = body;
    #[allow(
        clippy::expect_used,
        clippy::diverging_sub_expression,
        clippy::needless_return,
        clippy::unwrap_in_result
    )]
    {
        use tokio::runtime::Builder;
        return Builder::new_current_thread()
            .enable_all()
            .build()
            .expect("Failed building the Runtime")
            .block_on(body);
    }
}
extern crate test;
#[rustc_test_marker = "list_parameter_in_clause"]
#[doc(hidden)]
pub const list_parameter_in_clause: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("list_parameter_in_clause"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "tests/tests.rs",
        start_line: 284usize,
        start_col: 10usize,
        end_line: 284usize,
        end_col: 34usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(list_parameter_in_clause()),
    ),
};
fn list_parameter_in_clause() {
    let body = async {
        let pool = pool().await;
        let ids = ::alloc::boxed::box_assume_init_into_vec_unsafe(
            ::alloc::intrinsics::write_box_via_move(
                ::alloc::boxed::Box::new_uninit(),
                [1i64, 3, 5],
            ),
        );
        let users: Vec<User> = {
            let _sql_forge_validator = || {
                let __sql_forge_top_level_ids = &(ids);
                {
                    type __SqlForgeModel = User;
                    {
                        let __sql_forge_validator_arg_0 = (__sql_forge_top_level_ids)
                            .as_slice()
                            .first()
                            .expect(
                                "sql_forge!: list parameters used in validation must have at least one representative element",
                            );
                        let __sql_forge_validator_arg_1 = (__sql_forge_top_level_ids)
                            .as_slice()
                            .first()
                            .expect(
                                "sql_forge!: list parameters used in validation must have at least one representative element",
                            );
                        let __sql_forge_validator_arg_2 = (__sql_forge_top_level_ids)
                            .as_slice()
                            .first()
                            .expect(
                                "sql_forge!: list parameters used in validation must have at least one representative element",
                            );
                        let _ = {
                            {
                                #[allow(clippy::all)]
                                {
                                    use ::sqlx::Arguments as _;
                                    let arg0 = &(__sql_forge_validator_arg_0);
                                    let arg1 = &(__sql_forge_validator_arg_1);
                                    let arg2 = &(__sql_forge_validator_arg_2);
                                    let mut query_args = <sqlx::sqlite::Sqlite as ::sqlx::database::Database>::Arguments::default();
                                    query_args
                                        .reserve(
                                            3usize,
                                            0
                                                + ::sqlx::encode::Encode::<
                                                    sqlx::sqlite::Sqlite,
                                                >::size_hint(arg0)
                                                + ::sqlx::encode::Encode::<
                                                    sqlx::sqlite::Sqlite,
                                                >::size_hint(arg1)
                                                + ::sqlx::encode::Encode::<
                                                    sqlx::sqlite::Sqlite,
                                                >::size_hint(arg2),
                                        );
                                    let query_args = ::core::result::Result::<
                                        _,
                                        ::sqlx::error::BoxDynError,
                                    >::Ok(query_args)
                                        .and_then(move |mut query_args| {
                                            query_args.add(arg0).map(move |()| query_args)
                                        })
                                        .and_then(move |mut query_args| {
                                            query_args.add(arg1).map(move |()| query_args)
                                        })
                                        .and_then(move |mut query_args| {
                                            query_args.add(arg2).map(move |()| query_args)
                                        });
                                    ::sqlx::__query_with_result::<
                                        sqlx::sqlite::Sqlite,
                                        _,
                                    >(
                                            "SELECT id, name FROM users WHERE id IN (?, ?, ?)",
                                            query_args,
                                        )
                                        .try_map(|row: sqlx::sqlite::SqliteRow| {
                                            use ::sqlx::Row as _;
                                            #[allow(non_snake_case)]
                                            let sqlx_query_as_id = row
                                                .try_get_unchecked::<i64, _>(0usize)?
                                                .into();
                                            #[allow(non_snake_case)]
                                            let sqlx_query_as_name = row
                                                .try_get_unchecked::<String, _>(1usize)?
                                                .into();
                                            ::std::result::Result::Ok(__SqlForgeModel {
                                                id: sqlx_query_as_id,
                                                name: sqlx_query_as_name,
                                            })
                                        })
                                }
                            }
                        };
                    }
                }
            };
            struct __SqlForgeQuery_single {
                inner: sqlx::QueryBuilder<sqlx::Sqlite>,
            }
            impl __SqlForgeQuery_single {
                async fn fetch_all<'e, E>(
                    mut self,
                    executor: E,
                ) -> Result<Vec<User>, sqlx::Error>
                where
                    E: sqlx::Executor<'e, Database = sqlx::Sqlite>,
                {
                    self.inner.build_query_as::<User>().fetch_all(executor).await
                }
                async fn fetch_one<'e, E>(
                    mut self,
                    executor: E,
                ) -> Result<User, sqlx::Error>
                where
                    E: sqlx::Executor<'e, Database = sqlx::Sqlite>,
                {
                    self.inner.build_query_as::<User>().fetch_one(executor).await
                }
                async fn fetch_optional<'e, E>(
                    mut self,
                    executor: E,
                ) -> Result<Option<User>, sqlx::Error>
                where
                    E: sqlx::Executor<'e, Database = sqlx::Sqlite>,
                {
                    self.inner.build_query_as::<User>().fetch_optional(executor).await
                }
                async fn execute<'e, E>(
                    mut self,
                    executor: E,
                ) -> Result<<sqlx::Sqlite as sqlx::Database>::QueryResult, sqlx::Error>
                where
                    E: sqlx::Executor<'e, Database = sqlx::Sqlite>,
                {
                    self.inner.build().execute(executor).await
                }
            }
            impl sql_forge::SqlForgeQuery<User> for __SqlForgeQuery_single {
                type Db = sqlx::Sqlite;
                fn fetch_all<'e, E>(
                    self,
                    executor: E,
                ) -> impl std::future::Future<
                    Output = Result<Vec<User>, sqlx::Error>,
                > + Send + 'e
                where
                    Self: Sized + 'e,
                    E: sqlx::Executor<'e, Database = sqlx::Sqlite> + Send + 'e,
                    sqlx::Sqlite: 'e,
                {
                    __SqlForgeQuery_single::fetch_all(self, executor)
                }
                fn fetch_one<'e, E>(
                    self,
                    executor: E,
                ) -> impl std::future::Future<
                    Output = Result<User, sqlx::Error>,
                > + Send + 'e
                where
                    Self: Sized + 'e,
                    E: sqlx::Executor<'e, Database = sqlx::Sqlite> + Send + 'e,
                    sqlx::Sqlite: 'e,
                {
                    __SqlForgeQuery_single::fetch_one(self, executor)
                }
                fn fetch_optional<'e, E>(
                    self,
                    executor: E,
                ) -> impl std::future::Future<
                    Output = Result<Option<User>, sqlx::Error>,
                > + Send + 'e
                where
                    Self: Sized + 'e,
                    E: sqlx::Executor<'e, Database = sqlx::Sqlite> + Send + 'e,
                    sqlx::Sqlite: 'e,
                {
                    __SqlForgeQuery_single::fetch_optional(self, executor)
                }
                fn execute<'e, E>(
                    self,
                    executor: E,
                ) -> impl std::future::Future<
                    Output = Result<
                        <sqlx::Sqlite as sqlx::Database>::QueryResult,
                        sqlx::Error,
                    >,
                > + Send + 'e
                where
                    Self: Sized + 'e,
                    E: sqlx::Executor<'e, Database = sqlx::Sqlite> + Send + 'e,
                    sqlx::Sqlite: 'e,
                {
                    __SqlForgeQuery_single::execute(self, executor)
                }
            }
            let __sql_forge_runtime_ids = ids;
            let mut __builder: sqlx::QueryBuilder<sqlx::Sqlite> = sqlx::QueryBuilder::new(
                "",
            );
            __builder.push("SELECT id, name FROM users WHERE id IN (");
            let __sql_forge_values = __sql_forge_runtime_ids;
            let mut __separated = __builder.separated(", ");
            for __value in __sql_forge_values {
                __separated.push_bind(__value);
            }
            __builder.push(")");
            let __sql_forge_value_single = __SqlForgeQuery_single {
                inner: __builder,
            };
            __sql_forge_value_single
        }
            .fetch_all(&pool)
            .await
            .expect("list param query failed");
        match (&users.len(), &3) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::None,
                    );
                }
            }
        };
        match (&users[0].id, &1) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::None,
                    );
                }
            }
        };
        match (&users[1].id, &3) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::None,
                    );
                }
            }
        };
        match (&users[2].id, &5) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::None,
                    );
                }
            }
        };
    };
    let mut body = body;
    #[allow(unused_mut)]
    let mut body = unsafe { ::tokio::macros::support::Pin::new_unchecked(&mut body) };
    let body: ::core::pin::Pin<&mut dyn ::core::future::Future<Output = ()>> = body;
    #[allow(
        clippy::expect_used,
        clippy::diverging_sub_expression,
        clippy::needless_return,
        clippy::unwrap_in_result
    )]
    {
        use tokio::runtime::Builder;
        return Builder::new_current_thread()
            .enable_all()
            .build()
            .expect("Failed building the Runtime")
            .block_on(body);
    }
}
extern crate test;
#[rustc_test_marker = "list_parameter_in_main_sql_with_match_filter"]
#[doc(hidden)]
pub const list_parameter_in_main_sql_with_match_filter: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("list_parameter_in_main_sql_with_match_filter"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "tests/tests.rs",
        start_line: 305usize,
        start_col: 10usize,
        end_line: 305usize,
        end_col: 54usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(list_parameter_in_main_sql_with_match_filter()),
    ),
};
fn list_parameter_in_main_sql_with_match_filter() {
    let body = async {
        let pool = pool().await;
        let ids = ::alloc::boxed::box_assume_init_into_vec_unsafe(
            ::alloc::intrinsics::write_box_via_move(
                ::alloc::boxed::Box::new_uninit(),
                [UserId(1), UserId(3), UserId(4), UserId(5)],
            ),
        );
        let min_id = Some(3i64);
        let expected_ids = [UserId(3), UserId(4), UserId(5)];
        let users: Vec<User> = {
            let _sql_forge_validator = || {
                let __sql_forge_top_level_ids = &(ids);
                {
                    type __SqlForgeModel = User;
                    {
                        let __sql_forge_validator_arg_0 = (__sql_forge_top_level_ids)
                            .as_slice()
                            .first()
                            .expect(
                                "sql_forge!: list parameters used in validation must have at least one representative element",
                            );
                        let __sql_forge_validator_arg_1 = (__sql_forge_top_level_ids)
                            .as_slice()
                            .first()
                            .expect(
                                "sql_forge!: list parameters used in validation must have at least one representative element",
                            );
                        let __sql_forge_validator_arg_2 = (__sql_forge_top_level_ids)
                            .as_slice()
                            .first()
                            .expect(
                                "sql_forge!: list parameters used in validation must have at least one representative element",
                            );
                        let __sql_forge_section_case_single_0_filter_min_id = &(match &(min_id) {
                            Some(min_id) => {
                                let _ = &min_id;
                                min_id
                            }
                            _ => {
                                ::core::panicking::panic_fmt(
                                    format_args!(
                                        "internal error: entered unreachable code: {0}",
                                        format_args!("sql_forge!: validator arm mismatch"),
                                    ),
                                );
                            }
                        });
                        let __sql_forge_validator_arg_3 = __sql_forge_section_case_single_0_filter_min_id;
                        let _ = {
                            {
                                #[allow(clippy::all)]
                                {
                                    use ::sqlx::Arguments as _;
                                    let arg0 = &(__sql_forge_validator_arg_0);
                                    let arg1 = &(__sql_forge_validator_arg_1);
                                    let arg2 = &(__sql_forge_validator_arg_2);
                                    let arg3 = &(__sql_forge_validator_arg_3);
                                    let mut query_args = <sqlx::sqlite::Sqlite as ::sqlx::database::Database>::Arguments::default();
                                    query_args
                                        .reserve(
                                            4usize,
                                            0
                                                + ::sqlx::encode::Encode::<
                                                    sqlx::sqlite::Sqlite,
                                                >::size_hint(arg0)
                                                + ::sqlx::encode::Encode::<
                                                    sqlx::sqlite::Sqlite,
                                                >::size_hint(arg1)
                                                + ::sqlx::encode::Encode::<
                                                    sqlx::sqlite::Sqlite,
                                                >::size_hint(arg2)
                                                + ::sqlx::encode::Encode::<
                                                    sqlx::sqlite::Sqlite,
                                                >::size_hint(arg3),
                                        );
                                    let query_args = ::core::result::Result::<
                                        _,
                                        ::sqlx::error::BoxDynError,
                                    >::Ok(query_args)
                                        .and_then(move |mut query_args| {
                                            query_args.add(arg0).map(move |()| query_args)
                                        })
                                        .and_then(move |mut query_args| {
                                            query_args.add(arg1).map(move |()| query_args)
                                        })
                                        .and_then(move |mut query_args| {
                                            query_args.add(arg2).map(move |()| query_args)
                                        })
                                        .and_then(move |mut query_args| {
                                            query_args.add(arg3).map(move |()| query_args)
                                        });
                                    ::sqlx::__query_with_result::<
                                        sqlx::sqlite::Sqlite,
                                        _,
                                    >(
                                            "SELECT id, name FROM users WHERE id IN (?, ?, ?)  AND id >= ? ORDER BY id",
                                            query_args,
                                        )
                                        .try_map(|row: sqlx::sqlite::SqliteRow| {
                                            use ::sqlx::Row as _;
                                            #[allow(non_snake_case)]
                                            let sqlx_query_as_id = row
                                                .try_get_unchecked::<i64, _>(0usize)?
                                                .into();
                                            #[allow(non_snake_case)]
                                            let sqlx_query_as_name = row
                                                .try_get_unchecked::<String, _>(1usize)?
                                                .into();
                                            ::std::result::Result::Ok(__SqlForgeModel {
                                                id: sqlx_query_as_id,
                                                name: sqlx_query_as_name,
                                            })
                                        })
                                }
                            }
                        };
                    }
                    {
                        let __sql_forge_validator_arg_0 = (__sql_forge_top_level_ids)
                            .as_slice()
                            .first()
                            .expect(
                                "sql_forge!: list parameters used in validation must have at least one representative element",
                            );
                        let __sql_forge_validator_arg_1 = (__sql_forge_top_level_ids)
                            .as_slice()
                            .first()
                            .expect(
                                "sql_forge!: list parameters used in validation must have at least one representative element",
                            );
                        let __sql_forge_validator_arg_2 = (__sql_forge_top_level_ids)
                            .as_slice()
                            .first()
                            .expect(
                                "sql_forge!: list parameters used in validation must have at least one representative element",
                            );
                        let _ = {
                            {
                                #[allow(clippy::all)]
                                {
                                    use ::sqlx::Arguments as _;
                                    let arg0 = &(__sql_forge_validator_arg_0);
                                    let arg1 = &(__sql_forge_validator_arg_1);
                                    let arg2 = &(__sql_forge_validator_arg_2);
                                    let mut query_args = <sqlx::sqlite::Sqlite as ::sqlx::database::Database>::Arguments::default();
                                    query_args
                                        .reserve(
                                            3usize,
                                            0
                                                + ::sqlx::encode::Encode::<
                                                    sqlx::sqlite::Sqlite,
                                                >::size_hint(arg0)
                                                + ::sqlx::encode::Encode::<
                                                    sqlx::sqlite::Sqlite,
                                                >::size_hint(arg1)
                                                + ::sqlx::encode::Encode::<
                                                    sqlx::sqlite::Sqlite,
                                                >::size_hint(arg2),
                                        );
                                    let query_args = ::core::result::Result::<
                                        _,
                                        ::sqlx::error::BoxDynError,
                                    >::Ok(query_args)
                                        .and_then(move |mut query_args| {
                                            query_args.add(arg0).map(move |()| query_args)
                                        })
                                        .and_then(move |mut query_args| {
                                            query_args.add(arg1).map(move |()| query_args)
                                        })
                                        .and_then(move |mut query_args| {
                                            query_args.add(arg2).map(move |()| query_args)
                                        });
                                    ::sqlx::__query_with_result::<
                                        sqlx::sqlite::Sqlite,
                                        _,
                                    >(
                                            "SELECT id, name FROM users WHERE id IN (?, ?, ?)  ORDER BY id",
                                            query_args,
                                        )
                                        .try_map(|row: sqlx::sqlite::SqliteRow| {
                                            use ::sqlx::Row as _;
                                            #[allow(non_snake_case)]
                                            let sqlx_query_as_id = row
                                                .try_get_unchecked::<i64, _>(0usize)?
                                                .into();
                                            #[allow(non_snake_case)]
                                            let sqlx_query_as_name = row
                                                .try_get_unchecked::<String, _>(1usize)?
                                                .into();
                                            ::std::result::Result::Ok(__SqlForgeModel {
                                                id: sqlx_query_as_id,
                                                name: sqlx_query_as_name,
                                            })
                                        })
                                }
                            }
                        };
                    }
                }
            };
            struct __SqlForgeQuery_single {
                inner: sqlx::QueryBuilder<sqlx::Sqlite>,
            }
            impl __SqlForgeQuery_single {
                async fn fetch_all<'e, E>(
                    mut self,
                    executor: E,
                ) -> Result<Vec<User>, sqlx::Error>
                where
                    E: sqlx::Executor<'e, Database = sqlx::Sqlite>,
                {
                    self.inner.build_query_as::<User>().fetch_all(executor).await
                }
                async fn fetch_one<'e, E>(
                    mut self,
                    executor: E,
                ) -> Result<User, sqlx::Error>
                where
                    E: sqlx::Executor<'e, Database = sqlx::Sqlite>,
                {
                    self.inner.build_query_as::<User>().fetch_one(executor).await
                }
                async fn fetch_optional<'e, E>(
                    mut self,
                    executor: E,
                ) -> Result<Option<User>, sqlx::Error>
                where
                    E: sqlx::Executor<'e, Database = sqlx::Sqlite>,
                {
                    self.inner.build_query_as::<User>().fetch_optional(executor).await
                }
                async fn execute<'e, E>(
                    mut self,
                    executor: E,
                ) -> Result<<sqlx::Sqlite as sqlx::Database>::QueryResult, sqlx::Error>
                where
                    E: sqlx::Executor<'e, Database = sqlx::Sqlite>,
                {
                    self.inner.build().execute(executor).await
                }
            }
            impl sql_forge::SqlForgeQuery<User> for __SqlForgeQuery_single {
                type Db = sqlx::Sqlite;
                fn fetch_all<'e, E>(
                    self,
                    executor: E,
                ) -> impl std::future::Future<
                    Output = Result<Vec<User>, sqlx::Error>,
                > + Send + 'e
                where
                    Self: Sized + 'e,
                    E: sqlx::Executor<'e, Database = sqlx::Sqlite> + Send + 'e,
                    sqlx::Sqlite: 'e,
                {
                    __SqlForgeQuery_single::fetch_all(self, executor)
                }
                fn fetch_one<'e, E>(
                    self,
                    executor: E,
                ) -> impl std::future::Future<
                    Output = Result<User, sqlx::Error>,
                > + Send + 'e
                where
                    Self: Sized + 'e,
                    E: sqlx::Executor<'e, Database = sqlx::Sqlite> + Send + 'e,
                    sqlx::Sqlite: 'e,
                {
                    __SqlForgeQuery_single::fetch_one(self, executor)
                }
                fn fetch_optional<'e, E>(
                    self,
                    executor: E,
                ) -> impl std::future::Future<
                    Output = Result<Option<User>, sqlx::Error>,
                > + Send + 'e
                where
                    Self: Sized + 'e,
                    E: sqlx::Executor<'e, Database = sqlx::Sqlite> + Send + 'e,
                    sqlx::Sqlite: 'e,
                {
                    __SqlForgeQuery_single::fetch_optional(self, executor)
                }
                fn execute<'e, E>(
                    self,
                    executor: E,
                ) -> impl std::future::Future<
                    Output = Result<
                        <sqlx::Sqlite as sqlx::Database>::QueryResult,
                        sqlx::Error,
                    >,
                > + Send + 'e
                where
                    Self: Sized + 'e,
                    E: sqlx::Executor<'e, Database = sqlx::Sqlite> + Send + 'e,
                    sqlx::Sqlite: 'e,
                {
                    __SqlForgeQuery_single::execute(self, executor)
                }
            }
            let __sql_forge_runtime_ids = ids;
            let mut __builder: sqlx::QueryBuilder<sqlx::Sqlite> = sqlx::QueryBuilder::new(
                "",
            );
            __builder.push("SELECT id, name FROM users WHERE id IN (");
            let __sql_forge_values = __sql_forge_runtime_ids;
            let mut __separated = __builder.separated(", ");
            for __value in __sql_forge_values {
                __separated.push_bind(__value);
            }
            __builder.push(") ");
            match min_id {
                Some(min_id) => {
                    ::core::hint::black_box(&min_id);
                    {
                        let __sql_forge_section_filter_0_min_id = min_id;
                        __builder.push(" AND id >= ");
                        __builder.push_bind(__sql_forge_section_filter_0_min_id);
                    }
                }
                None => {}
            }
            __builder.push(" ORDER BY id");
            let __sql_forge_value_single = __SqlForgeQuery_single {
                inner: __builder,
            };
            __sql_forge_value_single
        }
            .fetch_all(&pool)
            .await
            .expect("list param with match filter query failed");
        match (&users.len(), &expected_ids.len()) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::None,
                    );
                }
            }
        };
        for (user, expected_id) in users.iter().zip(expected_ids) {
            match (&user.id, &expected_id.0) {
                (left_val, right_val) => {
                    if !(*left_val == *right_val) {
                        let kind = ::core::panicking::AssertKind::Eq;
                        ::core::panicking::assert_failed(
                            kind,
                            &*left_val,
                            &*right_val,
                            ::core::option::Option::None,
                        );
                    }
                }
            };
        }
    };
    let mut body = body;
    #[allow(unused_mut)]
    let mut body = unsafe { ::tokio::macros::support::Pin::new_unchecked(&mut body) };
    let body: ::core::pin::Pin<&mut dyn ::core::future::Future<Output = ()>> = body;
    #[allow(
        clippy::expect_used,
        clippy::diverging_sub_expression,
        clippy::needless_return,
        clippy::unwrap_in_result
    )]
    {
        use tokio::runtime::Builder;
        return Builder::new_current_thread()
            .enable_all()
            .build()
            .expect("Failed building the Runtime")
            .block_on(body);
    }
}
extern crate test;
#[rustc_test_marker = "list_parameter_with_empty_guard"]
#[doc(hidden)]
pub const list_parameter_with_empty_guard: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("list_parameter_with_empty_guard"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "tests/tests.rs",
        start_line: 337usize,
        start_col: 10usize,
        end_line: 337usize,
        end_col: 41usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(list_parameter_with_empty_guard()),
    ),
};
fn list_parameter_with_empty_guard() {
    let body = async {
        let pool = pool().await;
        let ids: Vec<i64> = ::alloc::vec::Vec::new();
        let users: Vec<User> = {
            let _sql_forge_validator = || {
                {
                    type __SqlForgeModel = User;
                    {
                        let _ = {
                            {
                                #[allow(clippy::all)]
                                {
                                    use ::sqlx::Arguments as _;
                                    let query_args = ::core::result::Result::<
                                        _,
                                        ::sqlx::error::BoxDynError,
                                    >::Ok(
                                        <sqlx::sqlite::Sqlite as ::sqlx::database::Database>::Arguments::default(),
                                    );
                                    ::sqlx::__query_with_result::<
                                        sqlx::sqlite::Sqlite,
                                        _,
                                    >("SELECT id, name FROM users WHERE 1 = 0", query_args)
                                        .try_map(|row: sqlx::sqlite::SqliteRow| {
                                            use ::sqlx::Row as _;
                                            #[allow(non_snake_case)]
                                            let sqlx_query_as_id = row
                                                .try_get_unchecked::<i64, _>(0usize)?
                                                .into();
                                            #[allow(non_snake_case)]
                                            let sqlx_query_as_name = row
                                                .try_get_unchecked::<String, _>(1usize)?
                                                .into();
                                            ::std::result::Result::Ok(__SqlForgeModel {
                                                id: sqlx_query_as_id,
                                                name: sqlx_query_as_name,
                                            })
                                        })
                                }
                            }
                        };
                    }
                    {
                        let __sql_forge_section_case_single_1_filter_ids = &(match &(ids
                            .is_empty())
                        {
                            false => &(ids),
                            _ => {
                                ::core::panicking::panic_fmt(
                                    format_args!(
                                        "internal error: entered unreachable code: {0}",
                                        format_args!("sql_forge!: validator arm mismatch"),
                                    ),
                                );
                            }
                        });
                        let __sql_forge_validator_arg_0 = (__sql_forge_section_case_single_1_filter_ids)
                            .as_slice()
                            .first()
                            .expect(
                                "sql_forge!: list parameters used in validation must have at least one representative element",
                            );
                        let __sql_forge_validator_arg_1 = (__sql_forge_section_case_single_1_filter_ids)
                            .as_slice()
                            .first()
                            .expect(
                                "sql_forge!: list parameters used in validation must have at least one representative element",
                            );
                        let __sql_forge_validator_arg_2 = (__sql_forge_section_case_single_1_filter_ids)
                            .as_slice()
                            .first()
                            .expect(
                                "sql_forge!: list parameters used in validation must have at least one representative element",
                            );
                        let _ = {
                            {
                                #[allow(clippy::all)]
                                {
                                    use ::sqlx::Arguments as _;
                                    let arg0 = &(__sql_forge_validator_arg_0);
                                    let arg1 = &(__sql_forge_validator_arg_1);
                                    let arg2 = &(__sql_forge_validator_arg_2);
                                    let mut query_args = <sqlx::sqlite::Sqlite as ::sqlx::database::Database>::Arguments::default();
                                    query_args
                                        .reserve(
                                            3usize,
                                            0
                                                + ::sqlx::encode::Encode::<
                                                    sqlx::sqlite::Sqlite,
                                                >::size_hint(arg0)
                                                + ::sqlx::encode::Encode::<
                                                    sqlx::sqlite::Sqlite,
                                                >::size_hint(arg1)
                                                + ::sqlx::encode::Encode::<
                                                    sqlx::sqlite::Sqlite,
                                                >::size_hint(arg2),
                                        );
                                    let query_args = ::core::result::Result::<
                                        _,
                                        ::sqlx::error::BoxDynError,
                                    >::Ok(query_args)
                                        .and_then(move |mut query_args| {
                                            query_args.add(arg0).map(move |()| query_args)
                                        })
                                        .and_then(move |mut query_args| {
                                            query_args.add(arg1).map(move |()| query_args)
                                        })
                                        .and_then(move |mut query_args| {
                                            query_args.add(arg2).map(move |()| query_args)
                                        });
                                    ::sqlx::__query_with_result::<
                                        sqlx::sqlite::Sqlite,
                                        _,
                                    >(
                                            "SELECT id, name FROM users WHERE id IN (?, ?, ?)",
                                            query_args,
                                        )
                                        .try_map(|row: sqlx::sqlite::SqliteRow| {
                                            use ::sqlx::Row as _;
                                            #[allow(non_snake_case)]
                                            let sqlx_query_as_id = row
                                                .try_get_unchecked::<i64, _>(0usize)?
                                                .into();
                                            #[allow(non_snake_case)]
                                            let sqlx_query_as_name = row
                                                .try_get_unchecked::<String, _>(1usize)?
                                                .into();
                                            ::std::result::Result::Ok(__SqlForgeModel {
                                                id: sqlx_query_as_id,
                                                name: sqlx_query_as_name,
                                            })
                                        })
                                }
                            }
                        };
                    }
                }
            };
            struct __SqlForgeQuery_single {
                inner: sqlx::QueryBuilder<sqlx::Sqlite>,
            }
            impl __SqlForgeQuery_single {
                async fn fetch_all<'e, E>(
                    mut self,
                    executor: E,
                ) -> Result<Vec<User>, sqlx::Error>
                where
                    E: sqlx::Executor<'e, Database = sqlx::Sqlite>,
                {
                    self.inner.build_query_as::<User>().fetch_all(executor).await
                }
                async fn fetch_one<'e, E>(
                    mut self,
                    executor: E,
                ) -> Result<User, sqlx::Error>
                where
                    E: sqlx::Executor<'e, Database = sqlx::Sqlite>,
                {
                    self.inner.build_query_as::<User>().fetch_one(executor).await
                }
                async fn fetch_optional<'e, E>(
                    mut self,
                    executor: E,
                ) -> Result<Option<User>, sqlx::Error>
                where
                    E: sqlx::Executor<'e, Database = sqlx::Sqlite>,
                {
                    self.inner.build_query_as::<User>().fetch_optional(executor).await
                }
                async fn execute<'e, E>(
                    mut self,
                    executor: E,
                ) -> Result<<sqlx::Sqlite as sqlx::Database>::QueryResult, sqlx::Error>
                where
                    E: sqlx::Executor<'e, Database = sqlx::Sqlite>,
                {
                    self.inner.build().execute(executor).await
                }
            }
            impl sql_forge::SqlForgeQuery<User> for __SqlForgeQuery_single {
                type Db = sqlx::Sqlite;
                fn fetch_all<'e, E>(
                    self,
                    executor: E,
                ) -> impl std::future::Future<
                    Output = Result<Vec<User>, sqlx::Error>,
                > + Send + 'e
                where
                    Self: Sized + 'e,
                    E: sqlx::Executor<'e, Database = sqlx::Sqlite> + Send + 'e,
                    sqlx::Sqlite: 'e,
                {
                    __SqlForgeQuery_single::fetch_all(self, executor)
                }
                fn fetch_one<'e, E>(
                    self,
                    executor: E,
                ) -> impl std::future::Future<
                    Output = Result<User, sqlx::Error>,
                > + Send + 'e
                where
                    Self: Sized + 'e,
                    E: sqlx::Executor<'e, Database = sqlx::Sqlite> + Send + 'e,
                    sqlx::Sqlite: 'e,
                {
                    __SqlForgeQuery_single::fetch_one(self, executor)
                }
                fn fetch_optional<'e, E>(
                    self,
                    executor: E,
                ) -> impl std::future::Future<
                    Output = Result<Option<User>, sqlx::Error>,
                > + Send + 'e
                where
                    Self: Sized + 'e,
                    E: sqlx::Executor<'e, Database = sqlx::Sqlite> + Send + 'e,
                    sqlx::Sqlite: 'e,
                {
                    __SqlForgeQuery_single::fetch_optional(self, executor)
                }
                fn execute<'e, E>(
                    self,
                    executor: E,
                ) -> impl std::future::Future<
                    Output = Result<
                        <sqlx::Sqlite as sqlx::Database>::QueryResult,
                        sqlx::Error,
                    >,
                > + Send + 'e
                where
                    Self: Sized + 'e,
                    E: sqlx::Executor<'e, Database = sqlx::Sqlite> + Send + 'e,
                    sqlx::Sqlite: 'e,
                {
                    __SqlForgeQuery_single::execute(self, executor)
                }
            }
            let mut __builder: sqlx::QueryBuilder<sqlx::Sqlite> = sqlx::QueryBuilder::new(
                "",
            );
            __builder.push("SELECT id, name FROM users WHERE ");
            match ids.is_empty() {
                true => {
                    __builder.push("1 = 0");
                }
                false => {
                    let __sql_forge_section_filter_1_ids = ids;
                    __builder.push("id IN (");
                    let __sql_forge_values = __sql_forge_section_filter_1_ids;
                    let mut __separated = __builder.separated(", ");
                    for __value in __sql_forge_values {
                        __separated.push_bind(__value);
                    }
                    __builder.push(")");
                }
            }
            let __sql_forge_value_single = __SqlForgeQuery_single {
                inner: __builder,
            };
            __sql_forge_value_single
        }
            .fetch_all(&pool)
            .await
            .expect("empty list guard query failed");
        match (&users.len(), &0) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::None,
                    );
                }
            }
        };
    };
    let mut body = body;
    #[allow(unused_mut)]
    let mut body = unsafe { ::tokio::macros::support::Pin::new_unchecked(&mut body) };
    let body: ::core::pin::Pin<&mut dyn ::core::future::Future<Output = ()>> = body;
    #[allow(
        clippy::expect_used,
        clippy::diverging_sub_expression,
        clippy::needless_return,
        clippy::unwrap_in_result
    )]
    {
        use tokio::runtime::Builder;
        return Builder::new_current_thread()
            .enable_all()
            .build()
            .expect("Failed building the Runtime")
            .block_on(body);
    }
}
extern crate test;
#[rustc_test_marker = "multiple_results_group"]
#[doc(hidden)]
pub const multiple_results_group: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("multiple_results_group"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "tests/tests.rs",
        start_line: 363usize,
        start_col: 10usize,
        end_line: 363usize,
        end_col: 32usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(multiple_results_group()),
    ),
};
fn multiple_results_group() {
    let body = async {
        let pool = pool().await;
        let category_id = 1i64;
        let min_price = price_new(10000, 2);
        let group = {
            let _sql_forge_validator = || {
                let __sql_forge_top_level_category_id = &(category_id);
                let __sql_forge_top_level_min_price = &(min_price);
                {
                    let __sql_forge_result_flag_amount: bool = true;
                    let __sql_forge_result_flag_list: bool = false;
                    type __SqlForgeModel = AmountResult;
                    {
                        let __sql_forge_validator_arg_0 = __sql_forge_top_level_category_id;
                        let __sql_forge_validator_arg_1 = __sql_forge_top_level_min_price;
                        let _ = {
                            {
                                #[allow(clippy::all)]
                                {
                                    use ::sqlx::Arguments as _;
                                    let arg0 = &(__sql_forge_validator_arg_0);
                                    let arg1 = &(__sql_forge_validator_arg_1);
                                    let mut query_args = <sqlx::sqlite::Sqlite as ::sqlx::database::Database>::Arguments::default();
                                    query_args
                                        .reserve(
                                            2usize,
                                            0
                                                + ::sqlx::encode::Encode::<
                                                    sqlx::sqlite::Sqlite,
                                                >::size_hint(arg0)
                                                + ::sqlx::encode::Encode::<
                                                    sqlx::sqlite::Sqlite,
                                                >::size_hint(arg1),
                                        );
                                    let query_args = ::core::result::Result::<
                                        _,
                                        ::sqlx::error::BoxDynError,
                                    >::Ok(query_args)
                                        .and_then(move |mut query_args| {
                                            query_args.add(arg0).map(move |()| query_args)
                                        })
                                        .and_then(move |mut query_args| {
                                            query_args.add(arg1).map(move |()| query_args)
                                        });
                                    ::sqlx::__query_with_result::<
                                        sqlx::sqlite::Sqlite,
                                        _,
                                    >(
                                            "\n        SELECT COUNT(*) AS total\n        FROM items\n        \n        WHERE items.category_id = ?\n        AND   items.price      >= ?\n        \n        ",
                                            query_args,
                                        )
                                        .try_map(|row: sqlx::sqlite::SqliteRow| {
                                            use ::sqlx::Row as _;
                                            #[allow(non_snake_case)]
                                            let sqlx_query_as_total = row
                                                .try_get_unchecked::<i64, _>(0usize)?
                                                .into();
                                            ::std::result::Result::Ok(__SqlForgeModel {
                                                total: sqlx_query_as_total,
                                            })
                                        })
                                }
                            }
                        };
                    }
                }
                {
                    let __sql_forge_result_flag_amount: bool = false;
                    let __sql_forge_result_flag_list: bool = true;
                    type __SqlForgeModel = Item;
                    {
                        let __sql_forge_validator_arg_0 = __sql_forge_top_level_category_id;
                        let __sql_forge_validator_arg_1 = __sql_forge_top_level_min_price;
                        let __sql_forge_section_case_list_0_order_limit_start = &(match &(__sql_forge_result_flag_amount) {
                            false => &(0i64),
                            _ => {
                                ::core::panicking::panic_fmt(
                                    format_args!(
                                        "internal error: entered unreachable code: {0}",
                                        format_args!("sql_forge!: validator arm mismatch"),
                                    ),
                                );
                            }
                        });
                        let __sql_forge_section_case_list_0_order_limit_limit = &(match &(__sql_forge_result_flag_amount) {
                            false => &(50i64),
                            _ => {
                                ::core::panicking::panic_fmt(
                                    format_args!(
                                        "internal error: entered unreachable code: {0}",
                                        format_args!("sql_forge!: validator arm mismatch"),
                                    ),
                                );
                            }
                        });
                        let __sql_forge_validator_arg_2 = __sql_forge_section_case_list_0_order_limit_limit;
                        let __sql_forge_validator_arg_3 = __sql_forge_section_case_list_0_order_limit_start;
                        let _ = {
                            {
                                #[allow(clippy::all)]
                                {
                                    use ::sqlx::Arguments as _;
                                    let arg0 = &(__sql_forge_validator_arg_0);
                                    let arg1 = &(__sql_forge_validator_arg_1);
                                    let arg2 = &(__sql_forge_validator_arg_2);
                                    let arg3 = &(__sql_forge_validator_arg_3);
                                    let mut query_args = <sqlx::sqlite::Sqlite as ::sqlx::database::Database>::Arguments::default();
                                    query_args
                                        .reserve(
                                            4usize,
                                            0
                                                + ::sqlx::encode::Encode::<
                                                    sqlx::sqlite::Sqlite,
                                                >::size_hint(arg0)
                                                + ::sqlx::encode::Encode::<
                                                    sqlx::sqlite::Sqlite,
                                                >::size_hint(arg1)
                                                + ::sqlx::encode::Encode::<
                                                    sqlx::sqlite::Sqlite,
                                                >::size_hint(arg2)
                                                + ::sqlx::encode::Encode::<
                                                    sqlx::sqlite::Sqlite,
                                                >::size_hint(arg3),
                                        );
                                    let query_args = ::core::result::Result::<
                                        _,
                                        ::sqlx::error::BoxDynError,
                                    >::Ok(query_args)
                                        .and_then(move |mut query_args| {
                                            query_args.add(arg0).map(move |()| query_args)
                                        })
                                        .and_then(move |mut query_args| {
                                            query_args.add(arg1).map(move |()| query_args)
                                        })
                                        .and_then(move |mut query_args| {
                                            query_args.add(arg2).map(move |()| query_args)
                                        })
                                        .and_then(move |mut query_args| {
                                            query_args.add(arg3).map(move |()| query_args)
                                        });
                                    ::sqlx::__query_with_result::<
                                        sqlx::sqlite::Sqlite,
                                        _,
                                    >(
                                            "\n        SELECT items.id, items.name, items.price, items.stock\n        FROM items\n        JOIN categories ON categories.id = items.category_id\n        WHERE items.category_id = ?\n        AND   items.price      >= ?\n        ORDER BY items.created_at DESC LIMIT ? OFFSET ?\n        ",
                                            query_args,
                                        )
                                        .try_map(|row: sqlx::sqlite::SqliteRow| {
                                            use ::sqlx::Row as _;
                                            #[allow(non_snake_case)]
                                            let sqlx_query_as_id = row
                                                .try_get_unchecked::<i64, _>(0usize)?
                                                .into();
                                            #[allow(non_snake_case)]
                                            let sqlx_query_as_name = row
                                                .try_get_unchecked::<String, _>(1usize)?
                                                .into();
                                            #[allow(non_snake_case)]
                                            let sqlx_query_as_price = row
                                                .try_get_unchecked::<i64, _>(2usize)?
                                                .into();
                                            #[allow(non_snake_case)]
                                            let sqlx_query_as_stock = row
                                                .try_get_unchecked::<i64, _>(3usize)?
                                                .into();
                                            ::std::result::Result::Ok(__SqlForgeModel {
                                                id: sqlx_query_as_id,
                                                name: sqlx_query_as_name,
                                                price: sqlx_query_as_price,
                                                stock: sqlx_query_as_stock,
                                            })
                                        })
                                }
                            }
                        };
                    }
                }
            };
            struct __SqlForgeQuery_amount {
                inner: sqlx::QueryBuilder<sqlx::Sqlite>,
            }
            impl __SqlForgeQuery_amount {
                async fn fetch_all<'e, E>(
                    mut self,
                    executor: E,
                ) -> Result<Vec<AmountResult>, sqlx::Error>
                where
                    E: sqlx::Executor<'e, Database = sqlx::Sqlite>,
                {
                    self.inner.build_query_as::<AmountResult>().fetch_all(executor).await
                }
                async fn fetch_one<'e, E>(
                    mut self,
                    executor: E,
                ) -> Result<AmountResult, sqlx::Error>
                where
                    E: sqlx::Executor<'e, Database = sqlx::Sqlite>,
                {
                    self.inner.build_query_as::<AmountResult>().fetch_one(executor).await
                }
                async fn fetch_optional<'e, E>(
                    mut self,
                    executor: E,
                ) -> Result<Option<AmountResult>, sqlx::Error>
                where
                    E: sqlx::Executor<'e, Database = sqlx::Sqlite>,
                {
                    self.inner
                        .build_query_as::<AmountResult>()
                        .fetch_optional(executor)
                        .await
                }
                async fn execute<'e, E>(
                    mut self,
                    executor: E,
                ) -> Result<<sqlx::Sqlite as sqlx::Database>::QueryResult, sqlx::Error>
                where
                    E: sqlx::Executor<'e, Database = sqlx::Sqlite>,
                {
                    self.inner.build().execute(executor).await
                }
            }
            impl sql_forge::SqlForgeQuery<AmountResult> for __SqlForgeQuery_amount {
                type Db = sqlx::Sqlite;
                fn fetch_all<'e, E>(
                    self,
                    executor: E,
                ) -> impl std::future::Future<
                    Output = Result<Vec<AmountResult>, sqlx::Error>,
                > + Send + 'e
                where
                    Self: Sized + 'e,
                    E: sqlx::Executor<'e, Database = sqlx::Sqlite> + Send + 'e,
                    sqlx::Sqlite: 'e,
                {
                    __SqlForgeQuery_amount::fetch_all(self, executor)
                }
                fn fetch_one<'e, E>(
                    self,
                    executor: E,
                ) -> impl std::future::Future<
                    Output = Result<AmountResult, sqlx::Error>,
                > + Send + 'e
                where
                    Self: Sized + 'e,
                    E: sqlx::Executor<'e, Database = sqlx::Sqlite> + Send + 'e,
                    sqlx::Sqlite: 'e,
                {
                    __SqlForgeQuery_amount::fetch_one(self, executor)
                }
                fn fetch_optional<'e, E>(
                    self,
                    executor: E,
                ) -> impl std::future::Future<
                    Output = Result<Option<AmountResult>, sqlx::Error>,
                > + Send + 'e
                where
                    Self: Sized + 'e,
                    E: sqlx::Executor<'e, Database = sqlx::Sqlite> + Send + 'e,
                    sqlx::Sqlite: 'e,
                {
                    __SqlForgeQuery_amount::fetch_optional(self, executor)
                }
                fn execute<'e, E>(
                    self,
                    executor: E,
                ) -> impl std::future::Future<
                    Output = Result<
                        <sqlx::Sqlite as sqlx::Database>::QueryResult,
                        sqlx::Error,
                    >,
                > + Send + 'e
                where
                    Self: Sized + 'e,
                    E: sqlx::Executor<'e, Database = sqlx::Sqlite> + Send + 'e,
                    sqlx::Sqlite: 'e,
                {
                    __SqlForgeQuery_amount::execute(self, executor)
                }
            }
            struct __SqlForgeQuery_list {
                inner: sqlx::QueryBuilder<sqlx::Sqlite>,
            }
            impl __SqlForgeQuery_list {
                async fn fetch_all<'e, E>(
                    mut self,
                    executor: E,
                ) -> Result<Vec<Item>, sqlx::Error>
                where
                    E: sqlx::Executor<'e, Database = sqlx::Sqlite>,
                {
                    self.inner.build_query_as::<Item>().fetch_all(executor).await
                }
                async fn fetch_one<'e, E>(
                    mut self,
                    executor: E,
                ) -> Result<Item, sqlx::Error>
                where
                    E: sqlx::Executor<'e, Database = sqlx::Sqlite>,
                {
                    self.inner.build_query_as::<Item>().fetch_one(executor).await
                }
                async fn fetch_optional<'e, E>(
                    mut self,
                    executor: E,
                ) -> Result<Option<Item>, sqlx::Error>
                where
                    E: sqlx::Executor<'e, Database = sqlx::Sqlite>,
                {
                    self.inner.build_query_as::<Item>().fetch_optional(executor).await
                }
                async fn execute<'e, E>(
                    mut self,
                    executor: E,
                ) -> Result<<sqlx::Sqlite as sqlx::Database>::QueryResult, sqlx::Error>
                where
                    E: sqlx::Executor<'e, Database = sqlx::Sqlite>,
                {
                    self.inner.build().execute(executor).await
                }
            }
            impl sql_forge::SqlForgeQuery<Item> for __SqlForgeQuery_list {
                type Db = sqlx::Sqlite;
                fn fetch_all<'e, E>(
                    self,
                    executor: E,
                ) -> impl std::future::Future<
                    Output = Result<Vec<Item>, sqlx::Error>,
                > + Send + 'e
                where
                    Self: Sized + 'e,
                    E: sqlx::Executor<'e, Database = sqlx::Sqlite> + Send + 'e,
                    sqlx::Sqlite: 'e,
                {
                    __SqlForgeQuery_list::fetch_all(self, executor)
                }
                fn fetch_one<'e, E>(
                    self,
                    executor: E,
                ) -> impl std::future::Future<
                    Output = Result<Item, sqlx::Error>,
                > + Send + 'e
                where
                    Self: Sized + 'e,
                    E: sqlx::Executor<'e, Database = sqlx::Sqlite> + Send + 'e,
                    sqlx::Sqlite: 'e,
                {
                    __SqlForgeQuery_list::fetch_one(self, executor)
                }
                fn fetch_optional<'e, E>(
                    self,
                    executor: E,
                ) -> impl std::future::Future<
                    Output = Result<Option<Item>, sqlx::Error>,
                > + Send + 'e
                where
                    Self: Sized + 'e,
                    E: sqlx::Executor<'e, Database = sqlx::Sqlite> + Send + 'e,
                    sqlx::Sqlite: 'e,
                {
                    __SqlForgeQuery_list::fetch_optional(self, executor)
                }
                fn execute<'e, E>(
                    self,
                    executor: E,
                ) -> impl std::future::Future<
                    Output = Result<
                        <sqlx::Sqlite as sqlx::Database>::QueryResult,
                        sqlx::Error,
                    >,
                > + Send + 'e
                where
                    Self: Sized + 'e,
                    E: sqlx::Executor<'e, Database = sqlx::Sqlite> + Send + 'e,
                    sqlx::Sqlite: 'e,
                {
                    __SqlForgeQuery_list::execute(self, executor)
                }
            }
            let __sql_forge_runtime_category_id = category_id;
            let __sql_forge_runtime_min_price = min_price;
            let __sql_forge_result_flag_amount: bool = true;
            let __sql_forge_result_flag_list: bool = false;
            let mut __builder: sqlx::QueryBuilder<sqlx::Sqlite> = sqlx::QueryBuilder::new(
                "",
            );
            __builder.push("\n        SELECT ");
            match __sql_forge_result_flag_amount {
                true => {
                    __builder.push("COUNT(*) AS total");
                }
                false => {
                    __builder.push("items.id, items.name, items.price, items.stock");
                }
            }
            __builder.push("\n        FROM items\n        ");
            match __sql_forge_result_flag_amount {
                true => {}
                false => {
                    __builder
                        .push("JOIN categories ON categories.id = items.category_id");
                }
            }
            __builder.push("\n        WHERE items.category_id = ");
            __builder.push_bind(__sql_forge_runtime_category_id);
            __builder.push("\n        AND   items.price      >= ");
            __builder.push_bind(__sql_forge_runtime_min_price);
            __builder.push("\n        ");
            match __sql_forge_result_flag_amount {
                true => {}
                false => {
                    let __sql_forge_section_order_limit_1_grouped_2_start = 0i64;
                    let __sql_forge_section_order_limit_1_grouped_2_limit = 50i64;
                    __builder.push("ORDER BY items.created_at DESC LIMIT ");
                    __builder
                        .push_bind(__sql_forge_section_order_limit_1_grouped_2_limit);
                    __builder.push(" OFFSET ");
                    __builder
                        .push_bind(__sql_forge_section_order_limit_1_grouped_2_start);
                }
            }
            __builder.push("\n        ");
            let __sql_forge_value_amount = __SqlForgeQuery_amount {
                inner: __builder,
            };
            let __sql_forge_runtime_category_id = category_id;
            let __sql_forge_runtime_min_price = min_price;
            let __sql_forge_result_flag_amount: bool = false;
            let __sql_forge_result_flag_list: bool = true;
            let mut __builder: sqlx::QueryBuilder<sqlx::Sqlite> = sqlx::QueryBuilder::new(
                "",
            );
            __builder.push("\n        SELECT ");
            match __sql_forge_result_flag_amount {
                true => {
                    __builder.push("COUNT(*) AS total");
                }
                false => {
                    __builder.push("items.id, items.name, items.price, items.stock");
                }
            }
            __builder.push("\n        FROM items\n        ");
            match __sql_forge_result_flag_amount {
                true => {}
                false => {
                    __builder
                        .push("JOIN categories ON categories.id = items.category_id");
                }
            }
            __builder.push("\n        WHERE items.category_id = ");
            __builder.push_bind(__sql_forge_runtime_category_id);
            __builder.push("\n        AND   items.price      >= ");
            __builder.push_bind(__sql_forge_runtime_min_price);
            __builder.push("\n        ");
            match __sql_forge_result_flag_amount {
                true => {}
                false => {
                    let __sql_forge_section_order_limit_1_grouped_2_start = 0i64;
                    let __sql_forge_section_order_limit_1_grouped_2_limit = 50i64;
                    __builder.push("ORDER BY items.created_at DESC LIMIT ");
                    __builder
                        .push_bind(__sql_forge_section_order_limit_1_grouped_2_limit);
                    __builder.push(" OFFSET ");
                    __builder
                        .push_bind(__sql_forge_section_order_limit_1_grouped_2_start);
                }
            }
            __builder.push("\n        ");
            let __sql_forge_value_list = __SqlForgeQuery_list {
                inner: __builder,
            };
            struct __SqlForgeQueryGroup {
                amount: __SqlForgeQuery_amount,
                list: __SqlForgeQuery_list,
            }
            impl __SqlForgeQueryGroup {
                pub fn into_parts(
                    self,
                ) -> (__SqlForgeQuery_amount, __SqlForgeQuery_list) {
                    (self.amount, self.list)
                }
            }
            impl sql_forge::SqlForgeQueryGroup for __SqlForgeQueryGroup {
                type Db = sqlx::Sqlite;
            }
            struct __SqlForgeQueryGroupKey_amount;
            impl sql_forge::SqlForgeQueryGroupGet<
                __SqlForgeQueryGroupKey_amount,
                AmountResult,
            > for __SqlForgeQueryGroup {
                type Query = __SqlForgeQuery_amount;
                fn get(self, _: __SqlForgeQueryGroupKey_amount) -> Self::Query {
                    self.amount
                }
            }
            struct __SqlForgeQueryGroupKey_list;
            impl sql_forge::SqlForgeQueryGroupGet<__SqlForgeQueryGroupKey_list, Item>
            for __SqlForgeQueryGroup {
                type Query = __SqlForgeQuery_list;
                fn get(self, _: __SqlForgeQueryGroupKey_list) -> Self::Query {
                    self.list
                }
            }
            __SqlForgeQueryGroup {
                amount: __sql_forge_value_amount,
                list: __sql_forge_value_list,
            }
        };
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
        if !(total.total.unwrap_or(0) >= 3) {
            {
                ::core::panicking::panic_fmt(
                    format_args!(
                        "expected at least 3 items in Electronics with price >= 100",
                    ),
                );
            }
        }
        if !(items.len() >= 3) {
            ::core::panicking::panic("assertion failed: items.len() >= 3")
        }
        match (&items[0].name, &"Monitor") {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::None,
                    );
                }
            }
        };
        match (&items[1].name, &"Headphones") {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::None,
                    );
                }
            }
        };
    };
    let mut body = body;
    #[allow(unused_mut)]
    let mut body = unsafe { ::tokio::macros::support::Pin::new_unchecked(&mut body) };
    let body: ::core::pin::Pin<&mut dyn ::core::future::Future<Output = ()>> = body;
    #[allow(
        clippy::expect_used,
        clippy::diverging_sub_expression,
        clippy::needless_return,
        clippy::unwrap_in_result
    )]
    {
        use tokio::runtime::Builder;
        return Builder::new_current_thread()
            .enable_all()
            .build()
            .expect("Failed building the Runtime")
            .block_on(body);
    }
}
extern crate test;
#[rustc_test_marker = "multiple_results_scalar_key"]
#[doc(hidden)]
pub const multiple_results_scalar_key: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("multiple_results_scalar_key"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "tests/tests.rs",
        start_line: 426usize,
        start_col: 10usize,
        end_line: 426usize,
        end_col: 37usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(multiple_results_scalar_key()),
    ),
};
fn multiple_results_scalar_key() {
    let body = async {
        let pool = pool().await;
        let category_id = 2i64;
        let group = {
            let _sql_forge_validator = || {
                let __sql_forge_top_level_category_id = &(category_id);
                {
                    let __sql_forge_result_flag_amount: bool = true;
                    let __sql_forge_result_flag_first_name: bool = false;
                    {
                        let __sql_forge_validator_arg_0 = __sql_forge_top_level_category_id;
                        let _ = {
                            #[allow(clippy::all)]
                            {
                                use ::sqlx::Arguments as _;
                                let arg0 = &(__sql_forge_validator_arg_0);
                                let mut query_args = <sqlx::sqlite::Sqlite as ::sqlx::database::Database>::Arguments::default();
                                query_args
                                    .reserve(
                                        1usize,
                                        0
                                            + ::sqlx::encode::Encode::<
                                                sqlx::sqlite::Sqlite,
                                            >::size_hint(arg0),
                                    );
                                let query_args = ::core::result::Result::<
                                    _,
                                    ::sqlx::error::BoxDynError,
                                >::Ok(query_args)
                                    .and_then(move |mut query_args| {
                                        query_args.add(arg0).map(move |()| query_args)
                                    });
                                ::sqlx::__query_scalar_with_result::<
                                    sqlx::sqlite::Sqlite,
                                    i64,
                                    _,
                                >(
                                    "\n        SELECT COUNT(*)\n        FROM items\n        WHERE items.category_id = ?\n        ",
                                    query_args,
                                )
                            }
                        };
                    }
                }
                {
                    let __sql_forge_result_flag_amount: bool = false;
                    let __sql_forge_result_flag_first_name: bool = true;
                    {
                        let __sql_forge_validator_arg_0 = __sql_forge_top_level_category_id;
                        let _ = {
                            #[allow(clippy::all)]
                            {
                                use ::sqlx::Arguments as _;
                                let arg0 = &(__sql_forge_validator_arg_0);
                                let mut query_args = <sqlx::sqlite::Sqlite as ::sqlx::database::Database>::Arguments::default();
                                query_args
                                    .reserve(
                                        1usize,
                                        0
                                            + ::sqlx::encode::Encode::<
                                                sqlx::sqlite::Sqlite,
                                            >::size_hint(arg0),
                                    );
                                let query_args = ::core::result::Result::<
                                    _,
                                    ::sqlx::error::BoxDynError,
                                >::Ok(query_args)
                                    .and_then(move |mut query_args| {
                                        query_args.add(arg0).map(move |()| query_args)
                                    });
                                ::sqlx::__query_scalar_with_result::<
                                    sqlx::sqlite::Sqlite,
                                    String,
                                    _,
                                >(
                                    "\n        SELECT items.name\n        FROM items\n        WHERE items.category_id = ?\n        ",
                                    query_args,
                                )
                            }
                        };
                    }
                }
            };
            struct __SqlForgeQuery_amount {
                inner: sqlx::QueryBuilder<sqlx::Sqlite>,
            }
            impl __SqlForgeQuery_amount {
                async fn fetch_all<'e, E>(
                    mut self,
                    executor: E,
                ) -> Result<Vec<i64>, sqlx::Error>
                where
                    E: sqlx::Executor<'e, Database = sqlx::Sqlite>,
                {
                    self.inner.build_query_scalar::<i64>().fetch_all(executor).await
                }
                async fn fetch_one<'e, E>(
                    mut self,
                    executor: E,
                ) -> Result<i64, sqlx::Error>
                where
                    E: sqlx::Executor<'e, Database = sqlx::Sqlite>,
                {
                    self.inner.build_query_scalar::<i64>().fetch_one(executor).await
                }
                async fn fetch_optional<'e, E>(
                    mut self,
                    executor: E,
                ) -> Result<Option<i64>, sqlx::Error>
                where
                    E: sqlx::Executor<'e, Database = sqlx::Sqlite>,
                {
                    self.inner.build_query_scalar::<i64>().fetch_optional(executor).await
                }
                async fn execute<'e, E>(
                    mut self,
                    executor: E,
                ) -> Result<<sqlx::Sqlite as sqlx::Database>::QueryResult, sqlx::Error>
                where
                    E: sqlx::Executor<'e, Database = sqlx::Sqlite>,
                {
                    self.inner.build().execute(executor).await
                }
            }
            impl sql_forge::SqlForgeQuery<i64> for __SqlForgeQuery_amount {
                type Db = sqlx::Sqlite;
                fn fetch_all<'e, E>(
                    self,
                    executor: E,
                ) -> impl std::future::Future<
                    Output = Result<Vec<i64>, sqlx::Error>,
                > + Send + 'e
                where
                    Self: Sized + 'e,
                    E: sqlx::Executor<'e, Database = sqlx::Sqlite> + Send + 'e,
                    sqlx::Sqlite: 'e,
                {
                    __SqlForgeQuery_amount::fetch_all(self, executor)
                }
                fn fetch_one<'e, E>(
                    self,
                    executor: E,
                ) -> impl std::future::Future<
                    Output = Result<i64, sqlx::Error>,
                > + Send + 'e
                where
                    Self: Sized + 'e,
                    E: sqlx::Executor<'e, Database = sqlx::Sqlite> + Send + 'e,
                    sqlx::Sqlite: 'e,
                {
                    __SqlForgeQuery_amount::fetch_one(self, executor)
                }
                fn fetch_optional<'e, E>(
                    self,
                    executor: E,
                ) -> impl std::future::Future<
                    Output = Result<Option<i64>, sqlx::Error>,
                > + Send + 'e
                where
                    Self: Sized + 'e,
                    E: sqlx::Executor<'e, Database = sqlx::Sqlite> + Send + 'e,
                    sqlx::Sqlite: 'e,
                {
                    __SqlForgeQuery_amount::fetch_optional(self, executor)
                }
                fn execute<'e, E>(
                    self,
                    executor: E,
                ) -> impl std::future::Future<
                    Output = Result<
                        <sqlx::Sqlite as sqlx::Database>::QueryResult,
                        sqlx::Error,
                    >,
                > + Send + 'e
                where
                    Self: Sized + 'e,
                    E: sqlx::Executor<'e, Database = sqlx::Sqlite> + Send + 'e,
                    sqlx::Sqlite: 'e,
                {
                    __SqlForgeQuery_amount::execute(self, executor)
                }
            }
            struct __SqlForgeQuery_first_name {
                inner: sqlx::QueryBuilder<sqlx::Sqlite>,
            }
            impl __SqlForgeQuery_first_name {
                async fn fetch_all<'e, E>(
                    mut self,
                    executor: E,
                ) -> Result<Vec<String>, sqlx::Error>
                where
                    E: sqlx::Executor<'e, Database = sqlx::Sqlite>,
                {
                    self.inner.build_query_scalar::<String>().fetch_all(executor).await
                }
                async fn fetch_one<'e, E>(
                    mut self,
                    executor: E,
                ) -> Result<String, sqlx::Error>
                where
                    E: sqlx::Executor<'e, Database = sqlx::Sqlite>,
                {
                    self.inner.build_query_scalar::<String>().fetch_one(executor).await
                }
                async fn fetch_optional<'e, E>(
                    mut self,
                    executor: E,
                ) -> Result<Option<String>, sqlx::Error>
                where
                    E: sqlx::Executor<'e, Database = sqlx::Sqlite>,
                {
                    self.inner
                        .build_query_scalar::<String>()
                        .fetch_optional(executor)
                        .await
                }
                async fn execute<'e, E>(
                    mut self,
                    executor: E,
                ) -> Result<<sqlx::Sqlite as sqlx::Database>::QueryResult, sqlx::Error>
                where
                    E: sqlx::Executor<'e, Database = sqlx::Sqlite>,
                {
                    self.inner.build().execute(executor).await
                }
            }
            impl sql_forge::SqlForgeQuery<String> for __SqlForgeQuery_first_name {
                type Db = sqlx::Sqlite;
                fn fetch_all<'e, E>(
                    self,
                    executor: E,
                ) -> impl std::future::Future<
                    Output = Result<Vec<String>, sqlx::Error>,
                > + Send + 'e
                where
                    Self: Sized + 'e,
                    E: sqlx::Executor<'e, Database = sqlx::Sqlite> + Send + 'e,
                    sqlx::Sqlite: 'e,
                {
                    __SqlForgeQuery_first_name::fetch_all(self, executor)
                }
                fn fetch_one<'e, E>(
                    self,
                    executor: E,
                ) -> impl std::future::Future<
                    Output = Result<String, sqlx::Error>,
                > + Send + 'e
                where
                    Self: Sized + 'e,
                    E: sqlx::Executor<'e, Database = sqlx::Sqlite> + Send + 'e,
                    sqlx::Sqlite: 'e,
                {
                    __SqlForgeQuery_first_name::fetch_one(self, executor)
                }
                fn fetch_optional<'e, E>(
                    self,
                    executor: E,
                ) -> impl std::future::Future<
                    Output = Result<Option<String>, sqlx::Error>,
                > + Send + 'e
                where
                    Self: Sized + 'e,
                    E: sqlx::Executor<'e, Database = sqlx::Sqlite> + Send + 'e,
                    sqlx::Sqlite: 'e,
                {
                    __SqlForgeQuery_first_name::fetch_optional(self, executor)
                }
                fn execute<'e, E>(
                    self,
                    executor: E,
                ) -> impl std::future::Future<
                    Output = Result<
                        <sqlx::Sqlite as sqlx::Database>::QueryResult,
                        sqlx::Error,
                    >,
                > + Send + 'e
                where
                    Self: Sized + 'e,
                    E: sqlx::Executor<'e, Database = sqlx::Sqlite> + Send + 'e,
                    sqlx::Sqlite: 'e,
                {
                    __SqlForgeQuery_first_name::execute(self, executor)
                }
            }
            let __sql_forge_runtime_category_id = category_id;
            let __sql_forge_result_flag_amount: bool = true;
            let __sql_forge_result_flag_first_name: bool = false;
            let mut __builder: sqlx::QueryBuilder<sqlx::Sqlite> = sqlx::QueryBuilder::new(
                "",
            );
            __builder.push("\n        SELECT ");
            match __sql_forge_result_flag_amount {
                true => {
                    __builder.push("COUNT(*)");
                }
                false => {
                    __builder.push("items.name");
                }
            }
            __builder.push("\n        FROM items\n        WHERE items.category_id = ");
            __builder.push_bind(__sql_forge_runtime_category_id);
            __builder.push("\n        ");
            let __sql_forge_value_amount = __SqlForgeQuery_amount {
                inner: __builder,
            };
            let __sql_forge_runtime_category_id = category_id;
            let __sql_forge_result_flag_amount: bool = false;
            let __sql_forge_result_flag_first_name: bool = true;
            let mut __builder: sqlx::QueryBuilder<sqlx::Sqlite> = sqlx::QueryBuilder::new(
                "",
            );
            __builder.push("\n        SELECT ");
            match __sql_forge_result_flag_amount {
                true => {
                    __builder.push("COUNT(*)");
                }
                false => {
                    __builder.push("items.name");
                }
            }
            __builder.push("\n        FROM items\n        WHERE items.category_id = ");
            __builder.push_bind(__sql_forge_runtime_category_id);
            __builder.push("\n        ");
            let __sql_forge_value_first_name = __SqlForgeQuery_first_name {
                inner: __builder,
            };
            struct __SqlForgeQueryGroup {
                amount: __SqlForgeQuery_amount,
                first_name: __SqlForgeQuery_first_name,
            }
            impl __SqlForgeQueryGroup {
                pub fn into_parts(
                    self,
                ) -> (__SqlForgeQuery_amount, __SqlForgeQuery_first_name) {
                    (self.amount, self.first_name)
                }
            }
            impl sql_forge::SqlForgeQueryGroup for __SqlForgeQueryGroup {
                type Db = sqlx::Sqlite;
            }
            struct __SqlForgeQueryGroupKey_amount;
            impl sql_forge::SqlForgeQueryGroupGet<__SqlForgeQueryGroupKey_amount, i64>
            for __SqlForgeQueryGroup {
                type Query = __SqlForgeQuery_amount;
                fn get(self, _: __SqlForgeQueryGroupKey_amount) -> Self::Query {
                    self.amount
                }
            }
            struct __SqlForgeQueryGroupKey_first_name;
            impl sql_forge::SqlForgeQueryGroupGet<
                __SqlForgeQueryGroupKey_first_name,
                String,
            > for __SqlForgeQueryGroup {
                type Query = __SqlForgeQuery_first_name;
                fn get(self, _: __SqlForgeQueryGroupKey_first_name) -> Self::Query {
                    self.first_name
                }
            }
            __SqlForgeQueryGroup {
                amount: __sql_forge_value_amount,
                first_name: __sql_forge_value_first_name,
            }
        };
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
        match (&count, &1) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::None,
                    );
                }
            }
        };
        match (&first_name, &"Rust Book") {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::None,
                    );
                }
            }
        };
    };
    let mut body = body;
    #[allow(unused_mut)]
    let mut body = unsafe { ::tokio::macros::support::Pin::new_unchecked(&mut body) };
    let body: ::core::pin::Pin<&mut dyn ::core::future::Future<Output = ()>> = body;
    #[allow(
        clippy::expect_used,
        clippy::diverging_sub_expression,
        clippy::needless_return,
        clippy::unwrap_in_result
    )]
    {
        use tokio::runtime::Builder;
        return Builder::new_current_thread()
            .enable_all()
            .build()
            .expect("Failed building the Runtime")
            .block_on(body);
    }
}
extern crate test;
#[rustc_test_marker = "combining_features_example"]
#[doc(hidden)]
pub const combining_features_example: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("combining_features_example"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "tests/tests.rs",
        start_line: 467usize,
        start_col: 10usize,
        end_line: 467usize,
        end_col: 36usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(combining_features_example()),
    ),
};
#[allow(clippy::unnecessary_literal_unwrap)]
fn combining_features_example() {
    let body = async {
        let pool = pool().await;
        let category = Some("Electronics");
        let price_min = Some(price_new(5000, 2));
        let price_max: Option<Price> = None;
        let in_stock_only = true;
        let order_by = Some("price_desc".to_string());
        let page: i64 = 0;
        let page_size = Some(10i64);
        let products: Vec<Product> = {
            let _sql_forge_validator = || {
                {
                    type __SqlForgeModel = Product;
                    {
                        let __sql_forge_section_case_single_0_filter_category_cat = &(match &(category
                            .is_some())
                        {
                            true => &(category.unwrap()),
                            _ => {
                                ::core::panicking::panic_fmt(
                                    format_args!(
                                        "internal error: entered unreachable code: {0}",
                                        format_args!("sql_forge!: validator arm mismatch"),
                                    ),
                                );
                            }
                        });
                        let __sql_forge_validator_arg_0 = __sql_forge_section_case_single_0_filter_category_cat;
                        let __sql_forge_section_case_single_0_filter_price_min_price_min = &(match &(price_min
                            .is_some())
                        {
                            true => &(price_min.unwrap()),
                            _ => {
                                ::core::panicking::panic_fmt(
                                    format_args!(
                                        "internal error: entered unreachable code: {0}",
                                        format_args!("sql_forge!: validator arm mismatch"),
                                    ),
                                );
                            }
                        });
                        let __sql_forge_validator_arg_1 = __sql_forge_section_case_single_0_filter_price_min_price_min;
                        let __sql_forge_section_case_single_0_filter_price_max_price_max = &(match &(price_max
                            .is_some())
                        {
                            true => &(price_max.unwrap()),
                            _ => {
                                ::core::panicking::panic_fmt(
                                    format_args!(
                                        "internal error: entered unreachable code: {0}",
                                        format_args!("sql_forge!: validator arm mismatch"),
                                    ),
                                );
                            }
                        });
                        let __sql_forge_validator_arg_2 = __sql_forge_section_case_single_0_filter_price_max_price_max;
                        let __sql_forge_section_case_single_0_limit_offset = &(match &(page_size
                            .is_some())
                        {
                            true => &(page * page_size.unwrap()),
                            _ => {
                                ::core::panicking::panic_fmt(
                                    format_args!(
                                        "internal error: entered unreachable code: {0}",
                                        format_args!("sql_forge!: validator arm mismatch"),
                                    ),
                                );
                            }
                        });
                        let __sql_forge_section_case_single_0_limit_size = &(match &(page_size
                            .is_some())
                        {
                            true => &(page_size.unwrap()),
                            _ => {
                                ::core::panicking::panic_fmt(
                                    format_args!(
                                        "internal error: entered unreachable code: {0}",
                                        format_args!("sql_forge!: validator arm mismatch"),
                                    ),
                                );
                            }
                        });
                        let __sql_forge_validator_arg_3 = __sql_forge_section_case_single_0_limit_size;
                        let __sql_forge_validator_arg_4 = __sql_forge_section_case_single_0_limit_offset;
                        let _ = {
                            {
                                #[allow(clippy::all)]
                                {
                                    use ::sqlx::Arguments as _;
                                    let arg0 = &(__sql_forge_validator_arg_0);
                                    let arg1 = &(__sql_forge_validator_arg_1);
                                    let arg2 = &(__sql_forge_validator_arg_2);
                                    let arg3 = &(__sql_forge_validator_arg_3);
                                    let arg4 = &(__sql_forge_validator_arg_4);
                                    let mut query_args = <sqlx::sqlite::Sqlite as ::sqlx::database::Database>::Arguments::default();
                                    query_args
                                        .reserve(
                                            5usize,
                                            0
                                                + ::sqlx::encode::Encode::<
                                                    sqlx::sqlite::Sqlite,
                                                >::size_hint(arg0)
                                                + ::sqlx::encode::Encode::<
                                                    sqlx::sqlite::Sqlite,
                                                >::size_hint(arg1)
                                                + ::sqlx::encode::Encode::<
                                                    sqlx::sqlite::Sqlite,
                                                >::size_hint(arg2)
                                                + ::sqlx::encode::Encode::<
                                                    sqlx::sqlite::Sqlite,
                                                >::size_hint(arg3)
                                                + ::sqlx::encode::Encode::<
                                                    sqlx::sqlite::Sqlite,
                                                >::size_hint(arg4),
                                        );
                                    let query_args = ::core::result::Result::<
                                        _,
                                        ::sqlx::error::BoxDynError,
                                    >::Ok(query_args)
                                        .and_then(move |mut query_args| {
                                            query_args.add(arg0).map(move |()| query_args)
                                        })
                                        .and_then(move |mut query_args| {
                                            query_args.add(arg1).map(move |()| query_args)
                                        })
                                        .and_then(move |mut query_args| {
                                            query_args.add(arg2).map(move |()| query_args)
                                        })
                                        .and_then(move |mut query_args| {
                                            query_args.add(arg3).map(move |()| query_args)
                                        })
                                        .and_then(move |mut query_args| {
                                            query_args.add(arg4).map(move |()| query_args)
                                        });
                                    ::sqlx::__query_with_result::<
                                        sqlx::sqlite::Sqlite,
                                        _,
                                    >(
                                            "\n        SELECT\n            p.id,\n            p.name,\n            p.price,\n            p.stock,\n            p.category\n        FROM products p\n        WHERE 1 = 1\n         AND p.category = ? \n         AND p.price >= ? \n         AND p.price <= ? \n         AND p.stock > 0 \n         ORDER BY p.price ASC \n         LIMIT ? OFFSET ? \n        ",
                                            query_args,
                                        )
                                        .try_map(|row: sqlx::sqlite::SqliteRow| {
                                            use ::sqlx::Row as _;
                                            #[allow(non_snake_case)]
                                            let sqlx_query_as_id = row
                                                .try_get_unchecked::<i64, _>(0usize)?
                                                .into();
                                            #[allow(non_snake_case)]
                                            let sqlx_query_as_name = row
                                                .try_get_unchecked::<String, _>(1usize)?
                                                .into();
                                            #[allow(non_snake_case)]
                                            let sqlx_query_as_price = row
                                                .try_get_unchecked::<i64, _>(2usize)?
                                                .into();
                                            #[allow(non_snake_case)]
                                            let sqlx_query_as_stock = row
                                                .try_get_unchecked::<i64, _>(3usize)?
                                                .into();
                                            #[allow(non_snake_case)]
                                            let sqlx_query_as_category = row
                                                .try_get_unchecked::<String, _>(4usize)?
                                                .into();
                                            ::std::result::Result::Ok(__SqlForgeModel {
                                                id: sqlx_query_as_id,
                                                name: sqlx_query_as_name,
                                                price: sqlx_query_as_price,
                                                stock: sqlx_query_as_stock,
                                                category: sqlx_query_as_category,
                                            })
                                        })
                                }
                            }
                        };
                    }
                    {
                        let _ = {
                            {
                                #[allow(clippy::all)]
                                {
                                    use ::sqlx::Arguments as _;
                                    let query_args = ::core::result::Result::<
                                        _,
                                        ::sqlx::error::BoxDynError,
                                    >::Ok(
                                        <sqlx::sqlite::Sqlite as ::sqlx::database::Database>::Arguments::default(),
                                    );
                                    ::sqlx::__query_with_result::<
                                        sqlx::sqlite::Sqlite,
                                        _,
                                    >(
                                            "\n        SELECT\n            p.id,\n            p.name,\n            p.price,\n            p.stock,\n            p.category\n        FROM products p\n        WHERE 1 = 1\n        \n        \n        \n        \n         ORDER BY p.price DESC \n        \n        ",
                                            query_args,
                                        )
                                        .try_map(|row: sqlx::sqlite::SqliteRow| {
                                            use ::sqlx::Row as _;
                                            #[allow(non_snake_case)]
                                            let sqlx_query_as_id = row
                                                .try_get_unchecked::<i64, _>(0usize)?
                                                .into();
                                            #[allow(non_snake_case)]
                                            let sqlx_query_as_name = row
                                                .try_get_unchecked::<String, _>(1usize)?
                                                .into();
                                            #[allow(non_snake_case)]
                                            let sqlx_query_as_price = row
                                                .try_get_unchecked::<i64, _>(2usize)?
                                                .into();
                                            #[allow(non_snake_case)]
                                            let sqlx_query_as_stock = row
                                                .try_get_unchecked::<i64, _>(3usize)?
                                                .into();
                                            #[allow(non_snake_case)]
                                            let sqlx_query_as_category = row
                                                .try_get_unchecked::<String, _>(4usize)?
                                                .into();
                                            ::std::result::Result::Ok(__SqlForgeModel {
                                                id: sqlx_query_as_id,
                                                name: sqlx_query_as_name,
                                                price: sqlx_query_as_price,
                                                stock: sqlx_query_as_stock,
                                                category: sqlx_query_as_category,
                                            })
                                        })
                                }
                            }
                        };
                    }
                    {
                        let __sql_forge_section_case_single_2_filter_category_cat = &(match &(category
                            .is_some())
                        {
                            true => &(category.unwrap()),
                            _ => {
                                ::core::panicking::panic_fmt(
                                    format_args!(
                                        "internal error: entered unreachable code: {0}",
                                        format_args!("sql_forge!: validator arm mismatch"),
                                    ),
                                );
                            }
                        });
                        let __sql_forge_validator_arg_0 = __sql_forge_section_case_single_2_filter_category_cat;
                        let __sql_forge_section_case_single_2_filter_price_min_price_min = &(match &(price_min
                            .is_some())
                        {
                            true => &(price_min.unwrap()),
                            _ => {
                                ::core::panicking::panic_fmt(
                                    format_args!(
                                        "internal error: entered unreachable code: {0}",
                                        format_args!("sql_forge!: validator arm mismatch"),
                                    ),
                                );
                            }
                        });
                        let __sql_forge_validator_arg_1 = __sql_forge_section_case_single_2_filter_price_min_price_min;
                        let __sql_forge_section_case_single_2_filter_price_max_price_max = &(match &(price_max
                            .is_some())
                        {
                            true => &(price_max.unwrap()),
                            _ => {
                                ::core::panicking::panic_fmt(
                                    format_args!(
                                        "internal error: entered unreachable code: {0}",
                                        format_args!("sql_forge!: validator arm mismatch"),
                                    ),
                                );
                            }
                        });
                        let __sql_forge_validator_arg_2 = __sql_forge_section_case_single_2_filter_price_max_price_max;
                        let __sql_forge_section_case_single_2_limit_offset = &(match &(page_size
                            .is_some())
                        {
                            true => &(page * page_size.unwrap()),
                            _ => {
                                ::core::panicking::panic_fmt(
                                    format_args!(
                                        "internal error: entered unreachable code: {0}",
                                        format_args!("sql_forge!: validator arm mismatch"),
                                    ),
                                );
                            }
                        });
                        let __sql_forge_section_case_single_2_limit_size = &(match &(page_size
                            .is_some())
                        {
                            true => &(page_size.unwrap()),
                            _ => {
                                ::core::panicking::panic_fmt(
                                    format_args!(
                                        "internal error: entered unreachable code: {0}",
                                        format_args!("sql_forge!: validator arm mismatch"),
                                    ),
                                );
                            }
                        });
                        let __sql_forge_validator_arg_3 = __sql_forge_section_case_single_2_limit_size;
                        let __sql_forge_validator_arg_4 = __sql_forge_section_case_single_2_limit_offset;
                        let _ = {
                            {
                                #[allow(clippy::all)]
                                {
                                    use ::sqlx::Arguments as _;
                                    let arg0 = &(__sql_forge_validator_arg_0);
                                    let arg1 = &(__sql_forge_validator_arg_1);
                                    let arg2 = &(__sql_forge_validator_arg_2);
                                    let arg3 = &(__sql_forge_validator_arg_3);
                                    let arg4 = &(__sql_forge_validator_arg_4);
                                    let mut query_args = <sqlx::sqlite::Sqlite as ::sqlx::database::Database>::Arguments::default();
                                    query_args
                                        .reserve(
                                            5usize,
                                            0
                                                + ::sqlx::encode::Encode::<
                                                    sqlx::sqlite::Sqlite,
                                                >::size_hint(arg0)
                                                + ::sqlx::encode::Encode::<
                                                    sqlx::sqlite::Sqlite,
                                                >::size_hint(arg1)
                                                + ::sqlx::encode::Encode::<
                                                    sqlx::sqlite::Sqlite,
                                                >::size_hint(arg2)
                                                + ::sqlx::encode::Encode::<
                                                    sqlx::sqlite::Sqlite,
                                                >::size_hint(arg3)
                                                + ::sqlx::encode::Encode::<
                                                    sqlx::sqlite::Sqlite,
                                                >::size_hint(arg4),
                                        );
                                    let query_args = ::core::result::Result::<
                                        _,
                                        ::sqlx::error::BoxDynError,
                                    >::Ok(query_args)
                                        .and_then(move |mut query_args| {
                                            query_args.add(arg0).map(move |()| query_args)
                                        })
                                        .and_then(move |mut query_args| {
                                            query_args.add(arg1).map(move |()| query_args)
                                        })
                                        .and_then(move |mut query_args| {
                                            query_args.add(arg2).map(move |()| query_args)
                                        })
                                        .and_then(move |mut query_args| {
                                            query_args.add(arg3).map(move |()| query_args)
                                        })
                                        .and_then(move |mut query_args| {
                                            query_args.add(arg4).map(move |()| query_args)
                                        });
                                    ::sqlx::__query_with_result::<
                                        sqlx::sqlite::Sqlite,
                                        _,
                                    >(
                                            "\n        SELECT\n            p.id,\n            p.name,\n            p.price,\n            p.stock,\n            p.category\n        FROM products p\n        WHERE 1 = 1\n         AND p.category = ? \n         AND p.price >= ? \n         AND p.price <= ? \n         AND p.stock > 0 \n         ORDER BY p.id ASC \n         LIMIT ? OFFSET ? \n        ",
                                            query_args,
                                        )
                                        .try_map(|row: sqlx::sqlite::SqliteRow| {
                                            use ::sqlx::Row as _;
                                            #[allow(non_snake_case)]
                                            let sqlx_query_as_id = row
                                                .try_get_unchecked::<i64, _>(0usize)?
                                                .into();
                                            #[allow(non_snake_case)]
                                            let sqlx_query_as_name = row
                                                .try_get_unchecked::<String, _>(1usize)?
                                                .into();
                                            #[allow(non_snake_case)]
                                            let sqlx_query_as_price = row
                                                .try_get_unchecked::<i64, _>(2usize)?
                                                .into();
                                            #[allow(non_snake_case)]
                                            let sqlx_query_as_stock = row
                                                .try_get_unchecked::<i64, _>(3usize)?
                                                .into();
                                            #[allow(non_snake_case)]
                                            let sqlx_query_as_category = row
                                                .try_get_unchecked::<String, _>(4usize)?
                                                .into();
                                            ::std::result::Result::Ok(__SqlForgeModel {
                                                id: sqlx_query_as_id,
                                                name: sqlx_query_as_name,
                                                price: sqlx_query_as_price,
                                                stock: sqlx_query_as_stock,
                                                category: sqlx_query_as_category,
                                            })
                                        })
                                }
                            }
                        };
                    }
                }
            };
            struct __SqlForgeQuery_single {
                inner: sqlx::QueryBuilder<sqlx::Sqlite>,
            }
            impl __SqlForgeQuery_single {
                async fn fetch_all<'e, E>(
                    mut self,
                    executor: E,
                ) -> Result<Vec<Product>, sqlx::Error>
                where
                    E: sqlx::Executor<'e, Database = sqlx::Sqlite>,
                {
                    self.inner.build_query_as::<Product>().fetch_all(executor).await
                }
                async fn fetch_one<'e, E>(
                    mut self,
                    executor: E,
                ) -> Result<Product, sqlx::Error>
                where
                    E: sqlx::Executor<'e, Database = sqlx::Sqlite>,
                {
                    self.inner.build_query_as::<Product>().fetch_one(executor).await
                }
                async fn fetch_optional<'e, E>(
                    mut self,
                    executor: E,
                ) -> Result<Option<Product>, sqlx::Error>
                where
                    E: sqlx::Executor<'e, Database = sqlx::Sqlite>,
                {
                    self.inner.build_query_as::<Product>().fetch_optional(executor).await
                }
                async fn execute<'e, E>(
                    mut self,
                    executor: E,
                ) -> Result<<sqlx::Sqlite as sqlx::Database>::QueryResult, sqlx::Error>
                where
                    E: sqlx::Executor<'e, Database = sqlx::Sqlite>,
                {
                    self.inner.build().execute(executor).await
                }
            }
            impl sql_forge::SqlForgeQuery<Product> for __SqlForgeQuery_single {
                type Db = sqlx::Sqlite;
                fn fetch_all<'e, E>(
                    self,
                    executor: E,
                ) -> impl std::future::Future<
                    Output = Result<Vec<Product>, sqlx::Error>,
                > + Send + 'e
                where
                    Self: Sized + 'e,
                    E: sqlx::Executor<'e, Database = sqlx::Sqlite> + Send + 'e,
                    sqlx::Sqlite: 'e,
                {
                    __SqlForgeQuery_single::fetch_all(self, executor)
                }
                fn fetch_one<'e, E>(
                    self,
                    executor: E,
                ) -> impl std::future::Future<
                    Output = Result<Product, sqlx::Error>,
                > + Send + 'e
                where
                    Self: Sized + 'e,
                    E: sqlx::Executor<'e, Database = sqlx::Sqlite> + Send + 'e,
                    sqlx::Sqlite: 'e,
                {
                    __SqlForgeQuery_single::fetch_one(self, executor)
                }
                fn fetch_optional<'e, E>(
                    self,
                    executor: E,
                ) -> impl std::future::Future<
                    Output = Result<Option<Product>, sqlx::Error>,
                > + Send + 'e
                where
                    Self: Sized + 'e,
                    E: sqlx::Executor<'e, Database = sqlx::Sqlite> + Send + 'e,
                    sqlx::Sqlite: 'e,
                {
                    __SqlForgeQuery_single::fetch_optional(self, executor)
                }
                fn execute<'e, E>(
                    self,
                    executor: E,
                ) -> impl std::future::Future<
                    Output = Result<
                        <sqlx::Sqlite as sqlx::Database>::QueryResult,
                        sqlx::Error,
                    >,
                > + Send + 'e
                where
                    Self: Sized + 'e,
                    E: sqlx::Executor<'e, Database = sqlx::Sqlite> + Send + 'e,
                    sqlx::Sqlite: 'e,
                {
                    __SqlForgeQuery_single::execute(self, executor)
                }
            }
            let mut __builder: sqlx::QueryBuilder<sqlx::Sqlite> = sqlx::QueryBuilder::new(
                "",
            );
            __builder
                .push(
                    "\n        SELECT\n            p.id,\n            p.name,\n            p.price,\n            p.stock,\n            p.category\n        FROM products p\n        WHERE 1 = 1\n        ",
                );
            match category.is_some() {
                true => {
                    let __sql_forge_section_filter_category_0_cat = category.unwrap();
                    __builder.push(" AND p.category = ");
                    __builder.push_bind(__sql_forge_section_filter_category_0_cat);
                    __builder.push(" ");
                }
                false => {}
            }
            __builder.push("\n        ");
            match price_min.is_some() {
                true => {
                    let __sql_forge_section_filter_price_min_0_price_min = price_min
                        .unwrap();
                    __builder.push(" AND p.price >= ");
                    __builder
                        .push_bind(__sql_forge_section_filter_price_min_0_price_min);
                    __builder.push(" ");
                }
                false => {}
            }
            __builder.push("\n        ");
            match price_max.is_some() {
                true => {
                    let __sql_forge_section_filter_price_max_0_price_max = price_max
                        .unwrap();
                    __builder.push(" AND p.price <= ");
                    __builder
                        .push_bind(__sql_forge_section_filter_price_max_0_price_max);
                    __builder.push(" ");
                }
                false => {}
            }
            __builder.push("\n        ");
            match in_stock_only {
                true => {
                    __builder.push(" AND p.stock > 0 ");
                }
                false => {}
            }
            __builder.push("\n        ");
            match order_by.as_deref() {
                Some("price_asc") => {
                    __builder.push(" ORDER BY p.price ASC ");
                }
                Some("price_desc") => {
                    __builder.push(" ORDER BY p.price DESC ");
                }
                _ => {
                    __builder.push(" ORDER BY p.id ASC ");
                }
            }
            __builder.push("\n        ");
            match page_size.is_some() {
                true => {
                    let __sql_forge_section_limit_0_offset = page * page_size.unwrap();
                    let __sql_forge_section_limit_0_size = page_size.unwrap();
                    __builder.push(" LIMIT ");
                    __builder.push_bind(__sql_forge_section_limit_0_size);
                    __builder.push(" OFFSET ");
                    __builder.push_bind(__sql_forge_section_limit_0_offset);
                    __builder.push(" ");
                }
                false => {}
            }
            __builder.push("\n        ");
            let __sql_forge_value_single = __SqlForgeQuery_single {
                inner: __builder,
            };
            __sql_forge_value_single
        }
            .fetch_all(&pool)
            .await
            .expect("combining features query failed");
        if !!products.is_empty() {
            {
                ::core::panicking::panic_fmt(
                    format_args!("expected at least one product"),
                );
            }
        }
        for p in &products {
            match (&p.category, &"Electronics") {
                (left_val, right_val) => {
                    if !(*left_val == *right_val) {
                        let kind = ::core::panicking::AssertKind::Eq;
                        ::core::panicking::assert_failed(
                            kind,
                            &*left_val,
                            &*right_val,
                            ::core::option::Option::None,
                        );
                    }
                }
            };
            if !(p.price >= price_new(50, 0)) {
                {
                    ::core::panicking::panic_fmt(format_args!("price should be >= 50"));
                }
            }
            if !(p.stock > 0) {
                {
                    ::core::panicking::panic_fmt(format_args!("stock should be > 0"));
                }
            }
        }
        match (&products.first().map(|p| p.name.as_str()), &Some("Tablet")) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::Some(
                            format_args!(
                                "expected price_desc order: Tablet (800.00) should be first",
                            ),
                        ),
                    );
                }
            }
        };
    };
    let mut body = body;
    #[allow(unused_mut)]
    let mut body = unsafe { ::tokio::macros::support::Pin::new_unchecked(&mut body) };
    let body: ::core::pin::Pin<&mut dyn ::core::future::Future<Output = ()>> = body;
    #[allow(
        clippy::expect_used,
        clippy::diverging_sub_expression,
        clippy::needless_return,
        clippy::unwrap_in_result
    )]
    {
        use tokio::runtime::Builder;
        return Builder::new_current_thread()
            .enable_all()
            .build()
            .expect("Failed building the Runtime")
            .block_on(body);
    }
}
extern crate test;
#[rustc_test_marker = "execute_only_query"]
#[doc(hidden)]
pub const execute_only_query: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("execute_only_query"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "tests/tests.rs",
        start_line: 554usize,
        start_col: 10usize,
        end_line: 554usize,
        end_col: 28usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(execute_only_query()),
    ),
};
fn execute_only_query() {
    let body = async {
        let pool = pool().await;
        {
            let _sql_forge_validator = || {
                let __sql_forge_top_level_id = &(1i64);
                {
                    {
                        let __sql_forge_validator_arg_0 = __sql_forge_top_level_id;
                        let _ = {
                            #[allow(clippy::all)]
                            {
                                use ::sqlx::Arguments as _;
                                let arg0 = &(__sql_forge_validator_arg_0);
                                let mut query_args = <sqlx::sqlite::Sqlite as ::sqlx::database::Database>::Arguments::default();
                                query_args
                                    .reserve(
                                        1usize,
                                        0
                                            + ::sqlx::encode::Encode::<
                                                sqlx::sqlite::Sqlite,
                                            >::size_hint(arg0),
                                    );
                                let query_args = ::core::result::Result::<
                                    _,
                                    ::sqlx::error::BoxDynError,
                                >::Ok(query_args)
                                    .and_then(move |mut query_args| {
                                        query_args.add(arg0).map(move |()| query_args)
                                    });
                                ::sqlx::__query_with_result::<
                                    sqlx::sqlite::Sqlite,
                                    _,
                                >("UPDATE products SET stock = 50 WHERE id = ?", query_args)
                            }
                        };
                    }
                }
            };
            struct __SqlForgeQuery_single {
                inner: sqlx::QueryBuilder<sqlx::Sqlite>,
            }
            impl __SqlForgeQuery_single {
                async fn execute<'e, E>(
                    mut self,
                    executor: E,
                ) -> Result<<sqlx::Sqlite as sqlx::Database>::QueryResult, sqlx::Error>
                where
                    E: sqlx::Executor<'e, Database = sqlx::Sqlite>,
                {
                    self.inner.build().execute(executor).await
                }
            }
            impl sql_forge::SqlForgeQueryExecute for __SqlForgeQuery_single {
                type Db = sqlx::Sqlite;
                fn execute<'e, E>(
                    self,
                    executor: E,
                ) -> impl std::future::Future<
                    Output = Result<
                        <sqlx::Sqlite as sqlx::Database>::QueryResult,
                        sqlx::Error,
                    >,
                > + Send + 'e
                where
                    Self: Sized + 'e,
                    E: sqlx::Executor<'e, Database = sqlx::Sqlite> + Send + 'e,
                    sqlx::Sqlite: 'e,
                {
                    __SqlForgeQuery_single::execute(self, executor)
                }
            }
            let __sql_forge_runtime_id = 1i64;
            let mut __builder: sqlx::QueryBuilder<sqlx::Sqlite> = sqlx::QueryBuilder::new(
                "",
            );
            __builder.push("UPDATE products SET stock = 50 WHERE id = ");
            __builder.push_bind(__sql_forge_runtime_id);
            let __sql_forge_value_single = __SqlForgeQuery_single {
                inner: __builder,
            };
            __sql_forge_value_single
        }
            .execute(&pool)
            .await
            .expect("reset stock failed");
        {
            let _sql_forge_validator = || {
                let __sql_forge_top_level_id = &(1i64);
                {
                    {
                        let __sql_forge_validator_arg_0 = __sql_forge_top_level_id;
                        let _ = {
                            #[allow(clippy::all)]
                            {
                                use ::sqlx::Arguments as _;
                                let arg0 = &(__sql_forge_validator_arg_0);
                                let mut query_args = <sqlx::sqlite::Sqlite as ::sqlx::database::Database>::Arguments::default();
                                query_args
                                    .reserve(
                                        1usize,
                                        0
                                            + ::sqlx::encode::Encode::<
                                                sqlx::sqlite::Sqlite,
                                            >::size_hint(arg0),
                                    );
                                let query_args = ::core::result::Result::<
                                    _,
                                    ::sqlx::error::BoxDynError,
                                >::Ok(query_args)
                                    .and_then(move |mut query_args| {
                                        query_args.add(arg0).map(move |()| query_args)
                                    });
                                ::sqlx::__query_with_result::<
                                    sqlx::sqlite::Sqlite,
                                    _,
                                >(
                                    "\n        UPDATE products SET stock = stock + 1 WHERE id = ?\n        ",
                                    query_args,
                                )
                            }
                        };
                    }
                }
            };
            struct __SqlForgeQuery_single {
                inner: sqlx::QueryBuilder<sqlx::Sqlite>,
            }
            impl __SqlForgeQuery_single {
                async fn execute<'e, E>(
                    mut self,
                    executor: E,
                ) -> Result<<sqlx::Sqlite as sqlx::Database>::QueryResult, sqlx::Error>
                where
                    E: sqlx::Executor<'e, Database = sqlx::Sqlite>,
                {
                    self.inner.build().execute(executor).await
                }
            }
            impl sql_forge::SqlForgeQueryExecute for __SqlForgeQuery_single {
                type Db = sqlx::Sqlite;
                fn execute<'e, E>(
                    self,
                    executor: E,
                ) -> impl std::future::Future<
                    Output = Result<
                        <sqlx::Sqlite as sqlx::Database>::QueryResult,
                        sqlx::Error,
                    >,
                > + Send + 'e
                where
                    Self: Sized + 'e,
                    E: sqlx::Executor<'e, Database = sqlx::Sqlite> + Send + 'e,
                    sqlx::Sqlite: 'e,
                {
                    __SqlForgeQuery_single::execute(self, executor)
                }
            }
            let __sql_forge_runtime_id = 1i64;
            let mut __builder: sqlx::QueryBuilder<sqlx::Sqlite> = sqlx::QueryBuilder::new(
                "",
            );
            __builder
                .push("\n        UPDATE products SET stock = stock + 1 WHERE id = ");
            __builder.push_bind(__sql_forge_runtime_id);
            __builder.push("\n        ");
            let __sql_forge_value_single = __SqlForgeQuery_single {
                inner: __builder,
            };
            __sql_forge_value_single
        }
            .execute(&pool)
            .await
            .expect("execute-only query failed");
        let row: (i64,) = sqlx::query_as::<
            _,
            (i64,),
        >("SELECT stock FROM products WHERE id = 1")
            .fetch_one(&pool)
            .await
            .expect("readback failed");
        match (&row.0, &51) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::Some(
                            format_args!(
                                "stock should have been incremented from 50 to 51",
                            ),
                        ),
                    );
                }
            }
        };
    };
    let mut body = body;
    #[allow(unused_mut)]
    let mut body = unsafe { ::tokio::macros::support::Pin::new_unchecked(&mut body) };
    let body: ::core::pin::Pin<&mut dyn ::core::future::Future<Output = ()>> = body;
    #[allow(
        clippy::expect_used,
        clippy::diverging_sub_expression,
        clippy::needless_return,
        clippy::unwrap_in_result
    )]
    {
        use tokio::runtime::Builder;
        return Builder::new_current_thread()
            .enable_all()
            .build()
            .expect("Failed building the Runtime")
            .block_on(body);
    }
}
extern crate test;
#[rustc_test_marker = "execute_only_insert_update_delete"]
#[doc(hidden)]
pub const execute_only_insert_update_delete: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("execute_only_insert_update_delete"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "tests/tests.rs",
        start_line: 586usize,
        start_col: 10usize,
        end_line: 586usize,
        end_col: 43usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(execute_only_insert_update_delete()),
    ),
};
fn execute_only_insert_update_delete() {
    let body = async {
        let pool = pool().await;
        {
            let _sql_forge_validator = || {
                let __sql_forge_top_level_category = &("Temporary");
                {
                    {
                        let __sql_forge_validator_arg_0 = __sql_forge_top_level_category;
                        let _ = {
                            #[allow(clippy::all)]
                            {
                                use ::sqlx::Arguments as _;
                                let arg0 = &(__sql_forge_validator_arg_0);
                                let mut query_args = <sqlx::sqlite::Sqlite as ::sqlx::database::Database>::Arguments::default();
                                query_args
                                    .reserve(
                                        1usize,
                                        0
                                            + ::sqlx::encode::Encode::<
                                                sqlx::sqlite::Sqlite,
                                            >::size_hint(arg0),
                                    );
                                let query_args = ::core::result::Result::<
                                    _,
                                    ::sqlx::error::BoxDynError,
                                >::Ok(query_args)
                                    .and_then(move |mut query_args| {
                                        query_args.add(arg0).map(move |()| query_args)
                                    });
                                ::sqlx::__query_with_result::<
                                    sqlx::sqlite::Sqlite,
                                    _,
                                >("DELETE FROM products WHERE category = ?", query_args)
                            }
                        };
                    }
                }
            };
            struct __SqlForgeQuery_single {
                inner: sqlx::QueryBuilder<sqlx::Sqlite>,
            }
            impl __SqlForgeQuery_single {
                async fn execute<'e, E>(
                    mut self,
                    executor: E,
                ) -> Result<<sqlx::Sqlite as sqlx::Database>::QueryResult, sqlx::Error>
                where
                    E: sqlx::Executor<'e, Database = sqlx::Sqlite>,
                {
                    self.inner.build().execute(executor).await
                }
            }
            impl sql_forge::SqlForgeQueryExecute for __SqlForgeQuery_single {
                type Db = sqlx::Sqlite;
                fn execute<'e, E>(
                    self,
                    executor: E,
                ) -> impl std::future::Future<
                    Output = Result<
                        <sqlx::Sqlite as sqlx::Database>::QueryResult,
                        sqlx::Error,
                    >,
                > + Send + 'e
                where
                    Self: Sized + 'e,
                    E: sqlx::Executor<'e, Database = sqlx::Sqlite> + Send + 'e,
                    sqlx::Sqlite: 'e,
                {
                    __SqlForgeQuery_single::execute(self, executor)
                }
            }
            let __sql_forge_runtime_category = "Temporary";
            let mut __builder: sqlx::QueryBuilder<sqlx::Sqlite> = sqlx::QueryBuilder::new(
                "",
            );
            __builder.push("DELETE FROM products WHERE category = ");
            __builder.push_bind(__sql_forge_runtime_category);
            let __sql_forge_value_single = __SqlForgeQuery_single {
                inner: __builder,
            };
            __sql_forge_value_single
        }
            .execute(&pool)
            .await
            .ok();
        let names = ["Temp A", "Temp B", "Temp C"];
        let base_price = price_new(9999, 2);
        for (i, name) in names.iter().enumerate() {
            {
                let _sql_forge_validator = || {
                    let __sql_forge_top_level_name = &(name);
                    let __sql_forge_top_level_price = &(price_inc(
                        &base_price,
                        i as i64,
                        2,
                    ));
                    let __sql_forge_top_level_stock = &(10i64);
                    let __sql_forge_top_level_category = &("Temporary");
                    {
                        {
                            let __sql_forge_validator_arg_0 = __sql_forge_top_level_name;
                            let __sql_forge_validator_arg_1 = __sql_forge_top_level_price;
                            let __sql_forge_validator_arg_2 = __sql_forge_top_level_stock;
                            let __sql_forge_validator_arg_3 = __sql_forge_top_level_category;
                            let _ = {
                                #[allow(clippy::all)]
                                {
                                    use ::sqlx::Arguments as _;
                                    let arg0 = &(__sql_forge_validator_arg_0);
                                    let arg1 = &(__sql_forge_validator_arg_1);
                                    let arg2 = &(__sql_forge_validator_arg_2);
                                    let arg3 = &(__sql_forge_validator_arg_3);
                                    let mut query_args = <sqlx::sqlite::Sqlite as ::sqlx::database::Database>::Arguments::default();
                                    query_args
                                        .reserve(
                                            4usize,
                                            0
                                                + ::sqlx::encode::Encode::<
                                                    sqlx::sqlite::Sqlite,
                                                >::size_hint(arg0)
                                                + ::sqlx::encode::Encode::<
                                                    sqlx::sqlite::Sqlite,
                                                >::size_hint(arg1)
                                                + ::sqlx::encode::Encode::<
                                                    sqlx::sqlite::Sqlite,
                                                >::size_hint(arg2)
                                                + ::sqlx::encode::Encode::<
                                                    sqlx::sqlite::Sqlite,
                                                >::size_hint(arg3),
                                        );
                                    let query_args = ::core::result::Result::<
                                        _,
                                        ::sqlx::error::BoxDynError,
                                    >::Ok(query_args)
                                        .and_then(move |mut query_args| {
                                            query_args.add(arg0).map(move |()| query_args)
                                        })
                                        .and_then(move |mut query_args| {
                                            query_args.add(arg1).map(move |()| query_args)
                                        })
                                        .and_then(move |mut query_args| {
                                            query_args.add(arg2).map(move |()| query_args)
                                        })
                                        .and_then(move |mut query_args| {
                                            query_args.add(arg3).map(move |()| query_args)
                                        });
                                    ::sqlx::__query_with_result::<
                                        sqlx::sqlite::Sqlite,
                                        _,
                                    >(
                                        "\n            INSERT INTO products (name, price, stock, category)\n            VALUES (?, ?, ?, ?)\n            ",
                                        query_args,
                                    )
                                }
                            };
                        }
                    }
                };
                struct __SqlForgeQuery_single {
                    inner: sqlx::QueryBuilder<sqlx::Sqlite>,
                }
                impl __SqlForgeQuery_single {
                    async fn execute<'e, E>(
                        mut self,
                        executor: E,
                    ) -> Result<
                        <sqlx::Sqlite as sqlx::Database>::QueryResult,
                        sqlx::Error,
                    >
                    where
                        E: sqlx::Executor<'e, Database = sqlx::Sqlite>,
                    {
                        self.inner.build().execute(executor).await
                    }
                }
                impl sql_forge::SqlForgeQueryExecute for __SqlForgeQuery_single {
                    type Db = sqlx::Sqlite;
                    fn execute<'e, E>(
                        self,
                        executor: E,
                    ) -> impl std::future::Future<
                        Output = Result<
                            <sqlx::Sqlite as sqlx::Database>::QueryResult,
                            sqlx::Error,
                        >,
                    > + Send + 'e
                    where
                        Self: Sized + 'e,
                        E: sqlx::Executor<'e, Database = sqlx::Sqlite> + Send + 'e,
                        sqlx::Sqlite: 'e,
                    {
                        __SqlForgeQuery_single::execute(self, executor)
                    }
                }
                let __sql_forge_runtime_name = name;
                let __sql_forge_runtime_price = price_inc(&base_price, i as i64, 2);
                let __sql_forge_runtime_stock = 10i64;
                let __sql_forge_runtime_category = "Temporary";
                let mut __builder: sqlx::QueryBuilder<sqlx::Sqlite> = sqlx::QueryBuilder::new(
                    "",
                );
                __builder
                    .push(
                        "\n            INSERT INTO products (name, price, stock, category)\n            VALUES (",
                    );
                __builder.push_bind(__sql_forge_runtime_name);
                __builder.push(", ");
                __builder.push_bind(__sql_forge_runtime_price);
                __builder.push(", ");
                __builder.push_bind(__sql_forge_runtime_stock);
                __builder.push(", ");
                __builder.push_bind(__sql_forge_runtime_category);
                __builder.push(")\n            ");
                let __sql_forge_value_single = __SqlForgeQuery_single {
                    inner: __builder,
                };
                __sql_forge_value_single
            }
                .execute(&pool)
                .await
                .expect("insert failed");
        }
        {
            let _sql_forge_validator = || {
                let __sql_forge_top_level_new_price = &(price_new(4999, 2));
                let __sql_forge_top_level_category = &("Temporary");
                let __sql_forge_top_level_name = &("Temp B");
                {
                    {
                        let __sql_forge_validator_arg_0 = __sql_forge_top_level_new_price;
                        let __sql_forge_validator_arg_1 = __sql_forge_top_level_category;
                        let __sql_forge_validator_arg_2 = __sql_forge_top_level_name;
                        let _ = {
                            #[allow(clippy::all)]
                            {
                                use ::sqlx::Arguments as _;
                                let arg0 = &(__sql_forge_validator_arg_0);
                                let arg1 = &(__sql_forge_validator_arg_1);
                                let arg2 = &(__sql_forge_validator_arg_2);
                                let mut query_args = <sqlx::sqlite::Sqlite as ::sqlx::database::Database>::Arguments::default();
                                query_args
                                    .reserve(
                                        3usize,
                                        0
                                            + ::sqlx::encode::Encode::<
                                                sqlx::sqlite::Sqlite,
                                            >::size_hint(arg0)
                                            + ::sqlx::encode::Encode::<
                                                sqlx::sqlite::Sqlite,
                                            >::size_hint(arg1)
                                            + ::sqlx::encode::Encode::<
                                                sqlx::sqlite::Sqlite,
                                            >::size_hint(arg2),
                                    );
                                let query_args = ::core::result::Result::<
                                    _,
                                    ::sqlx::error::BoxDynError,
                                >::Ok(query_args)
                                    .and_then(move |mut query_args| {
                                        query_args.add(arg0).map(move |()| query_args)
                                    })
                                    .and_then(move |mut query_args| {
                                        query_args.add(arg1).map(move |()| query_args)
                                    })
                                    .and_then(move |mut query_args| {
                                        query_args.add(arg2).map(move |()| query_args)
                                    });
                                ::sqlx::__query_with_result::<
                                    sqlx::sqlite::Sqlite,
                                    _,
                                >(
                                    "\n        UPDATE products\n        SET price = ?\n        WHERE category = ? AND name = ?\n        ",
                                    query_args,
                                )
                            }
                        };
                    }
                }
            };
            struct __SqlForgeQuery_single {
                inner: sqlx::QueryBuilder<sqlx::Sqlite>,
            }
            impl __SqlForgeQuery_single {
                async fn execute<'e, E>(
                    mut self,
                    executor: E,
                ) -> Result<<sqlx::Sqlite as sqlx::Database>::QueryResult, sqlx::Error>
                where
                    E: sqlx::Executor<'e, Database = sqlx::Sqlite>,
                {
                    self.inner.build().execute(executor).await
                }
            }
            impl sql_forge::SqlForgeQueryExecute for __SqlForgeQuery_single {
                type Db = sqlx::Sqlite;
                fn execute<'e, E>(
                    self,
                    executor: E,
                ) -> impl std::future::Future<
                    Output = Result<
                        <sqlx::Sqlite as sqlx::Database>::QueryResult,
                        sqlx::Error,
                    >,
                > + Send + 'e
                where
                    Self: Sized + 'e,
                    E: sqlx::Executor<'e, Database = sqlx::Sqlite> + Send + 'e,
                    sqlx::Sqlite: 'e,
                {
                    __SqlForgeQuery_single::execute(self, executor)
                }
            }
            let __sql_forge_runtime_new_price = price_new(4999, 2);
            let __sql_forge_runtime_category = "Temporary";
            let __sql_forge_runtime_name = "Temp B";
            let mut __builder: sqlx::QueryBuilder<sqlx::Sqlite> = sqlx::QueryBuilder::new(
                "",
            );
            __builder.push("\n        UPDATE products\n        SET price = ");
            __builder.push_bind(__sql_forge_runtime_new_price);
            __builder.push("\n        WHERE category = ");
            __builder.push_bind(__sql_forge_runtime_category);
            __builder.push(" AND name = ");
            __builder.push_bind(__sql_forge_runtime_name);
            __builder.push("\n        ");
            let __sql_forge_value_single = __SqlForgeQuery_single {
                inner: __builder,
            };
            __sql_forge_value_single
        }
            .execute(&pool)
            .await
            .expect("update failed");
        struct TempRow {
            #[expect(dead_code)]
            name: String,
            price: Price,
        }
        #[automatically_derived]
        impl<'a, R: ::sqlx::Row> ::sqlx::FromRow<'a, R> for TempRow
        where
            &'a ::std::primitive::str: ::sqlx::ColumnIndex<R>,
            String: ::sqlx::decode::Decode<'a, R::Database>,
            String: ::sqlx::types::Type<R::Database>,
            Price: ::sqlx::decode::Decode<'a, R::Database>,
            Price: ::sqlx::types::Type<R::Database>,
        {
            fn from_row(__row: &'a R) -> ::sqlx::Result<Self> {
                let name: String = __row.try_get("name")?;
                let price: Price = __row.try_get("price")?;
                ::std::result::Result::Ok(TempRow { name, price })
            }
        }
        let rows: Vec<TempRow> = {
            let _sql_forge_validator = || {
                let __sql_forge_top_level_cat = &("Temporary");
                {
                    type __SqlForgeModel = TempRow;
                    {
                        let __sql_forge_validator_arg_0 = __sql_forge_top_level_cat;
                        let _ = {
                            {
                                #[allow(clippy::all)]
                                {
                                    use ::sqlx::Arguments as _;
                                    let arg0 = &(__sql_forge_validator_arg_0);
                                    let mut query_args = <sqlx::sqlite::Sqlite as ::sqlx::database::Database>::Arguments::default();
                                    query_args
                                        .reserve(
                                            1usize,
                                            0
                                                + ::sqlx::encode::Encode::<
                                                    sqlx::sqlite::Sqlite,
                                                >::size_hint(arg0),
                                        );
                                    let query_args = ::core::result::Result::<
                                        _,
                                        ::sqlx::error::BoxDynError,
                                    >::Ok(query_args)
                                        .and_then(move |mut query_args| {
                                            query_args.add(arg0).map(move |()| query_args)
                                        });
                                    ::sqlx::__query_with_result::<
                                        sqlx::sqlite::Sqlite,
                                        _,
                                    >(
                                            "\n        SELECT name, price FROM products\n        WHERE category = ?\n        ORDER BY id\n        ",
                                            query_args,
                                        )
                                        .try_map(|row: sqlx::sqlite::SqliteRow| {
                                            use ::sqlx::Row as _;
                                            #[allow(non_snake_case)]
                                            let sqlx_query_as_name = row
                                                .try_get_unchecked::<String, _>(0usize)?
                                                .into();
                                            #[allow(non_snake_case)]
                                            let sqlx_query_as_price = row
                                                .try_get_unchecked::<i64, _>(1usize)?
                                                .into();
                                            ::std::result::Result::Ok(__SqlForgeModel {
                                                name: sqlx_query_as_name,
                                                price: sqlx_query_as_price,
                                            })
                                        })
                                }
                            }
                        };
                    }
                }
            };
            struct __SqlForgeQuery_single {
                inner: sqlx::QueryBuilder<sqlx::Sqlite>,
            }
            impl __SqlForgeQuery_single {
                async fn fetch_all<'e, E>(
                    mut self,
                    executor: E,
                ) -> Result<Vec<TempRow>, sqlx::Error>
                where
                    E: sqlx::Executor<'e, Database = sqlx::Sqlite>,
                {
                    self.inner.build_query_as::<TempRow>().fetch_all(executor).await
                }
                async fn fetch_one<'e, E>(
                    mut self,
                    executor: E,
                ) -> Result<TempRow, sqlx::Error>
                where
                    E: sqlx::Executor<'e, Database = sqlx::Sqlite>,
                {
                    self.inner.build_query_as::<TempRow>().fetch_one(executor).await
                }
                async fn fetch_optional<'e, E>(
                    mut self,
                    executor: E,
                ) -> Result<Option<TempRow>, sqlx::Error>
                where
                    E: sqlx::Executor<'e, Database = sqlx::Sqlite>,
                {
                    self.inner.build_query_as::<TempRow>().fetch_optional(executor).await
                }
                async fn execute<'e, E>(
                    mut self,
                    executor: E,
                ) -> Result<<sqlx::Sqlite as sqlx::Database>::QueryResult, sqlx::Error>
                where
                    E: sqlx::Executor<'e, Database = sqlx::Sqlite>,
                {
                    self.inner.build().execute(executor).await
                }
            }
            impl sql_forge::SqlForgeQuery<TempRow> for __SqlForgeQuery_single {
                type Db = sqlx::Sqlite;
                fn fetch_all<'e, E>(
                    self,
                    executor: E,
                ) -> impl std::future::Future<
                    Output = Result<Vec<TempRow>, sqlx::Error>,
                > + Send + 'e
                where
                    Self: Sized + 'e,
                    E: sqlx::Executor<'e, Database = sqlx::Sqlite> + Send + 'e,
                    sqlx::Sqlite: 'e,
                {
                    __SqlForgeQuery_single::fetch_all(self, executor)
                }
                fn fetch_one<'e, E>(
                    self,
                    executor: E,
                ) -> impl std::future::Future<
                    Output = Result<TempRow, sqlx::Error>,
                > + Send + 'e
                where
                    Self: Sized + 'e,
                    E: sqlx::Executor<'e, Database = sqlx::Sqlite> + Send + 'e,
                    sqlx::Sqlite: 'e,
                {
                    __SqlForgeQuery_single::fetch_one(self, executor)
                }
                fn fetch_optional<'e, E>(
                    self,
                    executor: E,
                ) -> impl std::future::Future<
                    Output = Result<Option<TempRow>, sqlx::Error>,
                > + Send + 'e
                where
                    Self: Sized + 'e,
                    E: sqlx::Executor<'e, Database = sqlx::Sqlite> + Send + 'e,
                    sqlx::Sqlite: 'e,
                {
                    __SqlForgeQuery_single::fetch_optional(self, executor)
                }
                fn execute<'e, E>(
                    self,
                    executor: E,
                ) -> impl std::future::Future<
                    Output = Result<
                        <sqlx::Sqlite as sqlx::Database>::QueryResult,
                        sqlx::Error,
                    >,
                > + Send + 'e
                where
                    Self: Sized + 'e,
                    E: sqlx::Executor<'e, Database = sqlx::Sqlite> + Send + 'e,
                    sqlx::Sqlite: 'e,
                {
                    __SqlForgeQuery_single::execute(self, executor)
                }
            }
            let __sql_forge_runtime_cat = "Temporary";
            let mut __builder: sqlx::QueryBuilder<sqlx::Sqlite> = sqlx::QueryBuilder::new(
                "",
            );
            __builder
                .push(
                    "\n        SELECT name, price FROM products\n        WHERE category = ",
                );
            __builder.push_bind(__sql_forge_runtime_cat);
            __builder.push("\n        ORDER BY id\n        ");
            let __sql_forge_value_single = __SqlForgeQuery_single {
                inner: __builder,
            };
            __sql_forge_value_single
        }
            .fetch_all(&pool)
            .await
            .expect("select after update failed");
        match (&rows.len(), &3) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::None,
                    );
                }
            }
        };
        match (&rows[0].price, &price_new(9999, 2)) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::None,
                    );
                }
            }
        };
        match (&rows[1].price, &price_new(4999, 2)) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::None,
                    );
                }
            }
        };
        match (&rows[2].price, &price_new(10001, 2)) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::None,
                    );
                }
            }
        };
        {
            let _sql_forge_validator = || {
                let __sql_forge_top_level_category = &("Temporary");
                {
                    {
                        let __sql_forge_validator_arg_0 = __sql_forge_top_level_category;
                        let _ = {
                            #[allow(clippy::all)]
                            {
                                use ::sqlx::Arguments as _;
                                let arg0 = &(__sql_forge_validator_arg_0);
                                let mut query_args = <sqlx::sqlite::Sqlite as ::sqlx::database::Database>::Arguments::default();
                                query_args
                                    .reserve(
                                        1usize,
                                        0
                                            + ::sqlx::encode::Encode::<
                                                sqlx::sqlite::Sqlite,
                                            >::size_hint(arg0),
                                    );
                                let query_args = ::core::result::Result::<
                                    _,
                                    ::sqlx::error::BoxDynError,
                                >::Ok(query_args)
                                    .and_then(move |mut query_args| {
                                        query_args.add(arg0).map(move |()| query_args)
                                    });
                                ::sqlx::__query_with_result::<
                                    sqlx::sqlite::Sqlite,
                                    _,
                                >(
                                    "\n        DELETE FROM products\n        WHERE category = ?\n        ",
                                    query_args,
                                )
                            }
                        };
                    }
                }
            };
            struct __SqlForgeQuery_single {
                inner: sqlx::QueryBuilder<sqlx::Sqlite>,
            }
            impl __SqlForgeQuery_single {
                async fn execute<'e, E>(
                    mut self,
                    executor: E,
                ) -> Result<<sqlx::Sqlite as sqlx::Database>::QueryResult, sqlx::Error>
                where
                    E: sqlx::Executor<'e, Database = sqlx::Sqlite>,
                {
                    self.inner.build().execute(executor).await
                }
            }
            impl sql_forge::SqlForgeQueryExecute for __SqlForgeQuery_single {
                type Db = sqlx::Sqlite;
                fn execute<'e, E>(
                    self,
                    executor: E,
                ) -> impl std::future::Future<
                    Output = Result<
                        <sqlx::Sqlite as sqlx::Database>::QueryResult,
                        sqlx::Error,
                    >,
                > + Send + 'e
                where
                    Self: Sized + 'e,
                    E: sqlx::Executor<'e, Database = sqlx::Sqlite> + Send + 'e,
                    sqlx::Sqlite: 'e,
                {
                    __SqlForgeQuery_single::execute(self, executor)
                }
            }
            let __sql_forge_runtime_category = "Temporary";
            let mut __builder: sqlx::QueryBuilder<sqlx::Sqlite> = sqlx::QueryBuilder::new(
                "",
            );
            __builder.push("\n        DELETE FROM products\n        WHERE category = ");
            __builder.push_bind(__sql_forge_runtime_category);
            __builder.push("\n        ");
            let __sql_forge_value_single = __SqlForgeQuery_single {
                inner: __builder,
            };
            __sql_forge_value_single
        }
            .execute(&pool)
            .await
            .expect("delete failed");
        let remaining: i64 = {
            let _sql_forge_validator = || {
                let __sql_forge_top_level_cat = &("Temporary");
                {
                    {
                        let __sql_forge_validator_arg_0 = __sql_forge_top_level_cat;
                        let _ = {
                            #[allow(clippy::all)]
                            {
                                use ::sqlx::Arguments as _;
                                let arg0 = &(__sql_forge_validator_arg_0);
                                let mut query_args = <sqlx::sqlite::Sqlite as ::sqlx::database::Database>::Arguments::default();
                                query_args
                                    .reserve(
                                        1usize,
                                        0
                                            + ::sqlx::encode::Encode::<
                                                sqlx::sqlite::Sqlite,
                                            >::size_hint(arg0),
                                    );
                                let query_args = ::core::result::Result::<
                                    _,
                                    ::sqlx::error::BoxDynError,
                                >::Ok(query_args)
                                    .and_then(move |mut query_args| {
                                        query_args.add(arg0).map(move |()| query_args)
                                    });
                                ::sqlx::__query_scalar_with_result::<
                                    sqlx::sqlite::Sqlite,
                                    i64,
                                    _,
                                >(
                                    "SELECT COUNT(*) FROM products WHERE category = ?",
                                    query_args,
                                )
                            }
                        };
                    }
                }
            };
            struct __SqlForgeQuery_single {
                inner: sqlx::QueryBuilder<sqlx::Sqlite>,
            }
            impl __SqlForgeQuery_single {
                async fn fetch_all<'e, E>(
                    mut self,
                    executor: E,
                ) -> Result<Vec<i64>, sqlx::Error>
                where
                    E: sqlx::Executor<'e, Database = sqlx::Sqlite>,
                {
                    self.inner.build_query_scalar::<i64>().fetch_all(executor).await
                }
                async fn fetch_one<'e, E>(
                    mut self,
                    executor: E,
                ) -> Result<i64, sqlx::Error>
                where
                    E: sqlx::Executor<'e, Database = sqlx::Sqlite>,
                {
                    self.inner.build_query_scalar::<i64>().fetch_one(executor).await
                }
                async fn fetch_optional<'e, E>(
                    mut self,
                    executor: E,
                ) -> Result<Option<i64>, sqlx::Error>
                where
                    E: sqlx::Executor<'e, Database = sqlx::Sqlite>,
                {
                    self.inner.build_query_scalar::<i64>().fetch_optional(executor).await
                }
                async fn execute<'e, E>(
                    mut self,
                    executor: E,
                ) -> Result<<sqlx::Sqlite as sqlx::Database>::QueryResult, sqlx::Error>
                where
                    E: sqlx::Executor<'e, Database = sqlx::Sqlite>,
                {
                    self.inner.build().execute(executor).await
                }
            }
            impl sql_forge::SqlForgeQuery<i64> for __SqlForgeQuery_single {
                type Db = sqlx::Sqlite;
                fn fetch_all<'e, E>(
                    self,
                    executor: E,
                ) -> impl std::future::Future<
                    Output = Result<Vec<i64>, sqlx::Error>,
                > + Send + 'e
                where
                    Self: Sized + 'e,
                    E: sqlx::Executor<'e, Database = sqlx::Sqlite> + Send + 'e,
                    sqlx::Sqlite: 'e,
                {
                    __SqlForgeQuery_single::fetch_all(self, executor)
                }
                fn fetch_one<'e, E>(
                    self,
                    executor: E,
                ) -> impl std::future::Future<
                    Output = Result<i64, sqlx::Error>,
                > + Send + 'e
                where
                    Self: Sized + 'e,
                    E: sqlx::Executor<'e, Database = sqlx::Sqlite> + Send + 'e,
                    sqlx::Sqlite: 'e,
                {
                    __SqlForgeQuery_single::fetch_one(self, executor)
                }
                fn fetch_optional<'e, E>(
                    self,
                    executor: E,
                ) -> impl std::future::Future<
                    Output = Result<Option<i64>, sqlx::Error>,
                > + Send + 'e
                where
                    Self: Sized + 'e,
                    E: sqlx::Executor<'e, Database = sqlx::Sqlite> + Send + 'e,
                    sqlx::Sqlite: 'e,
                {
                    __SqlForgeQuery_single::fetch_optional(self, executor)
                }
                fn execute<'e, E>(
                    self,
                    executor: E,
                ) -> impl std::future::Future<
                    Output = Result<
                        <sqlx::Sqlite as sqlx::Database>::QueryResult,
                        sqlx::Error,
                    >,
                > + Send + 'e
                where
                    Self: Sized + 'e,
                    E: sqlx::Executor<'e, Database = sqlx::Sqlite> + Send + 'e,
                    sqlx::Sqlite: 'e,
                {
                    __SqlForgeQuery_single::execute(self, executor)
                }
            }
            let __sql_forge_runtime_cat = "Temporary";
            let mut __builder: sqlx::QueryBuilder<sqlx::Sqlite> = sqlx::QueryBuilder::new(
                "",
            );
            __builder.push("SELECT COUNT(*) FROM products WHERE category = ");
            __builder.push_bind(__sql_forge_runtime_cat);
            let __sql_forge_value_single = __SqlForgeQuery_single {
                inner: __builder,
            };
            __sql_forge_value_single
        }
            .fetch_one(&pool)
            .await
            .expect("count after delete failed");
        match (&remaining, &0) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::Some(
                            format_args!(
                                "all temporary products should have been deleted",
                            ),
                        ),
                    );
                }
            }
        };
    };
    let mut body = body;
    #[allow(unused_mut)]
    let mut body = unsafe { ::tokio::macros::support::Pin::new_unchecked(&mut body) };
    let body: ::core::pin::Pin<&mut dyn ::core::future::Future<Output = ()>> = body;
    #[allow(
        clippy::expect_used,
        clippy::diverging_sub_expression,
        clippy::needless_return,
        clippy::unwrap_in_result
    )]
    {
        use tokio::runtime::Builder;
        return Builder::new_current_thread()
            .enable_all()
            .build()
            .expect("Failed building the Runtime")
            .block_on(body);
    }
}
struct BatchItem {
    name: String,
    price: Price,
}
#[automatically_derived]
impl<'a, R: ::sqlx::Row> ::sqlx::FromRow<'a, R> for BatchItem
where
    &'a ::std::primitive::str: ::sqlx::ColumnIndex<R>,
    String: ::sqlx::decode::Decode<'a, R::Database>,
    String: ::sqlx::types::Type<R::Database>,
    Price: ::sqlx::decode::Decode<'a, R::Database>,
    Price: ::sqlx::types::Type<R::Database>,
{
    fn from_row(__row: &'a R) -> ::sqlx::Result<Self> {
        let name: String = __row.try_get("name")?;
        let price: Price = __row.try_get("price")?;
        ::std::result::Result::Ok(BatchItem { name, price })
    }
}
extern crate test;
#[rustc_test_marker = "execute_batch"]
#[doc(hidden)]
pub const execute_batch: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("execute_batch"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "tests/tests.rs",
        start_line: 692usize,
        start_col: 10usize,
        end_line: 692usize,
        end_col: 23usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(execute_batch()),
    ),
};
fn execute_batch() {
    let body = async {
        let pool = pool().await;
        {
            let _sql_forge_validator = || {
                let __sql_forge_top_level_category = &("Batch");
                {
                    {
                        let __sql_forge_validator_arg_0 = __sql_forge_top_level_category;
                        let _ = {
                            #[allow(clippy::all)]
                            {
                                use ::sqlx::Arguments as _;
                                let arg0 = &(__sql_forge_validator_arg_0);
                                let mut query_args = <sqlx::sqlite::Sqlite as ::sqlx::database::Database>::Arguments::default();
                                query_args
                                    .reserve(
                                        1usize,
                                        0
                                            + ::sqlx::encode::Encode::<
                                                sqlx::sqlite::Sqlite,
                                            >::size_hint(arg0),
                                    );
                                let query_args = ::core::result::Result::<
                                    _,
                                    ::sqlx::error::BoxDynError,
                                >::Ok(query_args)
                                    .and_then(move |mut query_args| {
                                        query_args.add(arg0).map(move |()| query_args)
                                    });
                                ::sqlx::__query_with_result::<
                                    sqlx::sqlite::Sqlite,
                                    _,
                                >("DELETE FROM products WHERE category = ?", query_args)
                            }
                        };
                    }
                }
            };
            struct __SqlForgeQuery_single {
                inner: sqlx::QueryBuilder<sqlx::Sqlite>,
            }
            impl __SqlForgeQuery_single {
                async fn execute<'e, E>(
                    mut self,
                    executor: E,
                ) -> Result<<sqlx::Sqlite as sqlx::Database>::QueryResult, sqlx::Error>
                where
                    E: sqlx::Executor<'e, Database = sqlx::Sqlite>,
                {
                    self.inner.build().execute(executor).await
                }
            }
            impl sql_forge::SqlForgeQueryExecute for __SqlForgeQuery_single {
                type Db = sqlx::Sqlite;
                fn execute<'e, E>(
                    self,
                    executor: E,
                ) -> impl std::future::Future<
                    Output = Result<
                        <sqlx::Sqlite as sqlx::Database>::QueryResult,
                        sqlx::Error,
                    >,
                > + Send + 'e
                where
                    Self: Sized + 'e,
                    E: sqlx::Executor<'e, Database = sqlx::Sqlite> + Send + 'e,
                    sqlx::Sqlite: 'e,
                {
                    __SqlForgeQuery_single::execute(self, executor)
                }
            }
            let __sql_forge_runtime_category = "Batch";
            let mut __builder: sqlx::QueryBuilder<sqlx::Sqlite> = sqlx::QueryBuilder::new(
                "",
            );
            __builder.push("DELETE FROM products WHERE category = ");
            __builder.push_bind(__sql_forge_runtime_category);
            let __sql_forge_value_single = __SqlForgeQuery_single {
                inner: __builder,
            };
            __sql_forge_value_single
        }
            .execute(&pool)
            .await
            .ok();
        let items = ::alloc::boxed::box_assume_init_into_vec_unsafe(
            ::alloc::intrinsics::write_box_via_move(
                ::alloc::boxed::Box::new_uninit(),
                [
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
                ],
            ),
        );
        {
            let _sql_forge_validator = || {
                {
                    {
                        let _ = {
                            #[allow(clippy::all)]
                            {
                                use ::sqlx::Arguments as _;
                                let arg0 = &(items[0].name);
                                let arg1 = &(items[0].price);
                                let arg2 = &(items[0].name);
                                let arg3 = &(items[0].price);
                                let arg4 = &(items[0].name);
                                let arg5 = &(items[0].price);
                                let mut query_args = <sqlx::sqlite::Sqlite as ::sqlx::database::Database>::Arguments::default();
                                query_args
                                    .reserve(
                                        6usize,
                                        0
                                            + ::sqlx::encode::Encode::<
                                                sqlx::sqlite::Sqlite,
                                            >::size_hint(arg0)
                                            + ::sqlx::encode::Encode::<
                                                sqlx::sqlite::Sqlite,
                                            >::size_hint(arg1)
                                            + ::sqlx::encode::Encode::<
                                                sqlx::sqlite::Sqlite,
                                            >::size_hint(arg2)
                                            + ::sqlx::encode::Encode::<
                                                sqlx::sqlite::Sqlite,
                                            >::size_hint(arg3)
                                            + ::sqlx::encode::Encode::<
                                                sqlx::sqlite::Sqlite,
                                            >::size_hint(arg4)
                                            + ::sqlx::encode::Encode::<
                                                sqlx::sqlite::Sqlite,
                                            >::size_hint(arg5),
                                    );
                                let query_args = ::core::result::Result::<
                                    _,
                                    ::sqlx::error::BoxDynError,
                                >::Ok(query_args)
                                    .and_then(move |mut query_args| {
                                        query_args.add(arg0).map(move |()| query_args)
                                    })
                                    .and_then(move |mut query_args| {
                                        query_args.add(arg1).map(move |()| query_args)
                                    })
                                    .and_then(move |mut query_args| {
                                        query_args.add(arg2).map(move |()| query_args)
                                    })
                                    .and_then(move |mut query_args| {
                                        query_args.add(arg3).map(move |()| query_args)
                                    })
                                    .and_then(move |mut query_args| {
                                        query_args.add(arg4).map(move |()| query_args)
                                    })
                                    .and_then(move |mut query_args| {
                                        query_args.add(arg5).map(move |()| query_args)
                                    });
                                ::sqlx::__query_with_result::<
                                    sqlx::sqlite::Sqlite,
                                    _,
                                >(
                                    "\n        INSERT INTO products (name, price, stock, category)\n        VALUES (?, ?, 10, 'Batch'), (?, ?, 10, 'Batch'), (?, ?, 10, 'Batch')\n        ",
                                    query_args,
                                )
                            }
                        };
                    }
                }
            };
            struct __SqlForgeQuery_single {
                inner: sqlx::QueryBuilder<sqlx::Sqlite>,
            }
            impl __SqlForgeQuery_single {
                async fn execute<'e, E>(
                    mut self,
                    executor: E,
                ) -> Result<<sqlx::Sqlite as sqlx::Database>::QueryResult, sqlx::Error>
                where
                    E: sqlx::Executor<'e, Database = sqlx::Sqlite>,
                {
                    self.inner.build().execute(executor).await
                }
            }
            impl sql_forge::SqlForgeQueryExecute for __SqlForgeQuery_single {
                type Db = sqlx::Sqlite;
                fn execute<'e, E>(
                    self,
                    executor: E,
                ) -> impl std::future::Future<
                    Output = Result<
                        <sqlx::Sqlite as sqlx::Database>::QueryResult,
                        sqlx::Error,
                    >,
                > + Send + 'e
                where
                    Self: Sized + 'e,
                    E: sqlx::Executor<'e, Database = sqlx::Sqlite> + Send + 'e,
                    sqlx::Sqlite: 'e,
                {
                    __SqlForgeQuery_single::execute(self, executor)
                }
            }
            let mut __builder: sqlx::QueryBuilder<sqlx::Sqlite> = sqlx::QueryBuilder::new(
                "",
            );
            __builder
                .push(
                    "\n        INSERT INTO products (name, price, stock, category)\n        VALUES ",
                );
            {
                let mut __first = true;
                for __item in items {
                    if !__first {
                        __builder.push(", ");
                    }
                    __first = false;
                    __builder.push("(");
                    __builder.push_bind(__item.name);
                    __builder.push(", ");
                    __builder.push_bind(__item.price);
                    __builder.push(", 10, 'Batch')");
                }
            }
            __builder.push("\n        ");
            let __sql_forge_value_single = __SqlForgeQuery_single {
                inner: __builder,
            };
            __sql_forge_value_single
        }
            .execute(&pool)
            .await
            .expect("batch insert failed");
        let rows: Vec<BatchItem> = {
            let _sql_forge_validator = || {
                let __sql_forge_top_level_cat = &("Batch");
                {
                    type __SqlForgeModel = BatchItem;
                    {
                        let __sql_forge_validator_arg_0 = __sql_forge_top_level_cat;
                        let _ = {
                            {
                                #[allow(clippy::all)]
                                {
                                    use ::sqlx::Arguments as _;
                                    let arg0 = &(__sql_forge_validator_arg_0);
                                    let mut query_args = <sqlx::sqlite::Sqlite as ::sqlx::database::Database>::Arguments::default();
                                    query_args
                                        .reserve(
                                            1usize,
                                            0
                                                + ::sqlx::encode::Encode::<
                                                    sqlx::sqlite::Sqlite,
                                                >::size_hint(arg0),
                                        );
                                    let query_args = ::core::result::Result::<
                                        _,
                                        ::sqlx::error::BoxDynError,
                                    >::Ok(query_args)
                                        .and_then(move |mut query_args| {
                                            query_args.add(arg0).map(move |()| query_args)
                                        });
                                    ::sqlx::__query_with_result::<
                                        sqlx::sqlite::Sqlite,
                                        _,
                                    >(
                                            "\n        SELECT name, price FROM products\n        WHERE category = ?\n        ORDER BY id\n        ",
                                            query_args,
                                        )
                                        .try_map(|row: sqlx::sqlite::SqliteRow| {
                                            use ::sqlx::Row as _;
                                            #[allow(non_snake_case)]
                                            let sqlx_query_as_name = row
                                                .try_get_unchecked::<String, _>(0usize)?
                                                .into();
                                            #[allow(non_snake_case)]
                                            let sqlx_query_as_price = row
                                                .try_get_unchecked::<i64, _>(1usize)?
                                                .into();
                                            ::std::result::Result::Ok(__SqlForgeModel {
                                                name: sqlx_query_as_name,
                                                price: sqlx_query_as_price,
                                            })
                                        })
                                }
                            }
                        };
                    }
                }
            };
            struct __SqlForgeQuery_single {
                inner: sqlx::QueryBuilder<sqlx::Sqlite>,
            }
            impl __SqlForgeQuery_single {
                async fn fetch_all<'e, E>(
                    mut self,
                    executor: E,
                ) -> Result<Vec<BatchItem>, sqlx::Error>
                where
                    E: sqlx::Executor<'e, Database = sqlx::Sqlite>,
                {
                    self.inner.build_query_as::<BatchItem>().fetch_all(executor).await
                }
                async fn fetch_one<'e, E>(
                    mut self,
                    executor: E,
                ) -> Result<BatchItem, sqlx::Error>
                where
                    E: sqlx::Executor<'e, Database = sqlx::Sqlite>,
                {
                    self.inner.build_query_as::<BatchItem>().fetch_one(executor).await
                }
                async fn fetch_optional<'e, E>(
                    mut self,
                    executor: E,
                ) -> Result<Option<BatchItem>, sqlx::Error>
                where
                    E: sqlx::Executor<'e, Database = sqlx::Sqlite>,
                {
                    self.inner
                        .build_query_as::<BatchItem>()
                        .fetch_optional(executor)
                        .await
                }
                async fn execute<'e, E>(
                    mut self,
                    executor: E,
                ) -> Result<<sqlx::Sqlite as sqlx::Database>::QueryResult, sqlx::Error>
                where
                    E: sqlx::Executor<'e, Database = sqlx::Sqlite>,
                {
                    self.inner.build().execute(executor).await
                }
            }
            impl sql_forge::SqlForgeQuery<BatchItem> for __SqlForgeQuery_single {
                type Db = sqlx::Sqlite;
                fn fetch_all<'e, E>(
                    self,
                    executor: E,
                ) -> impl std::future::Future<
                    Output = Result<Vec<BatchItem>, sqlx::Error>,
                > + Send + 'e
                where
                    Self: Sized + 'e,
                    E: sqlx::Executor<'e, Database = sqlx::Sqlite> + Send + 'e,
                    sqlx::Sqlite: 'e,
                {
                    __SqlForgeQuery_single::fetch_all(self, executor)
                }
                fn fetch_one<'e, E>(
                    self,
                    executor: E,
                ) -> impl std::future::Future<
                    Output = Result<BatchItem, sqlx::Error>,
                > + Send + 'e
                where
                    Self: Sized + 'e,
                    E: sqlx::Executor<'e, Database = sqlx::Sqlite> + Send + 'e,
                    sqlx::Sqlite: 'e,
                {
                    __SqlForgeQuery_single::fetch_one(self, executor)
                }
                fn fetch_optional<'e, E>(
                    self,
                    executor: E,
                ) -> impl std::future::Future<
                    Output = Result<Option<BatchItem>, sqlx::Error>,
                > + Send + 'e
                where
                    Self: Sized + 'e,
                    E: sqlx::Executor<'e, Database = sqlx::Sqlite> + Send + 'e,
                    sqlx::Sqlite: 'e,
                {
                    __SqlForgeQuery_single::fetch_optional(self, executor)
                }
                fn execute<'e, E>(
                    self,
                    executor: E,
                ) -> impl std::future::Future<
                    Output = Result<
                        <sqlx::Sqlite as sqlx::Database>::QueryResult,
                        sqlx::Error,
                    >,
                > + Send + 'e
                where
                    Self: Sized + 'e,
                    E: sqlx::Executor<'e, Database = sqlx::Sqlite> + Send + 'e,
                    sqlx::Sqlite: 'e,
                {
                    __SqlForgeQuery_single::execute(self, executor)
                }
            }
            let __sql_forge_runtime_cat = "Batch";
            let mut __builder: sqlx::QueryBuilder<sqlx::Sqlite> = sqlx::QueryBuilder::new(
                "",
            );
            __builder
                .push(
                    "\n        SELECT name, price FROM products\n        WHERE category = ",
                );
            __builder.push_bind(__sql_forge_runtime_cat);
            __builder.push("\n        ORDER BY id\n        ");
            let __sql_forge_value_single = __SqlForgeQuery_single {
                inner: __builder,
            };
            __sql_forge_value_single
        }
            .fetch_all(&pool)
            .await
            .expect("select batch failed");
        match (&rows.len(), &3) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::None,
                    );
                }
            }
        };
        match (&rows[0].name, &"Batch A") {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::None,
                    );
                }
            }
        };
        match (&rows[0].price, &price_new(9999, 2)) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::None,
                    );
                }
            }
        };
        match (&rows[1].name, &"Batch B") {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::None,
                    );
                }
            }
        };
        match (&rows[1].price, &price_new(4999, 2)) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::None,
                    );
                }
            }
        };
        match (&rows[2].name, &"Batch C") {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::None,
                    );
                }
            }
        };
        match (&rows[2].price, &price_new(10001, 2)) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::None,
                    );
                }
            }
        };
        {
            let _sql_forge_validator = || {
                let __sql_forge_top_level_category = &("Batch");
                {
                    {
                        let __sql_forge_validator_arg_0 = __sql_forge_top_level_category;
                        let _ = {
                            #[allow(clippy::all)]
                            {
                                use ::sqlx::Arguments as _;
                                let arg0 = &(__sql_forge_validator_arg_0);
                                let mut query_args = <sqlx::sqlite::Sqlite as ::sqlx::database::Database>::Arguments::default();
                                query_args
                                    .reserve(
                                        1usize,
                                        0
                                            + ::sqlx::encode::Encode::<
                                                sqlx::sqlite::Sqlite,
                                            >::size_hint(arg0),
                                    );
                                let query_args = ::core::result::Result::<
                                    _,
                                    ::sqlx::error::BoxDynError,
                                >::Ok(query_args)
                                    .and_then(move |mut query_args| {
                                        query_args.add(arg0).map(move |()| query_args)
                                    });
                                ::sqlx::__query_with_result::<
                                    sqlx::sqlite::Sqlite,
                                    _,
                                >("DELETE FROM products WHERE category = ?", query_args)
                            }
                        };
                    }
                }
            };
            struct __SqlForgeQuery_single {
                inner: sqlx::QueryBuilder<sqlx::Sqlite>,
            }
            impl __SqlForgeQuery_single {
                async fn execute<'e, E>(
                    mut self,
                    executor: E,
                ) -> Result<<sqlx::Sqlite as sqlx::Database>::QueryResult, sqlx::Error>
                where
                    E: sqlx::Executor<'e, Database = sqlx::Sqlite>,
                {
                    self.inner.build().execute(executor).await
                }
            }
            impl sql_forge::SqlForgeQueryExecute for __SqlForgeQuery_single {
                type Db = sqlx::Sqlite;
                fn execute<'e, E>(
                    self,
                    executor: E,
                ) -> impl std::future::Future<
                    Output = Result<
                        <sqlx::Sqlite as sqlx::Database>::QueryResult,
                        sqlx::Error,
                    >,
                > + Send + 'e
                where
                    Self: Sized + 'e,
                    E: sqlx::Executor<'e, Database = sqlx::Sqlite> + Send + 'e,
                    sqlx::Sqlite: 'e,
                {
                    __SqlForgeQuery_single::execute(self, executor)
                }
            }
            let __sql_forge_runtime_category = "Batch";
            let mut __builder: sqlx::QueryBuilder<sqlx::Sqlite> = sqlx::QueryBuilder::new(
                "",
            );
            __builder.push("DELETE FROM products WHERE category = ");
            __builder.push_bind(__sql_forge_runtime_category);
            let __sql_forge_value_single = __SqlForgeQuery_single {
                inner: __builder,
            };
            __sql_forge_value_single
        }
            .execute(&pool)
            .await
            .expect("delete batch failed");
    };
    let mut body = body;
    #[allow(unused_mut)]
    let mut body = unsafe { ::tokio::macros::support::Pin::new_unchecked(&mut body) };
    let body: ::core::pin::Pin<&mut dyn ::core::future::Future<Output = ()>> = body;
    #[allow(
        clippy::expect_used,
        clippy::diverging_sub_expression,
        clippy::needless_return,
        clippy::unwrap_in_result
    )]
    {
        use tokio::runtime::Builder;
        return Builder::new_current_thread()
            .enable_all()
            .build()
            .expect("Failed building the Runtime")
            .block_on(body);
    }
}
extern crate test;
#[rustc_test_marker = "execute_batch_with_params"]
#[doc(hidden)]
pub const execute_batch_with_params: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("execute_batch_with_params"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "tests/tests.rs",
        start_line: 760usize,
        start_col: 10usize,
        end_line: 760usize,
        end_col: 35usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(execute_batch_with_params()),
    ),
};
fn execute_batch_with_params() {
    let body = async {
        let pool = pool().await;
        {
            let _sql_forge_validator = || {
                let __sql_forge_top_level_category = &("BatchWithParams");
                {
                    {
                        let __sql_forge_validator_arg_0 = __sql_forge_top_level_category;
                        let _ = {
                            #[allow(clippy::all)]
                            {
                                use ::sqlx::Arguments as _;
                                let arg0 = &(__sql_forge_validator_arg_0);
                                let mut query_args = <sqlx::sqlite::Sqlite as ::sqlx::database::Database>::Arguments::default();
                                query_args
                                    .reserve(
                                        1usize,
                                        0
                                            + ::sqlx::encode::Encode::<
                                                sqlx::sqlite::Sqlite,
                                            >::size_hint(arg0),
                                    );
                                let query_args = ::core::result::Result::<
                                    _,
                                    ::sqlx::error::BoxDynError,
                                >::Ok(query_args)
                                    .and_then(move |mut query_args| {
                                        query_args.add(arg0).map(move |()| query_args)
                                    });
                                ::sqlx::__query_with_result::<
                                    sqlx::sqlite::Sqlite,
                                    _,
                                >("DELETE FROM products WHERE category = ?", query_args)
                            }
                        };
                    }
                }
            };
            struct __SqlForgeQuery_single {
                inner: sqlx::QueryBuilder<sqlx::Sqlite>,
            }
            impl __SqlForgeQuery_single {
                async fn execute<'e, E>(
                    mut self,
                    executor: E,
                ) -> Result<<sqlx::Sqlite as sqlx::Database>::QueryResult, sqlx::Error>
                where
                    E: sqlx::Executor<'e, Database = sqlx::Sqlite>,
                {
                    self.inner.build().execute(executor).await
                }
            }
            impl sql_forge::SqlForgeQueryExecute for __SqlForgeQuery_single {
                type Db = sqlx::Sqlite;
                fn execute<'e, E>(
                    self,
                    executor: E,
                ) -> impl std::future::Future<
                    Output = Result<
                        <sqlx::Sqlite as sqlx::Database>::QueryResult,
                        sqlx::Error,
                    >,
                > + Send + 'e
                where
                    Self: Sized + 'e,
                    E: sqlx::Executor<'e, Database = sqlx::Sqlite> + Send + 'e,
                    sqlx::Sqlite: 'e,
                {
                    __SqlForgeQuery_single::execute(self, executor)
                }
            }
            let __sql_forge_runtime_category = "BatchWithParams";
            let mut __builder: sqlx::QueryBuilder<sqlx::Sqlite> = sqlx::QueryBuilder::new(
                "",
            );
            __builder.push("DELETE FROM products WHERE category = ");
            __builder.push_bind(__sql_forge_runtime_category);
            let __sql_forge_value_single = __SqlForgeQuery_single {
                inner: __builder,
            };
            __sql_forge_value_single
        }
            .execute(&pool)
            .await
            .ok();
        let batch_items = ::alloc::boxed::box_assume_init_into_vec_unsafe(
            ::alloc::intrinsics::write_box_via_move(
                ::alloc::boxed::Box::new_uninit(),
                [
                    BatchItem {
                        name: "Batch B".to_string(),
                        price: price_new(4999, 2),
                    },
                    BatchItem {
                        name: "Batch C".to_string(),
                        price: price_new(10001, 2),
                    },
                ],
            ),
        );
        {
            let _sql_forge_validator = || {
                let __sql_forge_top_level_name = &("Batch A Param".to_string());
                let __sql_forge_top_level_price = &(price_new(8999, 2));
                {
                    {
                        let __sql_forge_validator_arg_0 = __sql_forge_top_level_name;
                        let __sql_forge_validator_arg_1 = __sql_forge_top_level_price;
                        let _ = {
                            #[allow(clippy::all)]
                            {
                                use ::sqlx::Arguments as _;
                                let arg0 = &(__sql_forge_validator_arg_0);
                                let arg1 = &(__sql_forge_validator_arg_1);
                                let arg2 = &(batch_items[0].name);
                                let arg3 = &(batch_items[0].price);
                                let arg4 = &(batch_items[0].name);
                                let arg5 = &(batch_items[0].price);
                                let arg6 = &(batch_items[0].name);
                                let arg7 = &(batch_items[0].price);
                                let mut query_args = <sqlx::sqlite::Sqlite as ::sqlx::database::Database>::Arguments::default();
                                query_args
                                    .reserve(
                                        8usize,
                                        0
                                            + ::sqlx::encode::Encode::<
                                                sqlx::sqlite::Sqlite,
                                            >::size_hint(arg0)
                                            + ::sqlx::encode::Encode::<
                                                sqlx::sqlite::Sqlite,
                                            >::size_hint(arg1)
                                            + ::sqlx::encode::Encode::<
                                                sqlx::sqlite::Sqlite,
                                            >::size_hint(arg2)
                                            + ::sqlx::encode::Encode::<
                                                sqlx::sqlite::Sqlite,
                                            >::size_hint(arg3)
                                            + ::sqlx::encode::Encode::<
                                                sqlx::sqlite::Sqlite,
                                            >::size_hint(arg4)
                                            + ::sqlx::encode::Encode::<
                                                sqlx::sqlite::Sqlite,
                                            >::size_hint(arg5)
                                            + ::sqlx::encode::Encode::<
                                                sqlx::sqlite::Sqlite,
                                            >::size_hint(arg6)
                                            + ::sqlx::encode::Encode::<
                                                sqlx::sqlite::Sqlite,
                                            >::size_hint(arg7),
                                    );
                                let query_args = ::core::result::Result::<
                                    _,
                                    ::sqlx::error::BoxDynError,
                                >::Ok(query_args)
                                    .and_then(move |mut query_args| {
                                        query_args.add(arg0).map(move |()| query_args)
                                    })
                                    .and_then(move |mut query_args| {
                                        query_args.add(arg1).map(move |()| query_args)
                                    })
                                    .and_then(move |mut query_args| {
                                        query_args.add(arg2).map(move |()| query_args)
                                    })
                                    .and_then(move |mut query_args| {
                                        query_args.add(arg3).map(move |()| query_args)
                                    })
                                    .and_then(move |mut query_args| {
                                        query_args.add(arg4).map(move |()| query_args)
                                    })
                                    .and_then(move |mut query_args| {
                                        query_args.add(arg5).map(move |()| query_args)
                                    })
                                    .and_then(move |mut query_args| {
                                        query_args.add(arg6).map(move |()| query_args)
                                    })
                                    .and_then(move |mut query_args| {
                                        query_args.add(arg7).map(move |()| query_args)
                                    });
                                ::sqlx::__query_with_result::<
                                    sqlx::sqlite::Sqlite,
                                    _,
                                >(
                                    "\n        INSERT INTO products (name, price, stock, category)\n        VALUES (?, ?, 10, 'BatchWithParams'), (?, ?, 10, 'BatchWithParams'), (?, ?, 10, 'BatchWithParams'), (?, ?, 10, 'BatchWithParams')\n        ",
                                    query_args,
                                )
                            }
                        };
                    }
                }
            };
            struct __SqlForgeQuery_single {
                inner: sqlx::QueryBuilder<sqlx::Sqlite>,
            }
            impl __SqlForgeQuery_single {
                async fn execute<'e, E>(
                    mut self,
                    executor: E,
                ) -> Result<<sqlx::Sqlite as sqlx::Database>::QueryResult, sqlx::Error>
                where
                    E: sqlx::Executor<'e, Database = sqlx::Sqlite>,
                {
                    self.inner.build().execute(executor).await
                }
            }
            impl sql_forge::SqlForgeQueryExecute for __SqlForgeQuery_single {
                type Db = sqlx::Sqlite;
                fn execute<'e, E>(
                    self,
                    executor: E,
                ) -> impl std::future::Future<
                    Output = Result<
                        <sqlx::Sqlite as sqlx::Database>::QueryResult,
                        sqlx::Error,
                    >,
                > + Send + 'e
                where
                    Self: Sized + 'e,
                    E: sqlx::Executor<'e, Database = sqlx::Sqlite> + Send + 'e,
                    sqlx::Sqlite: 'e,
                {
                    __SqlForgeQuery_single::execute(self, executor)
                }
            }
            let __sql_forge_runtime_name = "Batch A Param".to_string();
            let __sql_forge_runtime_price = price_new(8999, 2);
            let mut __builder: sqlx::QueryBuilder<sqlx::Sqlite> = sqlx::QueryBuilder::new(
                "",
            );
            __builder
                .push(
                    "\n        INSERT INTO products (name, price, stock, category)\n        VALUES (",
                );
            __builder.push_bind(__sql_forge_runtime_name);
            __builder.push(", ");
            __builder.push_bind(__sql_forge_runtime_price);
            __builder.push(", 10, 'BatchWithParams'), ");
            {
                let mut __first = true;
                for __item in batch_items {
                    if !__first {
                        __builder.push(", ");
                    }
                    __first = false;
                    __builder.push("(");
                    __builder.push_bind(__item.name);
                    __builder.push(", ");
                    __builder.push_bind(__item.price);
                    __builder.push(", 10, 'BatchWithParams')");
                }
            }
            __builder.push("\n        ");
            let __sql_forge_value_single = __SqlForgeQuery_single {
                inner: __builder,
            };
            __sql_forge_value_single
        }
            .execute(&pool)
            .await
            .expect("batch insert failed");
        let rows: Vec<BatchItem> = {
            let _sql_forge_validator = || {
                let __sql_forge_top_level_cat = &("BatchWithParams");
                {
                    type __SqlForgeModel = BatchItem;
                    {
                        let __sql_forge_validator_arg_0 = __sql_forge_top_level_cat;
                        let _ = {
                            {
                                #[allow(clippy::all)]
                                {
                                    use ::sqlx::Arguments as _;
                                    let arg0 = &(__sql_forge_validator_arg_0);
                                    let mut query_args = <sqlx::sqlite::Sqlite as ::sqlx::database::Database>::Arguments::default();
                                    query_args
                                        .reserve(
                                            1usize,
                                            0
                                                + ::sqlx::encode::Encode::<
                                                    sqlx::sqlite::Sqlite,
                                                >::size_hint(arg0),
                                        );
                                    let query_args = ::core::result::Result::<
                                        _,
                                        ::sqlx::error::BoxDynError,
                                    >::Ok(query_args)
                                        .and_then(move |mut query_args| {
                                            query_args.add(arg0).map(move |()| query_args)
                                        });
                                    ::sqlx::__query_with_result::<
                                        sqlx::sqlite::Sqlite,
                                        _,
                                    >(
                                            "\n        SELECT name, price FROM products\n        WHERE category = ?\n        ORDER BY id\n        ",
                                            query_args,
                                        )
                                        .try_map(|row: sqlx::sqlite::SqliteRow| {
                                            use ::sqlx::Row as _;
                                            #[allow(non_snake_case)]
                                            let sqlx_query_as_name = row
                                                .try_get_unchecked::<String, _>(0usize)?
                                                .into();
                                            #[allow(non_snake_case)]
                                            let sqlx_query_as_price = row
                                                .try_get_unchecked::<i64, _>(1usize)?
                                                .into();
                                            ::std::result::Result::Ok(__SqlForgeModel {
                                                name: sqlx_query_as_name,
                                                price: sqlx_query_as_price,
                                            })
                                        })
                                }
                            }
                        };
                    }
                }
            };
            struct __SqlForgeQuery_single {
                inner: sqlx::QueryBuilder<sqlx::Sqlite>,
            }
            impl __SqlForgeQuery_single {
                async fn fetch_all<'e, E>(
                    mut self,
                    executor: E,
                ) -> Result<Vec<BatchItem>, sqlx::Error>
                where
                    E: sqlx::Executor<'e, Database = sqlx::Sqlite>,
                {
                    self.inner.build_query_as::<BatchItem>().fetch_all(executor).await
                }
                async fn fetch_one<'e, E>(
                    mut self,
                    executor: E,
                ) -> Result<BatchItem, sqlx::Error>
                where
                    E: sqlx::Executor<'e, Database = sqlx::Sqlite>,
                {
                    self.inner.build_query_as::<BatchItem>().fetch_one(executor).await
                }
                async fn fetch_optional<'e, E>(
                    mut self,
                    executor: E,
                ) -> Result<Option<BatchItem>, sqlx::Error>
                where
                    E: sqlx::Executor<'e, Database = sqlx::Sqlite>,
                {
                    self.inner
                        .build_query_as::<BatchItem>()
                        .fetch_optional(executor)
                        .await
                }
                async fn execute<'e, E>(
                    mut self,
                    executor: E,
                ) -> Result<<sqlx::Sqlite as sqlx::Database>::QueryResult, sqlx::Error>
                where
                    E: sqlx::Executor<'e, Database = sqlx::Sqlite>,
                {
                    self.inner.build().execute(executor).await
                }
            }
            impl sql_forge::SqlForgeQuery<BatchItem> for __SqlForgeQuery_single {
                type Db = sqlx::Sqlite;
                fn fetch_all<'e, E>(
                    self,
                    executor: E,
                ) -> impl std::future::Future<
                    Output = Result<Vec<BatchItem>, sqlx::Error>,
                > + Send + 'e
                where
                    Self: Sized + 'e,
                    E: sqlx::Executor<'e, Database = sqlx::Sqlite> + Send + 'e,
                    sqlx::Sqlite: 'e,
                {
                    __SqlForgeQuery_single::fetch_all(self, executor)
                }
                fn fetch_one<'e, E>(
                    self,
                    executor: E,
                ) -> impl std::future::Future<
                    Output = Result<BatchItem, sqlx::Error>,
                > + Send + 'e
                where
                    Self: Sized + 'e,
                    E: sqlx::Executor<'e, Database = sqlx::Sqlite> + Send + 'e,
                    sqlx::Sqlite: 'e,
                {
                    __SqlForgeQuery_single::fetch_one(self, executor)
                }
                fn fetch_optional<'e, E>(
                    self,
                    executor: E,
                ) -> impl std::future::Future<
                    Output = Result<Option<BatchItem>, sqlx::Error>,
                > + Send + 'e
                where
                    Self: Sized + 'e,
                    E: sqlx::Executor<'e, Database = sqlx::Sqlite> + Send + 'e,
                    sqlx::Sqlite: 'e,
                {
                    __SqlForgeQuery_single::fetch_optional(self, executor)
                }
                fn execute<'e, E>(
                    self,
                    executor: E,
                ) -> impl std::future::Future<
                    Output = Result<
                        <sqlx::Sqlite as sqlx::Database>::QueryResult,
                        sqlx::Error,
                    >,
                > + Send + 'e
                where
                    Self: Sized + 'e,
                    E: sqlx::Executor<'e, Database = sqlx::Sqlite> + Send + 'e,
                    sqlx::Sqlite: 'e,
                {
                    __SqlForgeQuery_single::execute(self, executor)
                }
            }
            let __sql_forge_runtime_cat = "BatchWithParams";
            let mut __builder: sqlx::QueryBuilder<sqlx::Sqlite> = sqlx::QueryBuilder::new(
                "",
            );
            __builder
                .push(
                    "\n        SELECT name, price FROM products\n        WHERE category = ",
                );
            __builder.push_bind(__sql_forge_runtime_cat);
            __builder.push("\n        ORDER BY id\n        ");
            let __sql_forge_value_single = __SqlForgeQuery_single {
                inner: __builder,
            };
            __sql_forge_value_single
        }
            .fetch_all(&pool)
            .await
            .expect("select batch failed");
        match (&rows.len(), &3) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::None,
                    );
                }
            }
        };
        match (&rows[0].name, &"Batch A Param") {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::None,
                    );
                }
            }
        };
        match (&rows[0].price, &price_new(8999, 2)) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::None,
                    );
                }
            }
        };
        match (&rows[1].name, &"Batch B") {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::None,
                    );
                }
            }
        };
        match (&rows[1].price, &price_new(4999, 2)) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::None,
                    );
                }
            }
        };
        match (&rows[2].name, &"Batch C") {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::None,
                    );
                }
            }
        };
        match (&rows[2].price, &price_new(10001, 2)) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::None,
                    );
                }
            }
        };
        {
            let _sql_forge_validator = || {
                let __sql_forge_top_level_category = &("BatchWithParams");
                {
                    {
                        let __sql_forge_validator_arg_0 = __sql_forge_top_level_category;
                        let _ = {
                            #[allow(clippy::all)]
                            {
                                use ::sqlx::Arguments as _;
                                let arg0 = &(__sql_forge_validator_arg_0);
                                let mut query_args = <sqlx::sqlite::Sqlite as ::sqlx::database::Database>::Arguments::default();
                                query_args
                                    .reserve(
                                        1usize,
                                        0
                                            + ::sqlx::encode::Encode::<
                                                sqlx::sqlite::Sqlite,
                                            >::size_hint(arg0),
                                    );
                                let query_args = ::core::result::Result::<
                                    _,
                                    ::sqlx::error::BoxDynError,
                                >::Ok(query_args)
                                    .and_then(move |mut query_args| {
                                        query_args.add(arg0).map(move |()| query_args)
                                    });
                                ::sqlx::__query_with_result::<
                                    sqlx::sqlite::Sqlite,
                                    _,
                                >("DELETE FROM products WHERE category = ?", query_args)
                            }
                        };
                    }
                }
            };
            struct __SqlForgeQuery_single {
                inner: sqlx::QueryBuilder<sqlx::Sqlite>,
            }
            impl __SqlForgeQuery_single {
                async fn execute<'e, E>(
                    mut self,
                    executor: E,
                ) -> Result<<sqlx::Sqlite as sqlx::Database>::QueryResult, sqlx::Error>
                where
                    E: sqlx::Executor<'e, Database = sqlx::Sqlite>,
                {
                    self.inner.build().execute(executor).await
                }
            }
            impl sql_forge::SqlForgeQueryExecute for __SqlForgeQuery_single {
                type Db = sqlx::Sqlite;
                fn execute<'e, E>(
                    self,
                    executor: E,
                ) -> impl std::future::Future<
                    Output = Result<
                        <sqlx::Sqlite as sqlx::Database>::QueryResult,
                        sqlx::Error,
                    >,
                > + Send + 'e
                where
                    Self: Sized + 'e,
                    E: sqlx::Executor<'e, Database = sqlx::Sqlite> + Send + 'e,
                    sqlx::Sqlite: 'e,
                {
                    __SqlForgeQuery_single::execute(self, executor)
                }
            }
            let __sql_forge_runtime_category = "BatchWithParams";
            let mut __builder: sqlx::QueryBuilder<sqlx::Sqlite> = sqlx::QueryBuilder::new(
                "",
            );
            __builder.push("DELETE FROM products WHERE category = ");
            __builder.push_bind(__sql_forge_runtime_category);
            let __sql_forge_value_single = __SqlForgeQuery_single {
                inner: __builder,
            };
            __sql_forge_value_single
        }
            .execute(&pool)
            .await
            .expect("delete batch failed");
    };
    let mut body = body;
    #[allow(unused_mut)]
    let mut body = unsafe { ::tokio::macros::support::Pin::new_unchecked(&mut body) };
    let body: ::core::pin::Pin<&mut dyn ::core::future::Future<Output = ()>> = body;
    #[allow(
        clippy::expect_used,
        clippy::diverging_sub_expression,
        clippy::needless_return,
        clippy::unwrap_in_result
    )]
    {
        use tokio::runtime::Builder;
        return Builder::new_current_thread()
            .enable_all()
            .build()
            .expect("Failed building the Runtime")
            .block_on(body);
    }
}
struct BatchFullItem {
    name: String,
    price: Price,
    stock: i64,
    category: String,
}
#[automatically_derived]
impl<'a, R: ::sqlx::Row> ::sqlx::FromRow<'a, R> for BatchFullItem
where
    &'a ::std::primitive::str: ::sqlx::ColumnIndex<R>,
    String: ::sqlx::decode::Decode<'a, R::Database>,
    String: ::sqlx::types::Type<R::Database>,
    Price: ::sqlx::decode::Decode<'a, R::Database>,
    Price: ::sqlx::types::Type<R::Database>,
    i64: ::sqlx::decode::Decode<'a, R::Database>,
    i64: ::sqlx::types::Type<R::Database>,
    String: ::sqlx::decode::Decode<'a, R::Database>,
    String: ::sqlx::types::Type<R::Database>,
{
    fn from_row(__row: &'a R) -> ::sqlx::Result<Self> {
        let name: String = __row.try_get("name")?;
        let price: Price = __row.try_get("price")?;
        let stock: i64 = __row.try_get("stock")?;
        let category: String = __row.try_get("category")?;
        ::std::result::Result::Ok(BatchFullItem {
            name,
            price,
            stock,
            category,
        })
    }
}
extern crate test;
#[rustc_test_marker = "execute_batch_full"]
#[doc(hidden)]
pub const execute_batch_full: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("execute_batch_full"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "tests/tests.rs",
        start_line: 837usize,
        start_col: 10usize,
        end_line: 837usize,
        end_col: 28usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(execute_batch_full()),
    ),
};
fn execute_batch_full() {
    let body = async {
        let pool = pool().await;
        {
            let _sql_forge_validator = || {
                let __sql_forge_top_level_category = &("BatchFull");
                {
                    {
                        let __sql_forge_validator_arg_0 = __sql_forge_top_level_category;
                        let _ = {
                            #[allow(clippy::all)]
                            {
                                use ::sqlx::Arguments as _;
                                let arg0 = &(__sql_forge_validator_arg_0);
                                let mut query_args = <sqlx::sqlite::Sqlite as ::sqlx::database::Database>::Arguments::default();
                                query_args
                                    .reserve(
                                        1usize,
                                        0
                                            + ::sqlx::encode::Encode::<
                                                sqlx::sqlite::Sqlite,
                                            >::size_hint(arg0),
                                    );
                                let query_args = ::core::result::Result::<
                                    _,
                                    ::sqlx::error::BoxDynError,
                                >::Ok(query_args)
                                    .and_then(move |mut query_args| {
                                        query_args.add(arg0).map(move |()| query_args)
                                    });
                                ::sqlx::__query_with_result::<
                                    sqlx::sqlite::Sqlite,
                                    _,
                                >("DELETE FROM products WHERE category = ?", query_args)
                            }
                        };
                    }
                }
            };
            struct __SqlForgeQuery_single {
                inner: sqlx::QueryBuilder<sqlx::Sqlite>,
            }
            impl __SqlForgeQuery_single {
                async fn execute<'e, E>(
                    mut self,
                    executor: E,
                ) -> Result<<sqlx::Sqlite as sqlx::Database>::QueryResult, sqlx::Error>
                where
                    E: sqlx::Executor<'e, Database = sqlx::Sqlite>,
                {
                    self.inner.build().execute(executor).await
                }
            }
            impl sql_forge::SqlForgeQueryExecute for __SqlForgeQuery_single {
                type Db = sqlx::Sqlite;
                fn execute<'e, E>(
                    self,
                    executor: E,
                ) -> impl std::future::Future<
                    Output = Result<
                        <sqlx::Sqlite as sqlx::Database>::QueryResult,
                        sqlx::Error,
                    >,
                > + Send + 'e
                where
                    Self: Sized + 'e,
                    E: sqlx::Executor<'e, Database = sqlx::Sqlite> + Send + 'e,
                    sqlx::Sqlite: 'e,
                {
                    __SqlForgeQuery_single::execute(self, executor)
                }
            }
            let __sql_forge_runtime_category = "BatchFull";
            let mut __builder: sqlx::QueryBuilder<sqlx::Sqlite> = sqlx::QueryBuilder::new(
                "",
            );
            __builder.push("DELETE FROM products WHERE category = ");
            __builder.push_bind(__sql_forge_runtime_category);
            let __sql_forge_value_single = __SqlForgeQuery_single {
                inner: __builder,
            };
            __sql_forge_value_single
        }
            .execute(&pool)
            .await
            .ok();
        let items = ::alloc::boxed::box_assume_init_into_vec_unsafe(
            ::alloc::intrinsics::write_box_via_move(
                ::alloc::boxed::Box::new_uninit(),
                [
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
                ],
            ),
        );
        {
            let _sql_forge_validator = || {
                {
                    {
                        let _ = {
                            #[allow(clippy::all)]
                            {
                                use ::sqlx::Arguments as _;
                                let arg0 = &(items[0].name);
                                let arg1 = &(items[0].price);
                                let arg2 = &(items[0].stock);
                                let arg3 = &(items[0].category);
                                let arg4 = &(items[0].name);
                                let arg5 = &(items[0].price);
                                let arg6 = &(items[0].stock);
                                let arg7 = &(items[0].category);
                                let arg8 = &(items[0].name);
                                let arg9 = &(items[0].price);
                                let arg10 = &(items[0].stock);
                                let arg11 = &(items[0].category);
                                let mut query_args = <sqlx::sqlite::Sqlite as ::sqlx::database::Database>::Arguments::default();
                                query_args
                                    .reserve(
                                        12usize,
                                        0
                                            + ::sqlx::encode::Encode::<
                                                sqlx::sqlite::Sqlite,
                                            >::size_hint(arg0)
                                            + ::sqlx::encode::Encode::<
                                                sqlx::sqlite::Sqlite,
                                            >::size_hint(arg1)
                                            + ::sqlx::encode::Encode::<
                                                sqlx::sqlite::Sqlite,
                                            >::size_hint(arg2)
                                            + ::sqlx::encode::Encode::<
                                                sqlx::sqlite::Sqlite,
                                            >::size_hint(arg3)
                                            + ::sqlx::encode::Encode::<
                                                sqlx::sqlite::Sqlite,
                                            >::size_hint(arg4)
                                            + ::sqlx::encode::Encode::<
                                                sqlx::sqlite::Sqlite,
                                            >::size_hint(arg5)
                                            + ::sqlx::encode::Encode::<
                                                sqlx::sqlite::Sqlite,
                                            >::size_hint(arg6)
                                            + ::sqlx::encode::Encode::<
                                                sqlx::sqlite::Sqlite,
                                            >::size_hint(arg7)
                                            + ::sqlx::encode::Encode::<
                                                sqlx::sqlite::Sqlite,
                                            >::size_hint(arg8)
                                            + ::sqlx::encode::Encode::<
                                                sqlx::sqlite::Sqlite,
                                            >::size_hint(arg9)
                                            + ::sqlx::encode::Encode::<
                                                sqlx::sqlite::Sqlite,
                                            >::size_hint(arg10)
                                            + ::sqlx::encode::Encode::<
                                                sqlx::sqlite::Sqlite,
                                            >::size_hint(arg11),
                                    );
                                let query_args = ::core::result::Result::<
                                    _,
                                    ::sqlx::error::BoxDynError,
                                >::Ok(query_args)
                                    .and_then(move |mut query_args| {
                                        query_args.add(arg0).map(move |()| query_args)
                                    })
                                    .and_then(move |mut query_args| {
                                        query_args.add(arg1).map(move |()| query_args)
                                    })
                                    .and_then(move |mut query_args| {
                                        query_args.add(arg2).map(move |()| query_args)
                                    })
                                    .and_then(move |mut query_args| {
                                        query_args.add(arg3).map(move |()| query_args)
                                    })
                                    .and_then(move |mut query_args| {
                                        query_args.add(arg4).map(move |()| query_args)
                                    })
                                    .and_then(move |mut query_args| {
                                        query_args.add(arg5).map(move |()| query_args)
                                    })
                                    .and_then(move |mut query_args| {
                                        query_args.add(arg6).map(move |()| query_args)
                                    })
                                    .and_then(move |mut query_args| {
                                        query_args.add(arg7).map(move |()| query_args)
                                    })
                                    .and_then(move |mut query_args| {
                                        query_args.add(arg8).map(move |()| query_args)
                                    })
                                    .and_then(move |mut query_args| {
                                        query_args.add(arg9).map(move |()| query_args)
                                    })
                                    .and_then(move |mut query_args| {
                                        query_args.add(arg10).map(move |()| query_args)
                                    })
                                    .and_then(move |mut query_args| {
                                        query_args.add(arg11).map(move |()| query_args)
                                    });
                                ::sqlx::__query_with_result::<
                                    sqlx::sqlite::Sqlite,
                                    _,
                                >(
                                    "\n        INSERT INTO products (name, price, stock, category)\n        VALUES (?, ?, ?, ?), (?, ?, ?, ?), (?, ?, ?, ?)\n        ",
                                    query_args,
                                )
                            }
                        };
                    }
                }
            };
            struct __SqlForgeQuery_single {
                inner: sqlx::QueryBuilder<sqlx::Sqlite>,
            }
            impl __SqlForgeQuery_single {
                async fn execute<'e, E>(
                    mut self,
                    executor: E,
                ) -> Result<<sqlx::Sqlite as sqlx::Database>::QueryResult, sqlx::Error>
                where
                    E: sqlx::Executor<'e, Database = sqlx::Sqlite>,
                {
                    self.inner.build().execute(executor).await
                }
            }
            impl sql_forge::SqlForgeQueryExecute for __SqlForgeQuery_single {
                type Db = sqlx::Sqlite;
                fn execute<'e, E>(
                    self,
                    executor: E,
                ) -> impl std::future::Future<
                    Output = Result<
                        <sqlx::Sqlite as sqlx::Database>::QueryResult,
                        sqlx::Error,
                    >,
                > + Send + 'e
                where
                    Self: Sized + 'e,
                    E: sqlx::Executor<'e, Database = sqlx::Sqlite> + Send + 'e,
                    sqlx::Sqlite: 'e,
                {
                    __SqlForgeQuery_single::execute(self, executor)
                }
            }
            let mut __builder: sqlx::QueryBuilder<sqlx::Sqlite> = sqlx::QueryBuilder::new(
                "",
            );
            __builder
                .push(
                    "\n        INSERT INTO products (name, price, stock, category)\n        VALUES ",
                );
            {
                let mut __first = true;
                for __item in items {
                    if !__first {
                        __builder.push(", ");
                    }
                    __first = false;
                    __builder.push("(");
                    __builder.push_bind(__item.name);
                    __builder.push(", ");
                    __builder.push_bind(__item.price);
                    __builder.push(", ");
                    __builder.push_bind(__item.stock);
                    __builder.push(", ");
                    __builder.push_bind(__item.category);
                    __builder.push(")");
                }
            }
            __builder.push("\n        ");
            let __sql_forge_value_single = __SqlForgeQuery_single {
                inner: __builder,
            };
            __sql_forge_value_single
        }
            .execute(&pool)
            .await
            .expect("batch insert failed");
        let rows: Vec<BatchFullItem> = {
            let _sql_forge_validator = || {
                let __sql_forge_top_level_cat = &("BatchFull");
                {
                    type __SqlForgeModel = BatchFullItem;
                    {
                        let __sql_forge_validator_arg_0 = __sql_forge_top_level_cat;
                        let _ = {
                            {
                                #[allow(clippy::all)]
                                {
                                    use ::sqlx::Arguments as _;
                                    let arg0 = &(__sql_forge_validator_arg_0);
                                    let mut query_args = <sqlx::sqlite::Sqlite as ::sqlx::database::Database>::Arguments::default();
                                    query_args
                                        .reserve(
                                            1usize,
                                            0
                                                + ::sqlx::encode::Encode::<
                                                    sqlx::sqlite::Sqlite,
                                                >::size_hint(arg0),
                                        );
                                    let query_args = ::core::result::Result::<
                                        _,
                                        ::sqlx::error::BoxDynError,
                                    >::Ok(query_args)
                                        .and_then(move |mut query_args| {
                                            query_args.add(arg0).map(move |()| query_args)
                                        });
                                    ::sqlx::__query_with_result::<
                                        sqlx::sqlite::Sqlite,
                                        _,
                                    >(
                                            "\n        SELECT name, price, stock, category FROM products\n        WHERE category = ?\n        ORDER BY id\n        ",
                                            query_args,
                                        )
                                        .try_map(|row: sqlx::sqlite::SqliteRow| {
                                            use ::sqlx::Row as _;
                                            #[allow(non_snake_case)]
                                            let sqlx_query_as_name = row
                                                .try_get_unchecked::<String, _>(0usize)?
                                                .into();
                                            #[allow(non_snake_case)]
                                            let sqlx_query_as_price = row
                                                .try_get_unchecked::<i64, _>(1usize)?
                                                .into();
                                            #[allow(non_snake_case)]
                                            let sqlx_query_as_stock = row
                                                .try_get_unchecked::<i64, _>(2usize)?
                                                .into();
                                            #[allow(non_snake_case)]
                                            let sqlx_query_as_category = row
                                                .try_get_unchecked::<String, _>(3usize)?
                                                .into();
                                            ::std::result::Result::Ok(__SqlForgeModel {
                                                name: sqlx_query_as_name,
                                                price: sqlx_query_as_price,
                                                stock: sqlx_query_as_stock,
                                                category: sqlx_query_as_category,
                                            })
                                        })
                                }
                            }
                        };
                    }
                }
            };
            struct __SqlForgeQuery_single {
                inner: sqlx::QueryBuilder<sqlx::Sqlite>,
            }
            impl __SqlForgeQuery_single {
                async fn fetch_all<'e, E>(
                    mut self,
                    executor: E,
                ) -> Result<Vec<BatchFullItem>, sqlx::Error>
                where
                    E: sqlx::Executor<'e, Database = sqlx::Sqlite>,
                {
                    self.inner
                        .build_query_as::<BatchFullItem>()
                        .fetch_all(executor)
                        .await
                }
                async fn fetch_one<'e, E>(
                    mut self,
                    executor: E,
                ) -> Result<BatchFullItem, sqlx::Error>
                where
                    E: sqlx::Executor<'e, Database = sqlx::Sqlite>,
                {
                    self.inner
                        .build_query_as::<BatchFullItem>()
                        .fetch_one(executor)
                        .await
                }
                async fn fetch_optional<'e, E>(
                    mut self,
                    executor: E,
                ) -> Result<Option<BatchFullItem>, sqlx::Error>
                where
                    E: sqlx::Executor<'e, Database = sqlx::Sqlite>,
                {
                    self.inner
                        .build_query_as::<BatchFullItem>()
                        .fetch_optional(executor)
                        .await
                }
                async fn execute<'e, E>(
                    mut self,
                    executor: E,
                ) -> Result<<sqlx::Sqlite as sqlx::Database>::QueryResult, sqlx::Error>
                where
                    E: sqlx::Executor<'e, Database = sqlx::Sqlite>,
                {
                    self.inner.build().execute(executor).await
                }
            }
            impl sql_forge::SqlForgeQuery<BatchFullItem> for __SqlForgeQuery_single {
                type Db = sqlx::Sqlite;
                fn fetch_all<'e, E>(
                    self,
                    executor: E,
                ) -> impl std::future::Future<
                    Output = Result<Vec<BatchFullItem>, sqlx::Error>,
                > + Send + 'e
                where
                    Self: Sized + 'e,
                    E: sqlx::Executor<'e, Database = sqlx::Sqlite> + Send + 'e,
                    sqlx::Sqlite: 'e,
                {
                    __SqlForgeQuery_single::fetch_all(self, executor)
                }
                fn fetch_one<'e, E>(
                    self,
                    executor: E,
                ) -> impl std::future::Future<
                    Output = Result<BatchFullItem, sqlx::Error>,
                > + Send + 'e
                where
                    Self: Sized + 'e,
                    E: sqlx::Executor<'e, Database = sqlx::Sqlite> + Send + 'e,
                    sqlx::Sqlite: 'e,
                {
                    __SqlForgeQuery_single::fetch_one(self, executor)
                }
                fn fetch_optional<'e, E>(
                    self,
                    executor: E,
                ) -> impl std::future::Future<
                    Output = Result<Option<BatchFullItem>, sqlx::Error>,
                > + Send + 'e
                where
                    Self: Sized + 'e,
                    E: sqlx::Executor<'e, Database = sqlx::Sqlite> + Send + 'e,
                    sqlx::Sqlite: 'e,
                {
                    __SqlForgeQuery_single::fetch_optional(self, executor)
                }
                fn execute<'e, E>(
                    self,
                    executor: E,
                ) -> impl std::future::Future<
                    Output = Result<
                        <sqlx::Sqlite as sqlx::Database>::QueryResult,
                        sqlx::Error,
                    >,
                > + Send + 'e
                where
                    Self: Sized + 'e,
                    E: sqlx::Executor<'e, Database = sqlx::Sqlite> + Send + 'e,
                    sqlx::Sqlite: 'e,
                {
                    __SqlForgeQuery_single::execute(self, executor)
                }
            }
            let __sql_forge_runtime_cat = "BatchFull";
            let mut __builder: sqlx::QueryBuilder<sqlx::Sqlite> = sqlx::QueryBuilder::new(
                "",
            );
            __builder
                .push(
                    "\n        SELECT name, price, stock, category FROM products\n        WHERE category = ",
                );
            __builder.push_bind(__sql_forge_runtime_cat);
            __builder.push("\n        ORDER BY id\n        ");
            let __sql_forge_value_single = __SqlForgeQuery_single {
                inner: __builder,
            };
            __sql_forge_value_single
        }
            .fetch_all(&pool)
            .await
            .expect("select batch full failed");
        match (&rows.len(), &2) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::None,
                    );
                }
            }
        };
        match (&rows[0].name, &"Batch A") {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::None,
                    );
                }
            }
        };
        match (&rows[0].price, &price_new(9999, 2)) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::None,
                    );
                }
            }
        };
        match (&rows[0].stock, &10i64) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::None,
                    );
                }
            }
        };
        match (&rows[0].category, &"BatchFull") {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::None,
                    );
                }
            }
        };
        match (&rows[1].name, &"Batch B") {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::None,
                    );
                }
            }
        };
        match (&rows[1].price, &price_new(4999, 2)) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::None,
                    );
                }
            }
        };
        match (&rows[1].stock, &10i64) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::None,
                    );
                }
            }
        };
        match (&rows[1].category, &"BatchFull") {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::None,
                    );
                }
            }
        };
        {
            let _sql_forge_validator = || {
                let __sql_forge_top_level_category = &("BatchFull");
                {
                    {
                        let __sql_forge_validator_arg_0 = __sql_forge_top_level_category;
                        let _ = {
                            #[allow(clippy::all)]
                            {
                                use ::sqlx::Arguments as _;
                                let arg0 = &(__sql_forge_validator_arg_0);
                                let mut query_args = <sqlx::sqlite::Sqlite as ::sqlx::database::Database>::Arguments::default();
                                query_args
                                    .reserve(
                                        1usize,
                                        0
                                            + ::sqlx::encode::Encode::<
                                                sqlx::sqlite::Sqlite,
                                            >::size_hint(arg0),
                                    );
                                let query_args = ::core::result::Result::<
                                    _,
                                    ::sqlx::error::BoxDynError,
                                >::Ok(query_args)
                                    .and_then(move |mut query_args| {
                                        query_args.add(arg0).map(move |()| query_args)
                                    });
                                ::sqlx::__query_with_result::<
                                    sqlx::sqlite::Sqlite,
                                    _,
                                >("DELETE FROM products WHERE category = ?", query_args)
                            }
                        };
                    }
                }
            };
            struct __SqlForgeQuery_single {
                inner: sqlx::QueryBuilder<sqlx::Sqlite>,
            }
            impl __SqlForgeQuery_single {
                async fn execute<'e, E>(
                    mut self,
                    executor: E,
                ) -> Result<<sqlx::Sqlite as sqlx::Database>::QueryResult, sqlx::Error>
                where
                    E: sqlx::Executor<'e, Database = sqlx::Sqlite>,
                {
                    self.inner.build().execute(executor).await
                }
            }
            impl sql_forge::SqlForgeQueryExecute for __SqlForgeQuery_single {
                type Db = sqlx::Sqlite;
                fn execute<'e, E>(
                    self,
                    executor: E,
                ) -> impl std::future::Future<
                    Output = Result<
                        <sqlx::Sqlite as sqlx::Database>::QueryResult,
                        sqlx::Error,
                    >,
                > + Send + 'e
                where
                    Self: Sized + 'e,
                    E: sqlx::Executor<'e, Database = sqlx::Sqlite> + Send + 'e,
                    sqlx::Sqlite: 'e,
                {
                    __SqlForgeQuery_single::execute(self, executor)
                }
            }
            let __sql_forge_runtime_category = "BatchFull";
            let mut __builder: sqlx::QueryBuilder<sqlx::Sqlite> = sqlx::QueryBuilder::new(
                "",
            );
            __builder.push("DELETE FROM products WHERE category = ");
            __builder.push_bind(__sql_forge_runtime_category);
            let __sql_forge_value_single = __SqlForgeQuery_single {
                inner: __builder,
            };
            __sql_forge_value_single
        }
            .execute(&pool)
            .await
            .expect("delete batch full failed");
    };
    let mut body = body;
    #[allow(unused_mut)]
    let mut body = unsafe { ::tokio::macros::support::Pin::new_unchecked(&mut body) };
    let body: ::core::pin::Pin<&mut dyn ::core::future::Future<Output = ()>> = body;
    #[allow(
        clippy::expect_used,
        clippy::diverging_sub_expression,
        clippy::needless_return,
        clippy::unwrap_in_result
    )]
    {
        use tokio::runtime::Builder;
        return Builder::new_current_thread()
            .enable_all()
            .build()
            .expect("Failed building the Runtime")
            .block_on(body);
    }
}
extern crate test;
#[rustc_test_marker = "section_match_bound_variable_no_warning"]
#[doc(hidden)]
pub const section_match_bound_variable_no_warning: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("section_match_bound_variable_no_warning"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "tests/tests.rs",
        start_line: 907usize,
        start_col: 10usize,
        end_line: 907usize,
        end_col: 49usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(section_match_bound_variable_no_warning()),
    ),
};
fn section_match_bound_variable_no_warning() {
    let body = async {
        let pool = pool().await;
        let max_price = Some(price_new(15000, 2));
        let products: Vec<Product> = {
            let _sql_forge_validator = || {
                {
                    type __SqlForgeModel = Product;
                    {
                        let __sql_forge_section_case_single_0_filter_price_max_price = &(match &(max_price) {
                            Some(max_price) => {
                                let _ = &max_price;
                                max_price
                            }
                            _ => {
                                ::core::panicking::panic_fmt(
                                    format_args!(
                                        "internal error: entered unreachable code: {0}",
                                        format_args!("sql_forge!: validator arm mismatch"),
                                    ),
                                );
                            }
                        });
                        let __sql_forge_validator_arg_0 = __sql_forge_section_case_single_0_filter_price_max_price;
                        let _ = {
                            {
                                #[allow(clippy::all)]
                                {
                                    use ::sqlx::Arguments as _;
                                    let arg0 = &(__sql_forge_validator_arg_0);
                                    let mut query_args = <sqlx::sqlite::Sqlite as ::sqlx::database::Database>::Arguments::default();
                                    query_args
                                        .reserve(
                                            1usize,
                                            0
                                                + ::sqlx::encode::Encode::<
                                                    sqlx::sqlite::Sqlite,
                                                >::size_hint(arg0),
                                        );
                                    let query_args = ::core::result::Result::<
                                        _,
                                        ::sqlx::error::BoxDynError,
                                    >::Ok(query_args)
                                        .and_then(move |mut query_args| {
                                            query_args.add(arg0).map(move |()| query_args)
                                        });
                                    ::sqlx::__query_with_result::<
                                        sqlx::sqlite::Sqlite,
                                        _,
                                    >(
                                            "SELECT id, name, price, stock, category FROM products WHERE 1=1  AND price <= ? ORDER BY id",
                                            query_args,
                                        )
                                        .try_map(|row: sqlx::sqlite::SqliteRow| {
                                            use ::sqlx::Row as _;
                                            #[allow(non_snake_case)]
                                            let sqlx_query_as_id = row
                                                .try_get_unchecked::<i64, _>(0usize)?
                                                .into();
                                            #[allow(non_snake_case)]
                                            let sqlx_query_as_name = row
                                                .try_get_unchecked::<String, _>(1usize)?
                                                .into();
                                            #[allow(non_snake_case)]
                                            let sqlx_query_as_price = row
                                                .try_get_unchecked::<i64, _>(2usize)?
                                                .into();
                                            #[allow(non_snake_case)]
                                            let sqlx_query_as_stock = row
                                                .try_get_unchecked::<i64, _>(3usize)?
                                                .into();
                                            #[allow(non_snake_case)]
                                            let sqlx_query_as_category = row
                                                .try_get_unchecked::<String, _>(4usize)?
                                                .into();
                                            ::std::result::Result::Ok(__SqlForgeModel {
                                                id: sqlx_query_as_id,
                                                name: sqlx_query_as_name,
                                                price: sqlx_query_as_price,
                                                stock: sqlx_query_as_stock,
                                                category: sqlx_query_as_category,
                                            })
                                        })
                                }
                            }
                        };
                    }
                    {
                        let _ = {
                            {
                                #[allow(clippy::all)]
                                {
                                    use ::sqlx::Arguments as _;
                                    let query_args = ::core::result::Result::<
                                        _,
                                        ::sqlx::error::BoxDynError,
                                    >::Ok(
                                        <sqlx::sqlite::Sqlite as ::sqlx::database::Database>::Arguments::default(),
                                    );
                                    ::sqlx::__query_with_result::<
                                        sqlx::sqlite::Sqlite,
                                        _,
                                    >(
                                            "SELECT id, name, price, stock, category FROM products WHERE 1=1  ORDER BY id",
                                            query_args,
                                        )
                                        .try_map(|row: sqlx::sqlite::SqliteRow| {
                                            use ::sqlx::Row as _;
                                            #[allow(non_snake_case)]
                                            let sqlx_query_as_id = row
                                                .try_get_unchecked::<i64, _>(0usize)?
                                                .into();
                                            #[allow(non_snake_case)]
                                            let sqlx_query_as_name = row
                                                .try_get_unchecked::<String, _>(1usize)?
                                                .into();
                                            #[allow(non_snake_case)]
                                            let sqlx_query_as_price = row
                                                .try_get_unchecked::<i64, _>(2usize)?
                                                .into();
                                            #[allow(non_snake_case)]
                                            let sqlx_query_as_stock = row
                                                .try_get_unchecked::<i64, _>(3usize)?
                                                .into();
                                            #[allow(non_snake_case)]
                                            let sqlx_query_as_category = row
                                                .try_get_unchecked::<String, _>(4usize)?
                                                .into();
                                            ::std::result::Result::Ok(__SqlForgeModel {
                                                id: sqlx_query_as_id,
                                                name: sqlx_query_as_name,
                                                price: sqlx_query_as_price,
                                                stock: sqlx_query_as_stock,
                                                category: sqlx_query_as_category,
                                            })
                                        })
                                }
                            }
                        };
                    }
                }
            };
            struct __SqlForgeQuery_single {
                inner: sqlx::QueryBuilder<sqlx::Sqlite>,
            }
            impl __SqlForgeQuery_single {
                async fn fetch_all<'e, E>(
                    mut self,
                    executor: E,
                ) -> Result<Vec<Product>, sqlx::Error>
                where
                    E: sqlx::Executor<'e, Database = sqlx::Sqlite>,
                {
                    self.inner.build_query_as::<Product>().fetch_all(executor).await
                }
                async fn fetch_one<'e, E>(
                    mut self,
                    executor: E,
                ) -> Result<Product, sqlx::Error>
                where
                    E: sqlx::Executor<'e, Database = sqlx::Sqlite>,
                {
                    self.inner.build_query_as::<Product>().fetch_one(executor).await
                }
                async fn fetch_optional<'e, E>(
                    mut self,
                    executor: E,
                ) -> Result<Option<Product>, sqlx::Error>
                where
                    E: sqlx::Executor<'e, Database = sqlx::Sqlite>,
                {
                    self.inner.build_query_as::<Product>().fetch_optional(executor).await
                }
                async fn execute<'e, E>(
                    mut self,
                    executor: E,
                ) -> Result<<sqlx::Sqlite as sqlx::Database>::QueryResult, sqlx::Error>
                where
                    E: sqlx::Executor<'e, Database = sqlx::Sqlite>,
                {
                    self.inner.build().execute(executor).await
                }
            }
            impl sql_forge::SqlForgeQuery<Product> for __SqlForgeQuery_single {
                type Db = sqlx::Sqlite;
                fn fetch_all<'e, E>(
                    self,
                    executor: E,
                ) -> impl std::future::Future<
                    Output = Result<Vec<Product>, sqlx::Error>,
                > + Send + 'e
                where
                    Self: Sized + 'e,
                    E: sqlx::Executor<'e, Database = sqlx::Sqlite> + Send + 'e,
                    sqlx::Sqlite: 'e,
                {
                    __SqlForgeQuery_single::fetch_all(self, executor)
                }
                fn fetch_one<'e, E>(
                    self,
                    executor: E,
                ) -> impl std::future::Future<
                    Output = Result<Product, sqlx::Error>,
                > + Send + 'e
                where
                    Self: Sized + 'e,
                    E: sqlx::Executor<'e, Database = sqlx::Sqlite> + Send + 'e,
                    sqlx::Sqlite: 'e,
                {
                    __SqlForgeQuery_single::fetch_one(self, executor)
                }
                fn fetch_optional<'e, E>(
                    self,
                    executor: E,
                ) -> impl std::future::Future<
                    Output = Result<Option<Product>, sqlx::Error>,
                > + Send + 'e
                where
                    Self: Sized + 'e,
                    E: sqlx::Executor<'e, Database = sqlx::Sqlite> + Send + 'e,
                    sqlx::Sqlite: 'e,
                {
                    __SqlForgeQuery_single::fetch_optional(self, executor)
                }
                fn execute<'e, E>(
                    self,
                    executor: E,
                ) -> impl std::future::Future<
                    Output = Result<
                        <sqlx::Sqlite as sqlx::Database>::QueryResult,
                        sqlx::Error,
                    >,
                > + Send + 'e
                where
                    Self: Sized + 'e,
                    E: sqlx::Executor<'e, Database = sqlx::Sqlite> + Send + 'e,
                    sqlx::Sqlite: 'e,
                {
                    __SqlForgeQuery_single::execute(self, executor)
                }
            }
            let mut __builder: sqlx::QueryBuilder<sqlx::Sqlite> = sqlx::QueryBuilder::new(
                "",
            );
            __builder
                .push(
                    "SELECT id, name, price, stock, category FROM products WHERE 1=1 ",
                );
            match max_price {
                Some(max_price) => {
                    ::core::hint::black_box(&max_price);
                    {
                        let __sql_forge_section_filter_price_0_max_price = max_price;
                        __builder.push(" AND price <= ");
                        __builder
                            .push_bind(__sql_forge_section_filter_price_0_max_price);
                    }
                }
                None => {}
            }
            __builder.push(" ORDER BY id");
            let __sql_forge_value_single = __SqlForgeQuery_single {
                inner: __builder,
            };
            __sql_forge_value_single
        }
            .fetch_all(&pool)
            .await
            .expect("section match pattern query failed");
        for p in &products {
            if !(p.price <= 15000) {
                ::core::panicking::panic("assertion failed: p.price <= 15000")
            }
        }
    };
    let mut body = body;
    #[allow(unused_mut)]
    let mut body = unsafe { ::tokio::macros::support::Pin::new_unchecked(&mut body) };
    let body: ::core::pin::Pin<&mut dyn ::core::future::Future<Output = ()>> = body;
    #[allow(
        clippy::expect_used,
        clippy::diverging_sub_expression,
        clippy::needless_return,
        clippy::unwrap_in_result
    )]
    {
        use tokio::runtime::Builder;
        return Builder::new_current_thread()
            .enable_all()
            .build()
            .expect("Failed building the Runtime")
            .block_on(body);
    }
}
extern crate test;
#[rustc_test_marker = "section_nested_match_outer_var_used"]
#[doc(hidden)]
pub const section_nested_match_outer_var_used: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("section_nested_match_outer_var_used"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "tests/tests.rs",
        start_line: 934usize,
        start_col: 10usize,
        end_line: 934usize,
        end_col: 45usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(section_nested_match_outer_var_used()),
    ),
};
fn section_nested_match_outer_var_used() {
    let body = async {
        let pool = pool().await;
        let limit_val = Some(3i64);
        let start_val = Some(0i64);
        let users: Vec<User> = {
            let _sql_forge_validator = || {
                {
                    type __SqlForgeModel = User;
                    {
                        let __sql_forge_section_case_single_0_limit_start = &(match &(limit_val) {
                            Some(limit) => {
                                let _ = &limit;
                                match &(start_val) {
                                    Some(start) => {
                                        let _ = &start;
                                        start
                                    }
                                    _ => {
                                        ::core::panicking::panic_fmt(
                                            format_args!(
                                                "internal error: entered unreachable code: {0}",
                                                format_args!("sql_forge!: validator arm mismatch"),
                                            ),
                                        );
                                    }
                                }
                            }
                            _ => {
                                ::core::panicking::panic_fmt(
                                    format_args!(
                                        "internal error: entered unreachable code: {0}",
                                        format_args!("sql_forge!: validator arm mismatch"),
                                    ),
                                );
                            }
                        });
                        let __sql_forge_section_case_single_0_limit_limit = &(match &(limit_val) {
                            Some(limit) => {
                                let _ = &limit;
                                match &(start_val) {
                                    Some(start) => {
                                        let _ = &start;
                                        limit
                                    }
                                    _ => {
                                        ::core::panicking::panic_fmt(
                                            format_args!(
                                                "internal error: entered unreachable code: {0}",
                                                format_args!("sql_forge!: validator arm mismatch"),
                                            ),
                                        );
                                    }
                                }
                            }
                            _ => {
                                ::core::panicking::panic_fmt(
                                    format_args!(
                                        "internal error: entered unreachable code: {0}",
                                        format_args!("sql_forge!: validator arm mismatch"),
                                    ),
                                );
                            }
                        });
                        let __sql_forge_validator_arg_0 = __sql_forge_section_case_single_0_limit_limit;
                        let __sql_forge_validator_arg_1 = __sql_forge_section_case_single_0_limit_start;
                        let _ = {
                            {
                                #[allow(clippy::all)]
                                {
                                    use ::sqlx::Arguments as _;
                                    let arg0 = &(__sql_forge_validator_arg_0);
                                    let arg1 = &(__sql_forge_validator_arg_1);
                                    let mut query_args = <sqlx::sqlite::Sqlite as ::sqlx::database::Database>::Arguments::default();
                                    query_args
                                        .reserve(
                                            2usize,
                                            0
                                                + ::sqlx::encode::Encode::<
                                                    sqlx::sqlite::Sqlite,
                                                >::size_hint(arg0)
                                                + ::sqlx::encode::Encode::<
                                                    sqlx::sqlite::Sqlite,
                                                >::size_hint(arg1),
                                        );
                                    let query_args = ::core::result::Result::<
                                        _,
                                        ::sqlx::error::BoxDynError,
                                    >::Ok(query_args)
                                        .and_then(move |mut query_args| {
                                            query_args.add(arg0).map(move |()| query_args)
                                        })
                                        .and_then(move |mut query_args| {
                                            query_args.add(arg1).map(move |()| query_args)
                                        });
                                    ::sqlx::__query_with_result::<
                                        sqlx::sqlite::Sqlite,
                                        _,
                                    >(
                                            "SELECT id, name FROM users WHERE 1=1 ORDER BY id  LIMIT ? OFFSET ? ",
                                            query_args,
                                        )
                                        .try_map(|row: sqlx::sqlite::SqliteRow| {
                                            use ::sqlx::Row as _;
                                            #[allow(non_snake_case)]
                                            let sqlx_query_as_id = row
                                                .try_get_unchecked::<i64, _>(0usize)?
                                                .into();
                                            #[allow(non_snake_case)]
                                            let sqlx_query_as_name = row
                                                .try_get_unchecked::<String, _>(1usize)?
                                                .into();
                                            ::std::result::Result::Ok(__SqlForgeModel {
                                                id: sqlx_query_as_id,
                                                name: sqlx_query_as_name,
                                            })
                                        })
                                }
                            }
                        };
                    }
                    {
                        let __sql_forge_section_case_single_1_limit_limit = &(match &(limit_val) {
                            Some(limit) => {
                                let _ = &limit;
                                match &(start_val) {
                                    None => limit,
                                    _ => {
                                        ::core::panicking::panic_fmt(
                                            format_args!(
                                                "internal error: entered unreachable code: {0}",
                                                format_args!("sql_forge!: validator arm mismatch"),
                                            ),
                                        );
                                    }
                                }
                            }
                            _ => {
                                ::core::panicking::panic_fmt(
                                    format_args!(
                                        "internal error: entered unreachable code: {0}",
                                        format_args!("sql_forge!: validator arm mismatch"),
                                    ),
                                );
                            }
                        });
                        let __sql_forge_validator_arg_0 = __sql_forge_section_case_single_1_limit_limit;
                        let _ = {
                            {
                                #[allow(clippy::all)]
                                {
                                    use ::sqlx::Arguments as _;
                                    let arg0 = &(__sql_forge_validator_arg_0);
                                    let mut query_args = <sqlx::sqlite::Sqlite as ::sqlx::database::Database>::Arguments::default();
                                    query_args
                                        .reserve(
                                            1usize,
                                            0
                                                + ::sqlx::encode::Encode::<
                                                    sqlx::sqlite::Sqlite,
                                                >::size_hint(arg0),
                                        );
                                    let query_args = ::core::result::Result::<
                                        _,
                                        ::sqlx::error::BoxDynError,
                                    >::Ok(query_args)
                                        .and_then(move |mut query_args| {
                                            query_args.add(arg0).map(move |()| query_args)
                                        });
                                    ::sqlx::__query_with_result::<
                                        sqlx::sqlite::Sqlite,
                                        _,
                                    >(
                                            "SELECT id, name FROM users WHERE 1=1 ORDER BY id  LIMIT ? ",
                                            query_args,
                                        )
                                        .try_map(|row: sqlx::sqlite::SqliteRow| {
                                            use ::sqlx::Row as _;
                                            #[allow(non_snake_case)]
                                            let sqlx_query_as_id = row
                                                .try_get_unchecked::<i64, _>(0usize)?
                                                .into();
                                            #[allow(non_snake_case)]
                                            let sqlx_query_as_name = row
                                                .try_get_unchecked::<String, _>(1usize)?
                                                .into();
                                            ::std::result::Result::Ok(__SqlForgeModel {
                                                id: sqlx_query_as_id,
                                                name: sqlx_query_as_name,
                                            })
                                        })
                                }
                            }
                        };
                    }
                    {
                        let _ = {
                            {
                                #[allow(clippy::all)]
                                {
                                    use ::sqlx::Arguments as _;
                                    let query_args = ::core::result::Result::<
                                        _,
                                        ::sqlx::error::BoxDynError,
                                    >::Ok(
                                        <sqlx::sqlite::Sqlite as ::sqlx::database::Database>::Arguments::default(),
                                    );
                                    ::sqlx::__query_with_result::<
                                        sqlx::sqlite::Sqlite,
                                        _,
                                    >(
                                            "SELECT id, name FROM users WHERE 1=1 ORDER BY id ",
                                            query_args,
                                        )
                                        .try_map(|row: sqlx::sqlite::SqliteRow| {
                                            use ::sqlx::Row as _;
                                            #[allow(non_snake_case)]
                                            let sqlx_query_as_id = row
                                                .try_get_unchecked::<i64, _>(0usize)?
                                                .into();
                                            #[allow(non_snake_case)]
                                            let sqlx_query_as_name = row
                                                .try_get_unchecked::<String, _>(1usize)?
                                                .into();
                                            ::std::result::Result::Ok(__SqlForgeModel {
                                                id: sqlx_query_as_id,
                                                name: sqlx_query_as_name,
                                            })
                                        })
                                }
                            }
                        };
                    }
                }
            };
            struct __SqlForgeQuery_single {
                inner: sqlx::QueryBuilder<sqlx::Sqlite>,
            }
            impl __SqlForgeQuery_single {
                async fn fetch_all<'e, E>(
                    mut self,
                    executor: E,
                ) -> Result<Vec<User>, sqlx::Error>
                where
                    E: sqlx::Executor<'e, Database = sqlx::Sqlite>,
                {
                    self.inner.build_query_as::<User>().fetch_all(executor).await
                }
                async fn fetch_one<'e, E>(
                    mut self,
                    executor: E,
                ) -> Result<User, sqlx::Error>
                where
                    E: sqlx::Executor<'e, Database = sqlx::Sqlite>,
                {
                    self.inner.build_query_as::<User>().fetch_one(executor).await
                }
                async fn fetch_optional<'e, E>(
                    mut self,
                    executor: E,
                ) -> Result<Option<User>, sqlx::Error>
                where
                    E: sqlx::Executor<'e, Database = sqlx::Sqlite>,
                {
                    self.inner.build_query_as::<User>().fetch_optional(executor).await
                }
                async fn execute<'e, E>(
                    mut self,
                    executor: E,
                ) -> Result<<sqlx::Sqlite as sqlx::Database>::QueryResult, sqlx::Error>
                where
                    E: sqlx::Executor<'e, Database = sqlx::Sqlite>,
                {
                    self.inner.build().execute(executor).await
                }
            }
            impl sql_forge::SqlForgeQuery<User> for __SqlForgeQuery_single {
                type Db = sqlx::Sqlite;
                fn fetch_all<'e, E>(
                    self,
                    executor: E,
                ) -> impl std::future::Future<
                    Output = Result<Vec<User>, sqlx::Error>,
                > + Send + 'e
                where
                    Self: Sized + 'e,
                    E: sqlx::Executor<'e, Database = sqlx::Sqlite> + Send + 'e,
                    sqlx::Sqlite: 'e,
                {
                    __SqlForgeQuery_single::fetch_all(self, executor)
                }
                fn fetch_one<'e, E>(
                    self,
                    executor: E,
                ) -> impl std::future::Future<
                    Output = Result<User, sqlx::Error>,
                > + Send + 'e
                where
                    Self: Sized + 'e,
                    E: sqlx::Executor<'e, Database = sqlx::Sqlite> + Send + 'e,
                    sqlx::Sqlite: 'e,
                {
                    __SqlForgeQuery_single::fetch_one(self, executor)
                }
                fn fetch_optional<'e, E>(
                    self,
                    executor: E,
                ) -> impl std::future::Future<
                    Output = Result<Option<User>, sqlx::Error>,
                > + Send + 'e
                where
                    Self: Sized + 'e,
                    E: sqlx::Executor<'e, Database = sqlx::Sqlite> + Send + 'e,
                    sqlx::Sqlite: 'e,
                {
                    __SqlForgeQuery_single::fetch_optional(self, executor)
                }
                fn execute<'e, E>(
                    self,
                    executor: E,
                ) -> impl std::future::Future<
                    Output = Result<
                        <sqlx::Sqlite as sqlx::Database>::QueryResult,
                        sqlx::Error,
                    >,
                > + Send + 'e
                where
                    Self: Sized + 'e,
                    E: sqlx::Executor<'e, Database = sqlx::Sqlite> + Send + 'e,
                    sqlx::Sqlite: 'e,
                {
                    __SqlForgeQuery_single::execute(self, executor)
                }
            }
            let mut __builder: sqlx::QueryBuilder<sqlx::Sqlite> = sqlx::QueryBuilder::new(
                "",
            );
            __builder.push("SELECT id, name FROM users WHERE 1=1 ORDER BY id ");
            match limit_val {
                Some(limit) => {
                    ::core::hint::black_box(&limit);
                    match start_val {
                        Some(start) => {
                            ::core::hint::black_box(&start);
                            {
                                let __sql_forge_section_limit_0_0_start = start;
                                let __sql_forge_section_limit_0_0_limit = limit;
                                __builder.push(" LIMIT ");
                                __builder.push_bind(__sql_forge_section_limit_0_0_limit);
                                __builder.push(" OFFSET ");
                                __builder.push_bind(__sql_forge_section_limit_0_0_start);
                                __builder.push(" ");
                            }
                        }
                        None => {
                            let __sql_forge_section_limit_0_1_limit = limit;
                            __builder.push(" LIMIT ");
                            __builder.push_bind(__sql_forge_section_limit_0_1_limit);
                            __builder.push(" ");
                        }
                    }
                }
                None => {}
            }
            let __sql_forge_value_single = __SqlForgeQuery_single {
                inner: __builder,
            };
            __sql_forge_value_single
        }
            .fetch_all(&pool)
            .await
            .expect("nested match query failed");
        if !!users.is_empty() {
            ::core::panicking::panic("assertion failed: !users.is_empty()")
        }
        if !(users.len() <= 3) {
            ::core::panicking::panic("assertion failed: users.len() <= 3")
        }
        for (i, user) in users.iter().enumerate() {
            if !(user.id >= i as i64) {
                ::core::panicking::panic("assertion failed: user.id >= i as i64")
            }
        }
    };
    let mut body = body;
    #[allow(unused_mut)]
    let mut body = unsafe { ::tokio::macros::support::Pin::new_unchecked(&mut body) };
    let body: ::core::pin::Pin<&mut dyn ::core::future::Future<Output = ()>> = body;
    #[allow(
        clippy::expect_used,
        clippy::diverging_sub_expression,
        clippy::needless_return,
        clippy::unwrap_in_result
    )]
    {
        use tokio::runtime::Builder;
        return Builder::new_current_thread()
            .enable_all()
            .build()
            .expect("Failed building the Runtime")
            .block_on(body);
    }
}
extern crate test;
#[rustc_test_marker = "execute_only_with_explicit_sqlite_db"]
#[doc(hidden)]
pub const execute_only_with_explicit_sqlite_db: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("execute_only_with_explicit_sqlite_db"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "tests/tests.rs",
        start_line: 994usize,
        start_col: 10usize,
        end_line: 994usize,
        end_col: 46usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(execute_only_with_explicit_sqlite_db()),
    ),
};
fn execute_only_with_explicit_sqlite_db() {
    let body = async {
        let pool = pool().await;
        {
            let _sql_forge_validator = || {
                {
                    {
                        let _ = {
                            #[allow(clippy::all)]
                            {
                                use ::sqlx::Arguments as _;
                                let query_args = ::core::result::Result::<
                                    _,
                                    ::sqlx::error::BoxDynError,
                                >::Ok(
                                    <sqlx::sqlite::Sqlite as ::sqlx::database::Database>::Arguments::default(),
                                );
                                ::sqlx::__query_scalar_with_result::<
                                    sqlx::sqlite::Sqlite,
                                    i64,
                                    _,
                                >("SELECT 1", query_args)
                            }
                        };
                    }
                }
            };
            struct __SqlForgeQuery_single {
                inner: sqlx::QueryBuilder<sqlx::Sqlite>,
            }
            impl __SqlForgeQuery_single {
                async fn execute<'e, E>(
                    mut self,
                    executor: E,
                ) -> Result<<sqlx::Sqlite as sqlx::Database>::QueryResult, sqlx::Error>
                where
                    E: sqlx::Executor<'e, Database = sqlx::Sqlite>,
                {
                    self.inner.build().execute(executor).await
                }
            }
            impl sql_forge::SqlForgeQueryExecute for __SqlForgeQuery_single {
                type Db = sqlx::Sqlite;
                fn execute<'e, E>(
                    self,
                    executor: E,
                ) -> impl std::future::Future<
                    Output = Result<
                        <sqlx::Sqlite as sqlx::Database>::QueryResult,
                        sqlx::Error,
                    >,
                > + Send + 'e
                where
                    Self: Sized + 'e,
                    E: sqlx::Executor<'e, Database = sqlx::Sqlite> + Send + 'e,
                    sqlx::Sqlite: 'e,
                {
                    __SqlForgeQuery_single::execute(self, executor)
                }
            }
            let mut __builder: sqlx::QueryBuilder<sqlx::Sqlite> = sqlx::QueryBuilder::new(
                "",
            );
            __builder.push("SELECT 1");
            let __sql_forge_value_single = __SqlForgeQuery_single {
                inner: __builder,
            };
            __sql_forge_value_single
        }
            .execute(&pool)
            .await
            .expect("execute-only with explicit Sqlite db failed");
    };
    let mut body = body;
    #[allow(unused_mut)]
    let mut body = unsafe { ::tokio::macros::support::Pin::new_unchecked(&mut body) };
    let body: ::core::pin::Pin<&mut dyn ::core::future::Future<Output = ()>> = body;
    #[allow(
        clippy::expect_used,
        clippy::diverging_sub_expression,
        clippy::needless_return,
        clippy::unwrap_in_result
    )]
    {
        use tokio::runtime::Builder;
        return Builder::new_current_thread()
            .enable_all()
            .build()
            .expect("Failed building the Runtime")
            .block_on(body);
    }
}
extern crate test;
#[rustc_test_marker = "compile_fail"]
#[doc(hidden)]
pub const compile_fail: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("compile_fail"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "tests/tests.rs",
        start_line: 1004usize,
        start_col: 4usize,
        end_line: 1004usize,
        end_col: 16usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(compile_fail()),
    ),
};
fn compile_fail() {
    let db_type = std::env::var("ENV_DB_TYPE").expect("ENV_DB_TYPE not defined");
    let pattern = ::alloc::__export::must_use({
        ::alloc::fmt::format(format_args!("tests/{0}/tmp-ui/*.rs", db_type))
    });
    let tests = trybuild::TestCases::new();
    tests.compile_fail(&pattern);
}
extern crate test;
#[rustc_test_marker = "compile_fail_specific"]
#[doc(hidden)]
pub const compile_fail_specific: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("compile_fail_specific"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "tests/tests.rs",
        start_line: 1012usize,
        start_col: 4usize,
        end_line: 1012usize,
        end_col: 25usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(compile_fail_specific()),
    ),
};
fn compile_fail_specific() {
    let db_type = std::env::var("ENV_DB_TYPE").expect("ENV_DB_TYPE not defined");
    let pattern = ::alloc::__export::must_use({
        ::alloc::fmt::format(format_args!("tests/{0}/ui/*.rs", db_type))
    });
    let tests = trybuild::TestCases::new();
    tests.compile_fail(&pattern);
}
#[rustc_main]
#[coverage(off)]
#[doc(hidden)]
pub fn main() -> () {
    extern crate test;
    test::test_main_static(
        &[
            &basic_query_with_inline_params,
            &combining_features_example,
            &compile_fail,
            &compile_fail_specific,
            &db_type_matches_env_db_type,
            &execute_batch,
            &execute_batch_full,
            &execute_batch_with_params,
            &execute_only_insert_update_delete,
            &execute_only_query,
            &execute_only_with_explicit_sqlite_db,
            &grouped_sections,
            &grouped_sections_with_nested_matches,
            &list_parameter_in_clause,
            &list_parameter_in_main_sql_with_match_filter,
            &list_parameter_with_empty_guard,
            &multiple_results_group,
            &multiple_results_scalar_key,
            &scalar_output,
            &section_dynamic_where,
            &section_match_bound_variable_no_warning,
            &section_nested_match_outer_var_used,
            &section_with_local_params,
            &struct_source_params,
        ],
    )
}
