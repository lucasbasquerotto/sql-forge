#![feature(prelude_import)]
extern crate std;
#[prelude_import]
use std::prelude::rust_2021::*;
use sql_forge::db_type;
use sql_forge::sql_forge;
pub type AppDb = sqlx::MySql;
pub type DbPool = sqlx::Pool<AppDb>;
type Price = i64;
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
        start_line: 61usize,
        start_col: 10usize,
        end_line: 61usize,
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
                let __enhanced_top_level_max_id = &(3i64);
                let __enhanced_top_level_limit = &(10i64);
                {
                    type __EnhancedModel = User;
                    {
                        let _ = {
                            {
                                #[allow(clippy::all)]
                                {
                                    use ::sqlx::Arguments as _;
                                    let arg0 = &(__enhanced_top_level_max_id);
                                    let arg1 = &(__enhanced_top_level_max_id);
                                    let arg2 = &(__enhanced_top_level_limit);
                                    let mut query_args = <sqlx::mysql::MySql as ::sqlx::database::Database>::Arguments::<
                                        '_,
                                    >::default();
                                    query_args
                                        .reserve(
                                            3usize,
                                            0
                                                + ::sqlx::encode::Encode::<
                                                    sqlx::mysql::MySql,
                                                >::size_hint(arg0)
                                                + ::sqlx::encode::Encode::<
                                                    sqlx::mysql::MySql,
                                                >::size_hint(arg1)
                                                + ::sqlx::encode::Encode::<
                                                    sqlx::mysql::MySql,
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
                                        sqlx::mysql::MySql,
                                        _,
                                    >(
                                            "SELECT id, name FROM users WHERE id <= ? AND ? >= id LIMIT ?",
                                            query_args,
                                        )
                                        .try_map(|row: sqlx::mysql::MySqlRow| {
                                            use ::sqlx::Row as _;
                                            #[allow(non_snake_case)]
                                            let sqlx_query_as_id = row
                                                .try_get_unchecked::<i64, _>(0usize)?
                                                .into();
                                            #[allow(non_snake_case)]
                                            let sqlx_query_as_name = row
                                                .try_get_unchecked::<String, _>(1usize)?
                                                .into();
                                            ::std::result::Result::Ok(__EnhancedModel {
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
            struct __EnhancedQuery_single<'args> {
                inner: sqlx::QueryBuilder<'args, sqlx::MySql>,
            }
            impl<'args> __EnhancedQuery_single<'args> {
                async fn fetch_all<'e, E>(
                    mut self,
                    executor: E,
                ) -> Result<Vec<User>, sqlx::Error>
                where
                    E: sqlx::Executor<'e, Database = sqlx::MySql>,
                {
                    self.inner.build_query_as::<User>().fetch_all(executor).await
                }
                async fn fetch_one<'e, E>(
                    mut self,
                    executor: E,
                ) -> Result<User, sqlx::Error>
                where
                    E: sqlx::Executor<'e, Database = sqlx::MySql>,
                {
                    self.inner.build_query_as::<User>().fetch_one(executor).await
                }
                async fn fetch_optional<'e, E>(
                    mut self,
                    executor: E,
                ) -> Result<Option<User>, sqlx::Error>
                where
                    E: sqlx::Executor<'e, Database = sqlx::MySql>,
                {
                    self.inner.build_query_as::<User>().fetch_optional(executor).await
                }
                async fn execute<'e, E>(
                    mut self,
                    executor: E,
                ) -> Result<<sqlx::MySql as sqlx::Database>::QueryResult, sqlx::Error>
                where
                    E: sqlx::Executor<'e, Database = sqlx::MySql>,
                {
                    self.inner.build().execute(executor).await
                }
            }
            impl<'args> sql_forge::EnhancedQuery<User>
            for __EnhancedQuery_single<'args> {
                type Db = sqlx::MySql;
                fn fetch_all<'e, E>(
                    self,
                    executor: E,
                ) -> impl std::future::Future<
                    Output = Result<Vec<User>, sqlx::Error>,
                > + Send + 'e
                where
                    Self: Sized + 'e,
                    E: sqlx::Executor<'e, Database = sqlx::MySql> + Send + 'e,
                    sqlx::MySql: 'e,
                {
                    __EnhancedQuery_single::fetch_all(self, executor)
                }
                fn fetch_one<'e, E>(
                    self,
                    executor: E,
                ) -> impl std::future::Future<
                    Output = Result<User, sqlx::Error>,
                > + Send + 'e
                where
                    Self: Sized + 'e,
                    E: sqlx::Executor<'e, Database = sqlx::MySql> + Send + 'e,
                    sqlx::MySql: 'e,
                {
                    __EnhancedQuery_single::fetch_one(self, executor)
                }
                fn fetch_optional<'e, E>(
                    self,
                    executor: E,
                ) -> impl std::future::Future<
                    Output = Result<Option<User>, sqlx::Error>,
                > + Send + 'e
                where
                    Self: Sized + 'e,
                    E: sqlx::Executor<'e, Database = sqlx::MySql> + Send + 'e,
                    sqlx::MySql: 'e,
                {
                    __EnhancedQuery_single::fetch_optional(self, executor)
                }
                fn execute<'e, E>(
                    self,
                    executor: E,
                ) -> impl std::future::Future<
                    Output = Result<
                        <sqlx::MySql as sqlx::Database>::QueryResult,
                        sqlx::Error,
                    >,
                > + Send + 'e
                where
                    Self: Sized + 'e,
                    E: sqlx::Executor<'e, Database = sqlx::MySql> + Send + 'e,
                    sqlx::MySql: 'e,
                {
                    __EnhancedQuery_single::execute(self, executor)
                }
            }
            let __enhanced_runtime_max_id = 3i64;
            let __enhanced_runtime_limit = 10i64;
            let mut __builder: sqlx::QueryBuilder<sqlx::MySql> = sqlx::QueryBuilder::new(
                "",
            );
            __builder.push("SELECT id, name FROM users WHERE id <= ");
            __builder.push_bind(__enhanced_runtime_max_id);
            __builder.push(" AND ");
            __builder.push_bind(__enhanced_runtime_max_id);
            __builder.push(" >= id LIMIT ");
            __builder.push_bind(__enhanced_runtime_limit);
            let __sql_forge_value_single = __EnhancedQuery_single {
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
        start_line: 80usize,
        start_col: 10usize,
        end_line: 80usize,
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
                let __enhanced_top_level_min_id = &(2i64);
                {
                    {
                        let _ = {
                            #[allow(clippy::all)]
                            {
                                use ::sqlx::Arguments as _;
                                let arg0 = &(__enhanced_top_level_min_id);
                                let mut query_args = <sqlx::mysql::MySql as ::sqlx::database::Database>::Arguments::<
                                    '_,
                                >::default();
                                query_args
                                    .reserve(
                                        1usize,
                                        0
                                            + ::sqlx::encode::Encode::<
                                                sqlx::mysql::MySql,
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
                                    sqlx::mysql::MySql,
                                    i64,
                                    _,
                                >("SELECT COUNT(*) FROM users WHERE id > ?", query_args)
                            }
                        };
                    }
                }
            };
            struct __EnhancedQuery_single<'args> {
                inner: sqlx::QueryBuilder<'args, sqlx::MySql>,
            }
            impl<'args> __EnhancedQuery_single<'args> {
                async fn fetch_all<'e, E>(
                    mut self,
                    executor: E,
                ) -> Result<Vec<i64>, sqlx::Error>
                where
                    E: sqlx::Executor<'e, Database = sqlx::MySql>,
                {
                    self.inner.build_query_scalar::<i64>().fetch_all(executor).await
                }
                async fn fetch_one<'e, E>(
                    mut self,
                    executor: E,
                ) -> Result<i64, sqlx::Error>
                where
                    E: sqlx::Executor<'e, Database = sqlx::MySql>,
                {
                    self.inner.build_query_scalar::<i64>().fetch_one(executor).await
                }
                async fn fetch_optional<'e, E>(
                    mut self,
                    executor: E,
                ) -> Result<Option<i64>, sqlx::Error>
                where
                    E: sqlx::Executor<'e, Database = sqlx::MySql>,
                {
                    self.inner.build_query_scalar::<i64>().fetch_optional(executor).await
                }
                async fn execute<'e, E>(
                    mut self,
                    executor: E,
                ) -> Result<<sqlx::MySql as sqlx::Database>::QueryResult, sqlx::Error>
                where
                    E: sqlx::Executor<'e, Database = sqlx::MySql>,
                {
                    self.inner.build().execute(executor).await
                }
            }
            impl<'args> sql_forge::EnhancedQuery<i64> for __EnhancedQuery_single<'args> {
                type Db = sqlx::MySql;
                fn fetch_all<'e, E>(
                    self,
                    executor: E,
                ) -> impl std::future::Future<
                    Output = Result<Vec<i64>, sqlx::Error>,
                > + Send + 'e
                where
                    Self: Sized + 'e,
                    E: sqlx::Executor<'e, Database = sqlx::MySql> + Send + 'e,
                    sqlx::MySql: 'e,
                {
                    __EnhancedQuery_single::fetch_all(self, executor)
                }
                fn fetch_one<'e, E>(
                    self,
                    executor: E,
                ) -> impl std::future::Future<
                    Output = Result<i64, sqlx::Error>,
                > + Send + 'e
                where
                    Self: Sized + 'e,
                    E: sqlx::Executor<'e, Database = sqlx::MySql> + Send + 'e,
                    sqlx::MySql: 'e,
                {
                    __EnhancedQuery_single::fetch_one(self, executor)
                }
                fn fetch_optional<'e, E>(
                    self,
                    executor: E,
                ) -> impl std::future::Future<
                    Output = Result<Option<i64>, sqlx::Error>,
                > + Send + 'e
                where
                    Self: Sized + 'e,
                    E: sqlx::Executor<'e, Database = sqlx::MySql> + Send + 'e,
                    sqlx::MySql: 'e,
                {
                    __EnhancedQuery_single::fetch_optional(self, executor)
                }
                fn execute<'e, E>(
                    self,
                    executor: E,
                ) -> impl std::future::Future<
                    Output = Result<
                        <sqlx::MySql as sqlx::Database>::QueryResult,
                        sqlx::Error,
                    >,
                > + Send + 'e
                where
                    Self: Sized + 'e,
                    E: sqlx::Executor<'e, Database = sqlx::MySql> + Send + 'e,
                    sqlx::MySql: 'e,
                {
                    __EnhancedQuery_single::execute(self, executor)
                }
            }
            let __enhanced_runtime_min_id = 2i64;
            let mut __builder: sqlx::QueryBuilder<sqlx::MySql> = sqlx::QueryBuilder::new(
                "",
            );
            __builder.push("SELECT COUNT(*) FROM users WHERE id > ");
            __builder.push_bind(__enhanced_runtime_min_id);
            let __sql_forge_value_single = __EnhancedQuery_single {
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
        start_line: 96usize,
        start_col: 10usize,
        end_line: 96usize,
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
                let __enhanced_source_top_level = &(filter);
                let __enhanced_top_level_max_id = __enhanced_source_top_level.max_id;
                let __enhanced_top_level_limit = __enhanced_source_top_level.limit;
                {
                    type __EnhancedModel = User;
                    {
                        let _ = {
                            {
                                #[allow(clippy::all)]
                                {
                                    use ::sqlx::Arguments as _;
                                    let arg0 = &(__enhanced_top_level_max_id);
                                    let arg1 = &(__enhanced_top_level_limit);
                                    let mut query_args = <sqlx::mysql::MySql as ::sqlx::database::Database>::Arguments::<
                                        '_,
                                    >::default();
                                    query_args
                                        .reserve(
                                            2usize,
                                            0
                                                + ::sqlx::encode::Encode::<
                                                    sqlx::mysql::MySql,
                                                >::size_hint(arg0)
                                                + ::sqlx::encode::Encode::<
                                                    sqlx::mysql::MySql,
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
                                        sqlx::mysql::MySql,
                                        _,
                                    >(
                                            "SELECT id, name FROM users WHERE id <= ? LIMIT ?",
                                            query_args,
                                        )
                                        .try_map(|row: sqlx::mysql::MySqlRow| {
                                            use ::sqlx::Row as _;
                                            #[allow(non_snake_case)]
                                            let sqlx_query_as_id = row
                                                .try_get_unchecked::<i64, _>(0usize)?
                                                .into();
                                            #[allow(non_snake_case)]
                                            let sqlx_query_as_name = row
                                                .try_get_unchecked::<String, _>(1usize)?
                                                .into();
                                            ::std::result::Result::Ok(__EnhancedModel {
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
            struct __EnhancedQuery_single<'args> {
                inner: sqlx::QueryBuilder<'args, sqlx::MySql>,
            }
            impl<'args> __EnhancedQuery_single<'args> {
                async fn fetch_all<'e, E>(
                    mut self,
                    executor: E,
                ) -> Result<Vec<User>, sqlx::Error>
                where
                    E: sqlx::Executor<'e, Database = sqlx::MySql>,
                {
                    self.inner.build_query_as::<User>().fetch_all(executor).await
                }
                async fn fetch_one<'e, E>(
                    mut self,
                    executor: E,
                ) -> Result<User, sqlx::Error>
                where
                    E: sqlx::Executor<'e, Database = sqlx::MySql>,
                {
                    self.inner.build_query_as::<User>().fetch_one(executor).await
                }
                async fn fetch_optional<'e, E>(
                    mut self,
                    executor: E,
                ) -> Result<Option<User>, sqlx::Error>
                where
                    E: sqlx::Executor<'e, Database = sqlx::MySql>,
                {
                    self.inner.build_query_as::<User>().fetch_optional(executor).await
                }
                async fn execute<'e, E>(
                    mut self,
                    executor: E,
                ) -> Result<<sqlx::MySql as sqlx::Database>::QueryResult, sqlx::Error>
                where
                    E: sqlx::Executor<'e, Database = sqlx::MySql>,
                {
                    self.inner.build().execute(executor).await
                }
            }
            impl<'args> sql_forge::EnhancedQuery<User>
            for __EnhancedQuery_single<'args> {
                type Db = sqlx::MySql;
                fn fetch_all<'e, E>(
                    self,
                    executor: E,
                ) -> impl std::future::Future<
                    Output = Result<Vec<User>, sqlx::Error>,
                > + Send + 'e
                where
                    Self: Sized + 'e,
                    E: sqlx::Executor<'e, Database = sqlx::MySql> + Send + 'e,
                    sqlx::MySql: 'e,
                {
                    __EnhancedQuery_single::fetch_all(self, executor)
                }
                fn fetch_one<'e, E>(
                    self,
                    executor: E,
                ) -> impl std::future::Future<
                    Output = Result<User, sqlx::Error>,
                > + Send + 'e
                where
                    Self: Sized + 'e,
                    E: sqlx::Executor<'e, Database = sqlx::MySql> + Send + 'e,
                    sqlx::MySql: 'e,
                {
                    __EnhancedQuery_single::fetch_one(self, executor)
                }
                fn fetch_optional<'e, E>(
                    self,
                    executor: E,
                ) -> impl std::future::Future<
                    Output = Result<Option<User>, sqlx::Error>,
                > + Send + 'e
                where
                    Self: Sized + 'e,
                    E: sqlx::Executor<'e, Database = sqlx::MySql> + Send + 'e,
                    sqlx::MySql: 'e,
                {
                    __EnhancedQuery_single::fetch_optional(self, executor)
                }
                fn execute<'e, E>(
                    self,
                    executor: E,
                ) -> impl std::future::Future<
                    Output = Result<
                        <sqlx::MySql as sqlx::Database>::QueryResult,
                        sqlx::Error,
                    >,
                > + Send + 'e
                where
                    Self: Sized + 'e,
                    E: sqlx::Executor<'e, Database = sqlx::MySql> + Send + 'e,
                    sqlx::MySql: 'e,
                {
                    __EnhancedQuery_single::execute(self, executor)
                }
            }
            let __enhanced_source_runtime = &(filter);
            let __enhanced_runtime_max_id = __enhanced_source_runtime.max_id;
            let __enhanced_runtime_limit = __enhanced_source_runtime.limit;
            let mut __builder: sqlx::QueryBuilder<sqlx::MySql> = sqlx::QueryBuilder::new(
                "",
            );
            __builder.push("SELECT id, name FROM users WHERE id <= ");
            __builder.push_bind(__enhanced_runtime_max_id);
            __builder.push(" LIMIT ");
            __builder.push_bind(__enhanced_runtime_limit);
            let __sql_forge_value_single = __EnhancedQuery_single {
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
        start_line: 117usize,
        start_col: 10usize,
        end_line: 117usize,
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
                    type __EnhancedModel = Product;
                    {
                        let __enhanced_section_case_single_0_filter_category_cat = &(cat);
                        let _ = {
                            {
                                #[allow(clippy::all)]
                                {
                                    use ::sqlx::Arguments as _;
                                    let arg0 = &(__enhanced_section_case_single_0_filter_category_cat);
                                    let mut query_args = <sqlx::mysql::MySql as ::sqlx::database::Database>::Arguments::<
                                        '_,
                                    >::default();
                                    query_args
                                        .reserve(
                                            1usize,
                                            0
                                                + ::sqlx::encode::Encode::<
                                                    sqlx::mysql::MySql,
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
                                        sqlx::mysql::MySql,
                                        _,
                                    >(
                                            "\n        SELECT id, name, price, stock, category\n        FROM products\n        WHERE 1 = 1\n         AND category = ? \n        ",
                                            query_args,
                                        )
                                        .try_map(|row: sqlx::mysql::MySqlRow| {
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
                                                .try_get_unchecked::<i32, _>(3usize)?
                                                .into();
                                            #[allow(non_snake_case)]
                                            let sqlx_query_as_category = row
                                                .try_get_unchecked::<String, _>(4usize)?
                                                .into();
                                            ::std::result::Result::Ok(__EnhancedModel {
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
            struct __EnhancedQuery_single<'args> {
                inner: sqlx::QueryBuilder<'args, sqlx::MySql>,
            }
            impl<'args> __EnhancedQuery_single<'args> {
                async fn fetch_all<'e, E>(
                    mut self,
                    executor: E,
                ) -> Result<Vec<Product>, sqlx::Error>
                where
                    E: sqlx::Executor<'e, Database = sqlx::MySql>,
                {
                    self.inner.build_query_as::<Product>().fetch_all(executor).await
                }
                async fn fetch_one<'e, E>(
                    mut self,
                    executor: E,
                ) -> Result<Product, sqlx::Error>
                where
                    E: sqlx::Executor<'e, Database = sqlx::MySql>,
                {
                    self.inner.build_query_as::<Product>().fetch_one(executor).await
                }
                async fn fetch_optional<'e, E>(
                    mut self,
                    executor: E,
                ) -> Result<Option<Product>, sqlx::Error>
                where
                    E: sqlx::Executor<'e, Database = sqlx::MySql>,
                {
                    self.inner.build_query_as::<Product>().fetch_optional(executor).await
                }
                async fn execute<'e, E>(
                    mut self,
                    executor: E,
                ) -> Result<<sqlx::MySql as sqlx::Database>::QueryResult, sqlx::Error>
                where
                    E: sqlx::Executor<'e, Database = sqlx::MySql>,
                {
                    self.inner.build().execute(executor).await
                }
            }
            impl<'args> sql_forge::EnhancedQuery<Product>
            for __EnhancedQuery_single<'args> {
                type Db = sqlx::MySql;
                fn fetch_all<'e, E>(
                    self,
                    executor: E,
                ) -> impl std::future::Future<
                    Output = Result<Vec<Product>, sqlx::Error>,
                > + Send + 'e
                where
                    Self: Sized + 'e,
                    E: sqlx::Executor<'e, Database = sqlx::MySql> + Send + 'e,
                    sqlx::MySql: 'e,
                {
                    __EnhancedQuery_single::fetch_all(self, executor)
                }
                fn fetch_one<'e, E>(
                    self,
                    executor: E,
                ) -> impl std::future::Future<
                    Output = Result<Product, sqlx::Error>,
                > + Send + 'e
                where
                    Self: Sized + 'e,
                    E: sqlx::Executor<'e, Database = sqlx::MySql> + Send + 'e,
                    sqlx::MySql: 'e,
                {
                    __EnhancedQuery_single::fetch_one(self, executor)
                }
                fn fetch_optional<'e, E>(
                    self,
                    executor: E,
                ) -> impl std::future::Future<
                    Output = Result<Option<Product>, sqlx::Error>,
                > + Send + 'e
                where
                    Self: Sized + 'e,
                    E: sqlx::Executor<'e, Database = sqlx::MySql> + Send + 'e,
                    sqlx::MySql: 'e,
                {
                    __EnhancedQuery_single::fetch_optional(self, executor)
                }
                fn execute<'e, E>(
                    self,
                    executor: E,
                ) -> impl std::future::Future<
                    Output = Result<
                        <sqlx::MySql as sqlx::Database>::QueryResult,
                        sqlx::Error,
                    >,
                > + Send + 'e
                where
                    Self: Sized + 'e,
                    E: sqlx::Executor<'e, Database = sqlx::MySql> + Send + 'e,
                    sqlx::MySql: 'e,
                {
                    __EnhancedQuery_single::execute(self, executor)
                }
            }
            let mut __builder: sqlx::QueryBuilder<sqlx::MySql> = sqlx::QueryBuilder::new(
                "",
            );
            __builder
                .push(
                    "\n        SELECT id, name, price, stock, category\n        FROM products\n        WHERE 1 = 1\n        ",
                );
            {
                let __enhanced_section_filter_category_cat = cat;
                __builder.push(" AND category = ");
                __builder.push_bind(__enhanced_section_filter_category_cat);
                __builder.push(" ");
            }
            __builder.push("\n        ");
            let __sql_forge_value_single = __EnhancedQuery_single {
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
        start_line: 148usize,
        start_col: 10usize,
        end_line: 148usize,
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
                    type __EnhancedModel = User;
                    {
                        let __enhanced_section_case_single_0_filter_max_id = &(max_id);
                        let _ = {
                            {
                                #[allow(clippy::all)]
                                {
                                    use ::sqlx::Arguments as _;
                                    let arg0 = &(__enhanced_section_case_single_0_filter_max_id);
                                    let mut query_args = <sqlx::mysql::MySql as ::sqlx::database::Database>::Arguments::<
                                        '_,
                                    >::default();
                                    query_args
                                        .reserve(
                                            1usize,
                                            0
                                                + ::sqlx::encode::Encode::<
                                                    sqlx::mysql::MySql,
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
                                        sqlx::mysql::MySql,
                                        _,
                                    >("SELECT id, name FROM users  WHERE id <= ? ", query_args)
                                        .try_map(|row: sqlx::mysql::MySqlRow| {
                                            use ::sqlx::Row as _;
                                            #[allow(non_snake_case)]
                                            let sqlx_query_as_id = row
                                                .try_get_unchecked::<i64, _>(0usize)?
                                                .into();
                                            #[allow(non_snake_case)]
                                            let sqlx_query_as_name = row
                                                .try_get_unchecked::<String, _>(1usize)?
                                                .into();
                                            ::std::result::Result::Ok(__EnhancedModel {
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
            struct __EnhancedQuery_single<'args> {
                inner: sqlx::QueryBuilder<'args, sqlx::MySql>,
            }
            impl<'args> __EnhancedQuery_single<'args> {
                async fn fetch_all<'e, E>(
                    mut self,
                    executor: E,
                ) -> Result<Vec<User>, sqlx::Error>
                where
                    E: sqlx::Executor<'e, Database = sqlx::MySql>,
                {
                    self.inner.build_query_as::<User>().fetch_all(executor).await
                }
                async fn fetch_one<'e, E>(
                    mut self,
                    executor: E,
                ) -> Result<User, sqlx::Error>
                where
                    E: sqlx::Executor<'e, Database = sqlx::MySql>,
                {
                    self.inner.build_query_as::<User>().fetch_one(executor).await
                }
                async fn fetch_optional<'e, E>(
                    mut self,
                    executor: E,
                ) -> Result<Option<User>, sqlx::Error>
                where
                    E: sqlx::Executor<'e, Database = sqlx::MySql>,
                {
                    self.inner.build_query_as::<User>().fetch_optional(executor).await
                }
                async fn execute<'e, E>(
                    mut self,
                    executor: E,
                ) -> Result<<sqlx::MySql as sqlx::Database>::QueryResult, sqlx::Error>
                where
                    E: sqlx::Executor<'e, Database = sqlx::MySql>,
                {
                    self.inner.build().execute(executor).await
                }
            }
            impl<'args> sql_forge::EnhancedQuery<User>
            for __EnhancedQuery_single<'args> {
                type Db = sqlx::MySql;
                fn fetch_all<'e, E>(
                    self,
                    executor: E,
                ) -> impl std::future::Future<
                    Output = Result<Vec<User>, sqlx::Error>,
                > + Send + 'e
                where
                    Self: Sized + 'e,
                    E: sqlx::Executor<'e, Database = sqlx::MySql> + Send + 'e,
                    sqlx::MySql: 'e,
                {
                    __EnhancedQuery_single::fetch_all(self, executor)
                }
                fn fetch_one<'e, E>(
                    self,
                    executor: E,
                ) -> impl std::future::Future<
                    Output = Result<User, sqlx::Error>,
                > + Send + 'e
                where
                    Self: Sized + 'e,
                    E: sqlx::Executor<'e, Database = sqlx::MySql> + Send + 'e,
                    sqlx::MySql: 'e,
                {
                    __EnhancedQuery_single::fetch_one(self, executor)
                }
                fn fetch_optional<'e, E>(
                    self,
                    executor: E,
                ) -> impl std::future::Future<
                    Output = Result<Option<User>, sqlx::Error>,
                > + Send + 'e
                where
                    Self: Sized + 'e,
                    E: sqlx::Executor<'e, Database = sqlx::MySql> + Send + 'e,
                    sqlx::MySql: 'e,
                {
                    __EnhancedQuery_single::fetch_optional(self, executor)
                }
                fn execute<'e, E>(
                    self,
                    executor: E,
                ) -> impl std::future::Future<
                    Output = Result<
                        <sqlx::MySql as sqlx::Database>::QueryResult,
                        sqlx::Error,
                    >,
                > + Send + 'e
                where
                    Self: Sized + 'e,
                    E: sqlx::Executor<'e, Database = sqlx::MySql> + Send + 'e,
                    sqlx::MySql: 'e,
                {
                    __EnhancedQuery_single::execute(self, executor)
                }
            }
            let mut __builder: sqlx::QueryBuilder<sqlx::MySql> = sqlx::QueryBuilder::new(
                "",
            );
            __builder.push("SELECT id, name FROM users ");
            {
                let __enhanced_section_filter_max_id = max_id;
                __builder.push(" WHERE id <= ");
                __builder.push_bind(__enhanced_section_filter_max_id);
                __builder.push(" ");
            }
            let __sql_forge_value_single = __EnhancedQuery_single {
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
        start_line: 171usize,
        start_col: 10usize,
        end_line: 171usize,
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
                    type __EnhancedModel = Row;
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
                                        <sqlx::mysql::MySql as ::sqlx::database::Database>::Arguments::<
                                            '_,
                                        >::default(),
                                    );
                                    ::sqlx::__query_with_result::<
                                        sqlx::mysql::MySql,
                                        _,
                                    >(
                                            "\n        SELECT t1.id AS field_1, o.name AS field_2\n        FROM users t1\n         JOIN organisations o ON o.id = t1.id \n        WHERE 1 = 1\n        ",
                                            query_args,
                                        )
                                        .try_map(|row: sqlx::mysql::MySqlRow| {
                                            use ::sqlx::Row as _;
                                            #[allow(non_snake_case)]
                                            let sqlx_query_as_field_1 = row
                                                .try_get_unchecked::<i64, _>(0usize)?
                                                .into();
                                            #[allow(non_snake_case)]
                                            let sqlx_query_as_field_2 = row
                                                .try_get_unchecked::<String, _>(1usize)?
                                                .into();
                                            ::std::result::Result::Ok(__EnhancedModel {
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
                                        <sqlx::mysql::MySql as ::sqlx::database::Database>::Arguments::<
                                            '_,
                                        >::default(),
                                    );
                                    ::sqlx::__query_with_result::<
                                        sqlx::mysql::MySql,
                                        _,
                                    >(
                                            "\n        SELECT t1.id AS field_1, t1.name AS field_2\n        FROM users t1\n        \n        WHERE 1 = 1\n        ",
                                            query_args,
                                        )
                                        .try_map(|row: sqlx::mysql::MySqlRow| {
                                            use ::sqlx::Row as _;
                                            #[allow(non_snake_case)]
                                            let sqlx_query_as_field_1 = row
                                                .try_get_unchecked::<i64, _>(0usize)?
                                                .into();
                                            #[allow(non_snake_case)]
                                            let sqlx_query_as_field_2 = row
                                                .try_get_unchecked::<String, _>(1usize)?
                                                .into();
                                            ::std::result::Result::Ok(__EnhancedModel {
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
            struct __EnhancedQuery_single<'args> {
                inner: sqlx::QueryBuilder<'args, sqlx::MySql>,
            }
            impl<'args> __EnhancedQuery_single<'args> {
                async fn fetch_all<'e, E>(
                    mut self,
                    executor: E,
                ) -> Result<Vec<Row>, sqlx::Error>
                where
                    E: sqlx::Executor<'e, Database = sqlx::MySql>,
                {
                    self.inner.build_query_as::<Row>().fetch_all(executor).await
                }
                async fn fetch_one<'e, E>(
                    mut self,
                    executor: E,
                ) -> Result<Row, sqlx::Error>
                where
                    E: sqlx::Executor<'e, Database = sqlx::MySql>,
                {
                    self.inner.build_query_as::<Row>().fetch_one(executor).await
                }
                async fn fetch_optional<'e, E>(
                    mut self,
                    executor: E,
                ) -> Result<Option<Row>, sqlx::Error>
                where
                    E: sqlx::Executor<'e, Database = sqlx::MySql>,
                {
                    self.inner.build_query_as::<Row>().fetch_optional(executor).await
                }
                async fn execute<'e, E>(
                    mut self,
                    executor: E,
                ) -> Result<<sqlx::MySql as sqlx::Database>::QueryResult, sqlx::Error>
                where
                    E: sqlx::Executor<'e, Database = sqlx::MySql>,
                {
                    self.inner.build().execute(executor).await
                }
            }
            impl<'args> sql_forge::EnhancedQuery<Row> for __EnhancedQuery_single<'args> {
                type Db = sqlx::MySql;
                fn fetch_all<'e, E>(
                    self,
                    executor: E,
                ) -> impl std::future::Future<
                    Output = Result<Vec<Row>, sqlx::Error>,
                > + Send + 'e
                where
                    Self: Sized + 'e,
                    E: sqlx::Executor<'e, Database = sqlx::MySql> + Send + 'e,
                    sqlx::MySql: 'e,
                {
                    __EnhancedQuery_single::fetch_all(self, executor)
                }
                fn fetch_one<'e, E>(
                    self,
                    executor: E,
                ) -> impl std::future::Future<
                    Output = Result<Row, sqlx::Error>,
                > + Send + 'e
                where
                    Self: Sized + 'e,
                    E: sqlx::Executor<'e, Database = sqlx::MySql> + Send + 'e,
                    sqlx::MySql: 'e,
                {
                    __EnhancedQuery_single::fetch_one(self, executor)
                }
                fn fetch_optional<'e, E>(
                    self,
                    executor: E,
                ) -> impl std::future::Future<
                    Output = Result<Option<Row>, sqlx::Error>,
                > + Send + 'e
                where
                    Self: Sized + 'e,
                    E: sqlx::Executor<'e, Database = sqlx::MySql> + Send + 'e,
                    sqlx::MySql: 'e,
                {
                    __EnhancedQuery_single::fetch_optional(self, executor)
                }
                fn execute<'e, E>(
                    self,
                    executor: E,
                ) -> impl std::future::Future<
                    Output = Result<
                        <sqlx::MySql as sqlx::Database>::QueryResult,
                        sqlx::Error,
                    >,
                > + Send + 'e
                where
                    Self: Sized + 'e,
                    E: sqlx::Executor<'e, Database = sqlx::MySql> + Send + 'e,
                    sqlx::MySql: 'e,
                {
                    __EnhancedQuery_single::execute(self, executor)
                }
            }
            let mut __builder: sqlx::QueryBuilder<sqlx::MySql> = sqlx::QueryBuilder::new(
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
            let __sql_forge_value_single = __EnhancedQuery_single {
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
#[rustc_test_marker = "list_parameter_in_clause"]
#[doc(hidden)]
pub const list_parameter_in_clause: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("list_parameter_in_clause"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "tests/tests.rs",
        start_line: 211usize,
        start_col: 10usize,
        end_line: 211usize,
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
                let __enhanced_top_level_ids = &(ids);
                {
                    type __EnhancedModel = User;
                    {
                        let _ = {
                            {
                                #[allow(clippy::all)]
                                {
                                    use ::sqlx::Arguments as _;
                                    let arg0 = &(*(__enhanced_top_level_ids)
                                        .as_slice()
                                        .first()
                                        .unwrap_or(&0i64));
                                    let arg1 = &(*(__enhanced_top_level_ids)
                                        .as_slice()
                                        .first()
                                        .unwrap_or(&0i64));
                                    let arg2 = &(*(__enhanced_top_level_ids)
                                        .as_slice()
                                        .first()
                                        .unwrap_or(&0i64));
                                    let mut query_args = <sqlx::mysql::MySql as ::sqlx::database::Database>::Arguments::<
                                        '_,
                                    >::default();
                                    query_args
                                        .reserve(
                                            3usize,
                                            0
                                                + ::sqlx::encode::Encode::<
                                                    sqlx::mysql::MySql,
                                                >::size_hint(arg0)
                                                + ::sqlx::encode::Encode::<
                                                    sqlx::mysql::MySql,
                                                >::size_hint(arg1)
                                                + ::sqlx::encode::Encode::<
                                                    sqlx::mysql::MySql,
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
                                        sqlx::mysql::MySql,
                                        _,
                                    >(
                                            "SELECT id, name FROM users WHERE id IN (?, ?, ?)",
                                            query_args,
                                        )
                                        .try_map(|row: sqlx::mysql::MySqlRow| {
                                            use ::sqlx::Row as _;
                                            #[allow(non_snake_case)]
                                            let sqlx_query_as_id = row
                                                .try_get_unchecked::<i64, _>(0usize)?
                                                .into();
                                            #[allow(non_snake_case)]
                                            let sqlx_query_as_name = row
                                                .try_get_unchecked::<String, _>(1usize)?
                                                .into();
                                            ::std::result::Result::Ok(__EnhancedModel {
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
            struct __EnhancedQuery_single<'args> {
                inner: sqlx::QueryBuilder<'args, sqlx::MySql>,
            }
            impl<'args> __EnhancedQuery_single<'args> {
                async fn fetch_all<'e, E>(
                    mut self,
                    executor: E,
                ) -> Result<Vec<User>, sqlx::Error>
                where
                    E: sqlx::Executor<'e, Database = sqlx::MySql>,
                {
                    self.inner.build_query_as::<User>().fetch_all(executor).await
                }
                async fn fetch_one<'e, E>(
                    mut self,
                    executor: E,
                ) -> Result<User, sqlx::Error>
                where
                    E: sqlx::Executor<'e, Database = sqlx::MySql>,
                {
                    self.inner.build_query_as::<User>().fetch_one(executor).await
                }
                async fn fetch_optional<'e, E>(
                    mut self,
                    executor: E,
                ) -> Result<Option<User>, sqlx::Error>
                where
                    E: sqlx::Executor<'e, Database = sqlx::MySql>,
                {
                    self.inner.build_query_as::<User>().fetch_optional(executor).await
                }
                async fn execute<'e, E>(
                    mut self,
                    executor: E,
                ) -> Result<<sqlx::MySql as sqlx::Database>::QueryResult, sqlx::Error>
                where
                    E: sqlx::Executor<'e, Database = sqlx::MySql>,
                {
                    self.inner.build().execute(executor).await
                }
            }
            impl<'args> sql_forge::EnhancedQuery<User>
            for __EnhancedQuery_single<'args> {
                type Db = sqlx::MySql;
                fn fetch_all<'e, E>(
                    self,
                    executor: E,
                ) -> impl std::future::Future<
                    Output = Result<Vec<User>, sqlx::Error>,
                > + Send + 'e
                where
                    Self: Sized + 'e,
                    E: sqlx::Executor<'e, Database = sqlx::MySql> + Send + 'e,
                    sqlx::MySql: 'e,
                {
                    __EnhancedQuery_single::fetch_all(self, executor)
                }
                fn fetch_one<'e, E>(
                    self,
                    executor: E,
                ) -> impl std::future::Future<
                    Output = Result<User, sqlx::Error>,
                > + Send + 'e
                where
                    Self: Sized + 'e,
                    E: sqlx::Executor<'e, Database = sqlx::MySql> + Send + 'e,
                    sqlx::MySql: 'e,
                {
                    __EnhancedQuery_single::fetch_one(self, executor)
                }
                fn fetch_optional<'e, E>(
                    self,
                    executor: E,
                ) -> impl std::future::Future<
                    Output = Result<Option<User>, sqlx::Error>,
                > + Send + 'e
                where
                    Self: Sized + 'e,
                    E: sqlx::Executor<'e, Database = sqlx::MySql> + Send + 'e,
                    sqlx::MySql: 'e,
                {
                    __EnhancedQuery_single::fetch_optional(self, executor)
                }
                fn execute<'e, E>(
                    self,
                    executor: E,
                ) -> impl std::future::Future<
                    Output = Result<
                        <sqlx::MySql as sqlx::Database>::QueryResult,
                        sqlx::Error,
                    >,
                > + Send + 'e
                where
                    Self: Sized + 'e,
                    E: sqlx::Executor<'e, Database = sqlx::MySql> + Send + 'e,
                    sqlx::MySql: 'e,
                {
                    __EnhancedQuery_single::execute(self, executor)
                }
            }
            let __enhanced_runtime_ids = ids;
            let mut __builder: sqlx::QueryBuilder<sqlx::MySql> = sqlx::QueryBuilder::new(
                "",
            );
            __builder.push("SELECT id, name FROM users WHERE id IN (");
            let __enhanced_values = __enhanced_runtime_ids;
            let mut __separated = __builder.separated(", ");
            for __value in __enhanced_values {
                __separated.push_bind(__value);
            }
            __builder.push(")");
            let __sql_forge_value_single = __EnhancedQuery_single {
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
#[rustc_test_marker = "list_parameter_with_empty_guard"]
#[doc(hidden)]
pub const list_parameter_with_empty_guard: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("list_parameter_with_empty_guard"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "tests/tests.rs",
        start_line: 232usize,
        start_col: 10usize,
        end_line: 232usize,
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
                    type __EnhancedModel = User;
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
                                        <sqlx::mysql::MySql as ::sqlx::database::Database>::Arguments::<
                                            '_,
                                        >::default(),
                                    );
                                    ::sqlx::__query_with_result::<
                                        sqlx::mysql::MySql,
                                        _,
                                    >("SELECT id, name FROM users WHERE 1 = 0", query_args)
                                        .try_map(|row: sqlx::mysql::MySqlRow| {
                                            use ::sqlx::Row as _;
                                            #[allow(non_snake_case)]
                                            let sqlx_query_as_id = row
                                                .try_get_unchecked::<i64, _>(0usize)?
                                                .into();
                                            #[allow(non_snake_case)]
                                            let sqlx_query_as_name = row
                                                .try_get_unchecked::<String, _>(1usize)?
                                                .into();
                                            ::std::result::Result::Ok(__EnhancedModel {
                                                id: sqlx_query_as_id,
                                                name: sqlx_query_as_name,
                                            })
                                        })
                                }
                            }
                        };
                    }
                    {
                        let __enhanced_section_case_single_1_filter_ids = &(ids);
                        let _ = {
                            {
                                #[allow(clippy::all)]
                                {
                                    use ::sqlx::Arguments as _;
                                    let arg0 = &(*(__enhanced_section_case_single_1_filter_ids)
                                        .as_slice()
                                        .first()
                                        .unwrap_or(&0i64));
                                    let arg1 = &(*(__enhanced_section_case_single_1_filter_ids)
                                        .as_slice()
                                        .first()
                                        .unwrap_or(&0i64));
                                    let arg2 = &(*(__enhanced_section_case_single_1_filter_ids)
                                        .as_slice()
                                        .first()
                                        .unwrap_or(&0i64));
                                    let mut query_args = <sqlx::mysql::MySql as ::sqlx::database::Database>::Arguments::<
                                        '_,
                                    >::default();
                                    query_args
                                        .reserve(
                                            3usize,
                                            0
                                                + ::sqlx::encode::Encode::<
                                                    sqlx::mysql::MySql,
                                                >::size_hint(arg0)
                                                + ::sqlx::encode::Encode::<
                                                    sqlx::mysql::MySql,
                                                >::size_hint(arg1)
                                                + ::sqlx::encode::Encode::<
                                                    sqlx::mysql::MySql,
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
                                        sqlx::mysql::MySql,
                                        _,
                                    >(
                                            "SELECT id, name FROM users WHERE id IN (?, ?, ?)",
                                            query_args,
                                        )
                                        .try_map(|row: sqlx::mysql::MySqlRow| {
                                            use ::sqlx::Row as _;
                                            #[allow(non_snake_case)]
                                            let sqlx_query_as_id = row
                                                .try_get_unchecked::<i64, _>(0usize)?
                                                .into();
                                            #[allow(non_snake_case)]
                                            let sqlx_query_as_name = row
                                                .try_get_unchecked::<String, _>(1usize)?
                                                .into();
                                            ::std::result::Result::Ok(__EnhancedModel {
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
            struct __EnhancedQuery_single<'args> {
                inner: sqlx::QueryBuilder<'args, sqlx::MySql>,
            }
            impl<'args> __EnhancedQuery_single<'args> {
                async fn fetch_all<'e, E>(
                    mut self,
                    executor: E,
                ) -> Result<Vec<User>, sqlx::Error>
                where
                    E: sqlx::Executor<'e, Database = sqlx::MySql>,
                {
                    self.inner.build_query_as::<User>().fetch_all(executor).await
                }
                async fn fetch_one<'e, E>(
                    mut self,
                    executor: E,
                ) -> Result<User, sqlx::Error>
                where
                    E: sqlx::Executor<'e, Database = sqlx::MySql>,
                {
                    self.inner.build_query_as::<User>().fetch_one(executor).await
                }
                async fn fetch_optional<'e, E>(
                    mut self,
                    executor: E,
                ) -> Result<Option<User>, sqlx::Error>
                where
                    E: sqlx::Executor<'e, Database = sqlx::MySql>,
                {
                    self.inner.build_query_as::<User>().fetch_optional(executor).await
                }
                async fn execute<'e, E>(
                    mut self,
                    executor: E,
                ) -> Result<<sqlx::MySql as sqlx::Database>::QueryResult, sqlx::Error>
                where
                    E: sqlx::Executor<'e, Database = sqlx::MySql>,
                {
                    self.inner.build().execute(executor).await
                }
            }
            impl<'args> sql_forge::EnhancedQuery<User>
            for __EnhancedQuery_single<'args> {
                type Db = sqlx::MySql;
                fn fetch_all<'e, E>(
                    self,
                    executor: E,
                ) -> impl std::future::Future<
                    Output = Result<Vec<User>, sqlx::Error>,
                > + Send + 'e
                where
                    Self: Sized + 'e,
                    E: sqlx::Executor<'e, Database = sqlx::MySql> + Send + 'e,
                    sqlx::MySql: 'e,
                {
                    __EnhancedQuery_single::fetch_all(self, executor)
                }
                fn fetch_one<'e, E>(
                    self,
                    executor: E,
                ) -> impl std::future::Future<
                    Output = Result<User, sqlx::Error>,
                > + Send + 'e
                where
                    Self: Sized + 'e,
                    E: sqlx::Executor<'e, Database = sqlx::MySql> + Send + 'e,
                    sqlx::MySql: 'e,
                {
                    __EnhancedQuery_single::fetch_one(self, executor)
                }
                fn fetch_optional<'e, E>(
                    self,
                    executor: E,
                ) -> impl std::future::Future<
                    Output = Result<Option<User>, sqlx::Error>,
                > + Send + 'e
                where
                    Self: Sized + 'e,
                    E: sqlx::Executor<'e, Database = sqlx::MySql> + Send + 'e,
                    sqlx::MySql: 'e,
                {
                    __EnhancedQuery_single::fetch_optional(self, executor)
                }
                fn execute<'e, E>(
                    self,
                    executor: E,
                ) -> impl std::future::Future<
                    Output = Result<
                        <sqlx::MySql as sqlx::Database>::QueryResult,
                        sqlx::Error,
                    >,
                > + Send + 'e
                where
                    Self: Sized + 'e,
                    E: sqlx::Executor<'e, Database = sqlx::MySql> + Send + 'e,
                    sqlx::MySql: 'e,
                {
                    __EnhancedQuery_single::execute(self, executor)
                }
            }
            let mut __builder: sqlx::QueryBuilder<sqlx::MySql> = sqlx::QueryBuilder::new(
                "",
            );
            __builder.push("SELECT id, name FROM users WHERE ");
            match ids.is_empty() {
                true => {
                    __builder.push("1 = 0");
                }
                false => {
                    let __enhanced_section_filter_1_ids = ids;
                    __builder.push("id IN (");
                    let __enhanced_values = __enhanced_section_filter_1_ids;
                    let mut __separated = __builder.separated(", ");
                    for __value in __enhanced_values {
                        __separated.push_bind(__value);
                    }
                    __builder.push(")");
                }
            }
            let __sql_forge_value_single = __EnhancedQuery_single {
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
        start_line: 258usize,
        start_col: 10usize,
        end_line: 258usize,
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
                let __enhanced_top_level_category_id = &(category_id);
                let __enhanced_top_level_min_price = &(min_price);
                {
                    let __enhanced_result_flag_amount: bool = true;
                    let __enhanced_result_flag_list: bool = false;
                    type __EnhancedModel = AmountResult;
                    {
                        let _ = {
                            {
                                #[allow(clippy::all)]
                                {
                                    use ::sqlx::Arguments as _;
                                    let arg0 = &(__enhanced_top_level_category_id);
                                    let arg1 = &(__enhanced_top_level_min_price);
                                    let mut query_args = <sqlx::mysql::MySql as ::sqlx::database::Database>::Arguments::<
                                        '_,
                                    >::default();
                                    query_args
                                        .reserve(
                                            2usize,
                                            0
                                                + ::sqlx::encode::Encode::<
                                                    sqlx::mysql::MySql,
                                                >::size_hint(arg0)
                                                + ::sqlx::encode::Encode::<
                                                    sqlx::mysql::MySql,
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
                                        sqlx::mysql::MySql,
                                        _,
                                    >(
                                            "\n        SELECT COUNT(*) AS total\n        FROM items\n        \n        WHERE items.category_id = ?\n        AND   items.price      >= ?\n        \n        ",
                                            query_args,
                                        )
                                        .try_map(|row: sqlx::mysql::MySqlRow| {
                                            use ::sqlx::Row as _;
                                            #[allow(non_snake_case)]
                                            let sqlx_query_as_total = row
                                                .try_get_unchecked::<i64, _>(0usize)?
                                                .into();
                                            ::std::result::Result::Ok(__EnhancedModel {
                                                total: sqlx_query_as_total,
                                            })
                                        })
                                }
                            }
                        };
                    }
                }
                {
                    let __enhanced_result_flag_amount: bool = false;
                    let __enhanced_result_flag_list: bool = true;
                    type __EnhancedModel = Item;
                    {
                        let __enhanced_section_case_list_0_order_limit_start = &(0i64);
                        let __enhanced_section_case_list_0_order_limit_limit = &(50i64);
                        let _ = {
                            {
                                #[allow(clippy::all)]
                                {
                                    use ::sqlx::Arguments as _;
                                    let arg0 = &(__enhanced_top_level_category_id);
                                    let arg1 = &(__enhanced_top_level_min_price);
                                    let arg2 = &(__enhanced_section_case_list_0_order_limit_limit);
                                    let arg3 = &(__enhanced_section_case_list_0_order_limit_start);
                                    let mut query_args = <sqlx::mysql::MySql as ::sqlx::database::Database>::Arguments::<
                                        '_,
                                    >::default();
                                    query_args
                                        .reserve(
                                            4usize,
                                            0
                                                + ::sqlx::encode::Encode::<
                                                    sqlx::mysql::MySql,
                                                >::size_hint(arg0)
                                                + ::sqlx::encode::Encode::<
                                                    sqlx::mysql::MySql,
                                                >::size_hint(arg1)
                                                + ::sqlx::encode::Encode::<
                                                    sqlx::mysql::MySql,
                                                >::size_hint(arg2)
                                                + ::sqlx::encode::Encode::<
                                                    sqlx::mysql::MySql,
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
                                        sqlx::mysql::MySql,
                                        _,
                                    >(
                                            "\n        SELECT items.id, items.name, items.price, items.stock\n        FROM items\n        JOIN categories ON categories.id = items.category_id\n        WHERE items.category_id = ?\n        AND   items.price      >= ?\n        ORDER BY items.created_at DESC LIMIT ? OFFSET ?\n        ",
                                            query_args,
                                        )
                                        .try_map(|row: sqlx::mysql::MySqlRow| {
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
                                                .try_get_unchecked::<i32, _>(3usize)?
                                                .into();
                                            ::std::result::Result::Ok(__EnhancedModel {
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
            struct __EnhancedQuery_amount<'args> {
                inner: sqlx::QueryBuilder<'args, sqlx::MySql>,
            }
            impl<'args> __EnhancedQuery_amount<'args> {
                async fn fetch_all<'e, E>(
                    mut self,
                    executor: E,
                ) -> Result<Vec<AmountResult>, sqlx::Error>
                where
                    E: sqlx::Executor<'e, Database = sqlx::MySql>,
                {
                    self.inner.build_query_as::<AmountResult>().fetch_all(executor).await
                }
                async fn fetch_one<'e, E>(
                    mut self,
                    executor: E,
                ) -> Result<AmountResult, sqlx::Error>
                where
                    E: sqlx::Executor<'e, Database = sqlx::MySql>,
                {
                    self.inner.build_query_as::<AmountResult>().fetch_one(executor).await
                }
                async fn fetch_optional<'e, E>(
                    mut self,
                    executor: E,
                ) -> Result<Option<AmountResult>, sqlx::Error>
                where
                    E: sqlx::Executor<'e, Database = sqlx::MySql>,
                {
                    self.inner
                        .build_query_as::<AmountResult>()
                        .fetch_optional(executor)
                        .await
                }
                async fn execute<'e, E>(
                    mut self,
                    executor: E,
                ) -> Result<<sqlx::MySql as sqlx::Database>::QueryResult, sqlx::Error>
                where
                    E: sqlx::Executor<'e, Database = sqlx::MySql>,
                {
                    self.inner.build().execute(executor).await
                }
            }
            impl<'args> sql_forge::EnhancedQuery<AmountResult>
            for __EnhancedQuery_amount<'args> {
                type Db = sqlx::MySql;
                fn fetch_all<'e, E>(
                    self,
                    executor: E,
                ) -> impl std::future::Future<
                    Output = Result<Vec<AmountResult>, sqlx::Error>,
                > + Send + 'e
                where
                    Self: Sized + 'e,
                    E: sqlx::Executor<'e, Database = sqlx::MySql> + Send + 'e,
                    sqlx::MySql: 'e,
                {
                    __EnhancedQuery_amount::fetch_all(self, executor)
                }
                fn fetch_one<'e, E>(
                    self,
                    executor: E,
                ) -> impl std::future::Future<
                    Output = Result<AmountResult, sqlx::Error>,
                > + Send + 'e
                where
                    Self: Sized + 'e,
                    E: sqlx::Executor<'e, Database = sqlx::MySql> + Send + 'e,
                    sqlx::MySql: 'e,
                {
                    __EnhancedQuery_amount::fetch_one(self, executor)
                }
                fn fetch_optional<'e, E>(
                    self,
                    executor: E,
                ) -> impl std::future::Future<
                    Output = Result<Option<AmountResult>, sqlx::Error>,
                > + Send + 'e
                where
                    Self: Sized + 'e,
                    E: sqlx::Executor<'e, Database = sqlx::MySql> + Send + 'e,
                    sqlx::MySql: 'e,
                {
                    __EnhancedQuery_amount::fetch_optional(self, executor)
                }
                fn execute<'e, E>(
                    self,
                    executor: E,
                ) -> impl std::future::Future<
                    Output = Result<
                        <sqlx::MySql as sqlx::Database>::QueryResult,
                        sqlx::Error,
                    >,
                > + Send + 'e
                where
                    Self: Sized + 'e,
                    E: sqlx::Executor<'e, Database = sqlx::MySql> + Send + 'e,
                    sqlx::MySql: 'e,
                {
                    __EnhancedQuery_amount::execute(self, executor)
                }
            }
            struct __EnhancedQuery_list<'args> {
                inner: sqlx::QueryBuilder<'args, sqlx::MySql>,
            }
            impl<'args> __EnhancedQuery_list<'args> {
                async fn fetch_all<'e, E>(
                    mut self,
                    executor: E,
                ) -> Result<Vec<Item>, sqlx::Error>
                where
                    E: sqlx::Executor<'e, Database = sqlx::MySql>,
                {
                    self.inner.build_query_as::<Item>().fetch_all(executor).await
                }
                async fn fetch_one<'e, E>(
                    mut self,
                    executor: E,
                ) -> Result<Item, sqlx::Error>
                where
                    E: sqlx::Executor<'e, Database = sqlx::MySql>,
                {
                    self.inner.build_query_as::<Item>().fetch_one(executor).await
                }
                async fn fetch_optional<'e, E>(
                    mut self,
                    executor: E,
                ) -> Result<Option<Item>, sqlx::Error>
                where
                    E: sqlx::Executor<'e, Database = sqlx::MySql>,
                {
                    self.inner.build_query_as::<Item>().fetch_optional(executor).await
                }
                async fn execute<'e, E>(
                    mut self,
                    executor: E,
                ) -> Result<<sqlx::MySql as sqlx::Database>::QueryResult, sqlx::Error>
                where
                    E: sqlx::Executor<'e, Database = sqlx::MySql>,
                {
                    self.inner.build().execute(executor).await
                }
            }
            impl<'args> sql_forge::EnhancedQuery<Item> for __EnhancedQuery_list<'args> {
                type Db = sqlx::MySql;
                fn fetch_all<'e, E>(
                    self,
                    executor: E,
                ) -> impl std::future::Future<
                    Output = Result<Vec<Item>, sqlx::Error>,
                > + Send + 'e
                where
                    Self: Sized + 'e,
                    E: sqlx::Executor<'e, Database = sqlx::MySql> + Send + 'e,
                    sqlx::MySql: 'e,
                {
                    __EnhancedQuery_list::fetch_all(self, executor)
                }
                fn fetch_one<'e, E>(
                    self,
                    executor: E,
                ) -> impl std::future::Future<
                    Output = Result<Item, sqlx::Error>,
                > + Send + 'e
                where
                    Self: Sized + 'e,
                    E: sqlx::Executor<'e, Database = sqlx::MySql> + Send + 'e,
                    sqlx::MySql: 'e,
                {
                    __EnhancedQuery_list::fetch_one(self, executor)
                }
                fn fetch_optional<'e, E>(
                    self,
                    executor: E,
                ) -> impl std::future::Future<
                    Output = Result<Option<Item>, sqlx::Error>,
                > + Send + 'e
                where
                    Self: Sized + 'e,
                    E: sqlx::Executor<'e, Database = sqlx::MySql> + Send + 'e,
                    sqlx::MySql: 'e,
                {
                    __EnhancedQuery_list::fetch_optional(self, executor)
                }
                fn execute<'e, E>(
                    self,
                    executor: E,
                ) -> impl std::future::Future<
                    Output = Result<
                        <sqlx::MySql as sqlx::Database>::QueryResult,
                        sqlx::Error,
                    >,
                > + Send + 'e
                where
                    Self: Sized + 'e,
                    E: sqlx::Executor<'e, Database = sqlx::MySql> + Send + 'e,
                    sqlx::MySql: 'e,
                {
                    __EnhancedQuery_list::execute(self, executor)
                }
            }
            let __enhanced_runtime_category_id = category_id;
            let __enhanced_runtime_min_price = min_price;
            let __enhanced_result_flag_amount: bool = true;
            let __enhanced_result_flag_list: bool = false;
            let mut __builder: sqlx::QueryBuilder<sqlx::MySql> = sqlx::QueryBuilder::new(
                "",
            );
            __builder.push("\n        SELECT ");
            match __enhanced_result_flag_amount {
                true => {
                    __builder.push("COUNT(*) AS total");
                }
                false => {
                    __builder.push("items.id, items.name, items.price, items.stock");
                }
            }
            __builder.push("\n        FROM items\n        ");
            match __enhanced_result_flag_amount {
                true => {}
                false => {
                    __builder
                        .push("JOIN categories ON categories.id = items.category_id");
                }
            }
            __builder.push("\n        WHERE items.category_id = ");
            __builder.push_bind(__enhanced_runtime_category_id);
            __builder.push("\n        AND   items.price      >= ");
            __builder.push_bind(__enhanced_runtime_min_price);
            __builder.push("\n        ");
            match __enhanced_result_flag_amount {
                true => {}
                false => {
                    let __enhanced_section_order_limit_1_start = 0i64;
                    let __enhanced_section_order_limit_1_limit = 50i64;
                    __builder.push("ORDER BY items.created_at DESC LIMIT ");
                    __builder.push_bind(__enhanced_section_order_limit_1_limit);
                    __builder.push(" OFFSET ");
                    __builder.push_bind(__enhanced_section_order_limit_1_start);
                }
            }
            __builder.push("\n        ");
            let __sql_forge_value_amount = __EnhancedQuery_amount {
                inner: __builder,
            };
            let __enhanced_runtime_category_id = category_id;
            let __enhanced_runtime_min_price = min_price;
            let __enhanced_result_flag_amount: bool = false;
            let __enhanced_result_flag_list: bool = true;
            let mut __builder: sqlx::QueryBuilder<sqlx::MySql> = sqlx::QueryBuilder::new(
                "",
            );
            __builder.push("\n        SELECT ");
            match __enhanced_result_flag_amount {
                true => {
                    __builder.push("COUNT(*) AS total");
                }
                false => {
                    __builder.push("items.id, items.name, items.price, items.stock");
                }
            }
            __builder.push("\n        FROM items\n        ");
            match __enhanced_result_flag_amount {
                true => {}
                false => {
                    __builder
                        .push("JOIN categories ON categories.id = items.category_id");
                }
            }
            __builder.push("\n        WHERE items.category_id = ");
            __builder.push_bind(__enhanced_runtime_category_id);
            __builder.push("\n        AND   items.price      >= ");
            __builder.push_bind(__enhanced_runtime_min_price);
            __builder.push("\n        ");
            match __enhanced_result_flag_amount {
                true => {}
                false => {
                    let __enhanced_section_order_limit_1_start = 0i64;
                    let __enhanced_section_order_limit_1_limit = 50i64;
                    __builder.push("ORDER BY items.created_at DESC LIMIT ");
                    __builder.push_bind(__enhanced_section_order_limit_1_limit);
                    __builder.push(" OFFSET ");
                    __builder.push_bind(__enhanced_section_order_limit_1_start);
                }
            }
            __builder.push("\n        ");
            let __sql_forge_value_list = __EnhancedQuery_list {
                inner: __builder,
            };
            struct __EnhancedQueryGroup<'args> {
                amount: __EnhancedQuery_amount<'args>,
                list: __EnhancedQuery_list<'args>,
            }
            impl<'args> __EnhancedQueryGroup<'args> {
                pub fn amount(self) -> __EnhancedQuery_amount<'args> {
                    self.amount
                }
                pub fn list(self) -> __EnhancedQuery_list<'args> {
                    self.list
                }
                pub fn into_parts(
                    self,
                ) -> (__EnhancedQuery_amount<'args>, __EnhancedQuery_list<'args>) {
                    (self.amount, self.list)
                }
            }
            impl<'args> sql_forge::EnhancedQueryGroup for __EnhancedQueryGroup<'args> {
                type Db = sqlx::MySql;
            }
            struct __EnhancedQueryGroupKey_amount;
            impl<
                'args,
            > sql_forge::EnhancedQueryGroupGet<
                __EnhancedQueryGroupKey_amount,
                AmountResult,
            > for __EnhancedQueryGroup<'args> {
                type Query = __EnhancedQuery_amount<'args>;
                fn get(self, _: __EnhancedQueryGroupKey_amount) -> Self::Query {
                    self.amount
                }
            }
            struct __EnhancedQueryGroupKey_list;
            impl<
                'args,
            > sql_forge::EnhancedQueryGroupGet<__EnhancedQueryGroupKey_list, Item>
            for __EnhancedQueryGroup<'args> {
                type Query = __EnhancedQuery_list<'args>;
                fn get(self, _: __EnhancedQueryGroupKey_list) -> Self::Query {
                    self.list
                }
            }
            __EnhancedQueryGroup {
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
        start_line: 321usize,
        start_col: 10usize,
        end_line: 321usize,
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
                let __enhanced_top_level_category_id = &(category_id);
                {
                    let __enhanced_result_flag_amount: bool = true;
                    let __enhanced_result_flag_first_name: bool = false;
                    {
                        let _ = {
                            #[allow(clippy::all)]
                            {
                                use ::sqlx::Arguments as _;
                                let arg0 = &(__enhanced_top_level_category_id);
                                let mut query_args = <sqlx::mysql::MySql as ::sqlx::database::Database>::Arguments::<
                                    '_,
                                >::default();
                                query_args
                                    .reserve(
                                        1usize,
                                        0
                                            + ::sqlx::encode::Encode::<
                                                sqlx::mysql::MySql,
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
                                    sqlx::mysql::MySql,
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
                    let __enhanced_result_flag_amount: bool = false;
                    let __enhanced_result_flag_first_name: bool = true;
                    {
                        let _ = {
                            #[allow(clippy::all)]
                            {
                                use ::sqlx::Arguments as _;
                                let arg0 = &(__enhanced_top_level_category_id);
                                let mut query_args = <sqlx::mysql::MySql as ::sqlx::database::Database>::Arguments::<
                                    '_,
                                >::default();
                                query_args
                                    .reserve(
                                        1usize,
                                        0
                                            + ::sqlx::encode::Encode::<
                                                sqlx::mysql::MySql,
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
                                    sqlx::mysql::MySql,
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
            struct __EnhancedQuery_amount<'args> {
                inner: sqlx::QueryBuilder<'args, sqlx::MySql>,
            }
            impl<'args> __EnhancedQuery_amount<'args> {
                async fn fetch_all<'e, E>(
                    mut self,
                    executor: E,
                ) -> Result<Vec<i64>, sqlx::Error>
                where
                    E: sqlx::Executor<'e, Database = sqlx::MySql>,
                {
                    self.inner.build_query_scalar::<i64>().fetch_all(executor).await
                }
                async fn fetch_one<'e, E>(
                    mut self,
                    executor: E,
                ) -> Result<i64, sqlx::Error>
                where
                    E: sqlx::Executor<'e, Database = sqlx::MySql>,
                {
                    self.inner.build_query_scalar::<i64>().fetch_one(executor).await
                }
                async fn fetch_optional<'e, E>(
                    mut self,
                    executor: E,
                ) -> Result<Option<i64>, sqlx::Error>
                where
                    E: sqlx::Executor<'e, Database = sqlx::MySql>,
                {
                    self.inner.build_query_scalar::<i64>().fetch_optional(executor).await
                }
                async fn execute<'e, E>(
                    mut self,
                    executor: E,
                ) -> Result<<sqlx::MySql as sqlx::Database>::QueryResult, sqlx::Error>
                where
                    E: sqlx::Executor<'e, Database = sqlx::MySql>,
                {
                    self.inner.build().execute(executor).await
                }
            }
            impl<'args> sql_forge::EnhancedQuery<i64> for __EnhancedQuery_amount<'args> {
                type Db = sqlx::MySql;
                fn fetch_all<'e, E>(
                    self,
                    executor: E,
                ) -> impl std::future::Future<
                    Output = Result<Vec<i64>, sqlx::Error>,
                > + Send + 'e
                where
                    Self: Sized + 'e,
                    E: sqlx::Executor<'e, Database = sqlx::MySql> + Send + 'e,
                    sqlx::MySql: 'e,
                {
                    __EnhancedQuery_amount::fetch_all(self, executor)
                }
                fn fetch_one<'e, E>(
                    self,
                    executor: E,
                ) -> impl std::future::Future<
                    Output = Result<i64, sqlx::Error>,
                > + Send + 'e
                where
                    Self: Sized + 'e,
                    E: sqlx::Executor<'e, Database = sqlx::MySql> + Send + 'e,
                    sqlx::MySql: 'e,
                {
                    __EnhancedQuery_amount::fetch_one(self, executor)
                }
                fn fetch_optional<'e, E>(
                    self,
                    executor: E,
                ) -> impl std::future::Future<
                    Output = Result<Option<i64>, sqlx::Error>,
                > + Send + 'e
                where
                    Self: Sized + 'e,
                    E: sqlx::Executor<'e, Database = sqlx::MySql> + Send + 'e,
                    sqlx::MySql: 'e,
                {
                    __EnhancedQuery_amount::fetch_optional(self, executor)
                }
                fn execute<'e, E>(
                    self,
                    executor: E,
                ) -> impl std::future::Future<
                    Output = Result<
                        <sqlx::MySql as sqlx::Database>::QueryResult,
                        sqlx::Error,
                    >,
                > + Send + 'e
                where
                    Self: Sized + 'e,
                    E: sqlx::Executor<'e, Database = sqlx::MySql> + Send + 'e,
                    sqlx::MySql: 'e,
                {
                    __EnhancedQuery_amount::execute(self, executor)
                }
            }
            struct __EnhancedQuery_first_name<'args> {
                inner: sqlx::QueryBuilder<'args, sqlx::MySql>,
            }
            impl<'args> __EnhancedQuery_first_name<'args> {
                async fn fetch_all<'e, E>(
                    mut self,
                    executor: E,
                ) -> Result<Vec<String>, sqlx::Error>
                where
                    E: sqlx::Executor<'e, Database = sqlx::MySql>,
                {
                    self.inner.build_query_scalar::<String>().fetch_all(executor).await
                }
                async fn fetch_one<'e, E>(
                    mut self,
                    executor: E,
                ) -> Result<String, sqlx::Error>
                where
                    E: sqlx::Executor<'e, Database = sqlx::MySql>,
                {
                    self.inner.build_query_scalar::<String>().fetch_one(executor).await
                }
                async fn fetch_optional<'e, E>(
                    mut self,
                    executor: E,
                ) -> Result<Option<String>, sqlx::Error>
                where
                    E: sqlx::Executor<'e, Database = sqlx::MySql>,
                {
                    self.inner
                        .build_query_scalar::<String>()
                        .fetch_optional(executor)
                        .await
                }
                async fn execute<'e, E>(
                    mut self,
                    executor: E,
                ) -> Result<<sqlx::MySql as sqlx::Database>::QueryResult, sqlx::Error>
                where
                    E: sqlx::Executor<'e, Database = sqlx::MySql>,
                {
                    self.inner.build().execute(executor).await
                }
            }
            impl<'args> sql_forge::EnhancedQuery<String>
            for __EnhancedQuery_first_name<'args> {
                type Db = sqlx::MySql;
                fn fetch_all<'e, E>(
                    self,
                    executor: E,
                ) -> impl std::future::Future<
                    Output = Result<Vec<String>, sqlx::Error>,
                > + Send + 'e
                where
                    Self: Sized + 'e,
                    E: sqlx::Executor<'e, Database = sqlx::MySql> + Send + 'e,
                    sqlx::MySql: 'e,
                {
                    __EnhancedQuery_first_name::fetch_all(self, executor)
                }
                fn fetch_one<'e, E>(
                    self,
                    executor: E,
                ) -> impl std::future::Future<
                    Output = Result<String, sqlx::Error>,
                > + Send + 'e
                where
                    Self: Sized + 'e,
                    E: sqlx::Executor<'e, Database = sqlx::MySql> + Send + 'e,
                    sqlx::MySql: 'e,
                {
                    __EnhancedQuery_first_name::fetch_one(self, executor)
                }
                fn fetch_optional<'e, E>(
                    self,
                    executor: E,
                ) -> impl std::future::Future<
                    Output = Result<Option<String>, sqlx::Error>,
                > + Send + 'e
                where
                    Self: Sized + 'e,
                    E: sqlx::Executor<'e, Database = sqlx::MySql> + Send + 'e,
                    sqlx::MySql: 'e,
                {
                    __EnhancedQuery_first_name::fetch_optional(self, executor)
                }
                fn execute<'e, E>(
                    self,
                    executor: E,
                ) -> impl std::future::Future<
                    Output = Result<
                        <sqlx::MySql as sqlx::Database>::QueryResult,
                        sqlx::Error,
                    >,
                > + Send + 'e
                where
                    Self: Sized + 'e,
                    E: sqlx::Executor<'e, Database = sqlx::MySql> + Send + 'e,
                    sqlx::MySql: 'e,
                {
                    __EnhancedQuery_first_name::execute(self, executor)
                }
            }
            let __enhanced_runtime_category_id = category_id;
            let __enhanced_result_flag_amount: bool = true;
            let __enhanced_result_flag_first_name: bool = false;
            let mut __builder: sqlx::QueryBuilder<sqlx::MySql> = sqlx::QueryBuilder::new(
                "",
            );
            __builder.push("\n        SELECT ");
            match __enhanced_result_flag_amount {
                true => {
                    __builder.push("COUNT(*)");
                }
                false => {
                    __builder.push("items.name");
                }
            }
            __builder.push("\n        FROM items\n        WHERE items.category_id = ");
            __builder.push_bind(__enhanced_runtime_category_id);
            __builder.push("\n        ");
            let __sql_forge_value_amount = __EnhancedQuery_amount {
                inner: __builder,
            };
            let __enhanced_runtime_category_id = category_id;
            let __enhanced_result_flag_amount: bool = false;
            let __enhanced_result_flag_first_name: bool = true;
            let mut __builder: sqlx::QueryBuilder<sqlx::MySql> = sqlx::QueryBuilder::new(
                "",
            );
            __builder.push("\n        SELECT ");
            match __enhanced_result_flag_amount {
                true => {
                    __builder.push("COUNT(*)");
                }
                false => {
                    __builder.push("items.name");
                }
            }
            __builder.push("\n        FROM items\n        WHERE items.category_id = ");
            __builder.push_bind(__enhanced_runtime_category_id);
            __builder.push("\n        ");
            let __sql_forge_value_first_name = __EnhancedQuery_first_name {
                inner: __builder,
            };
            struct __EnhancedQueryGroup<'args> {
                amount: __EnhancedQuery_amount<'args>,
                first_name: __EnhancedQuery_first_name<'args>,
            }
            impl<'args> __EnhancedQueryGroup<'args> {
                pub fn amount(self) -> __EnhancedQuery_amount<'args> {
                    self.amount
                }
                pub fn first_name(self) -> __EnhancedQuery_first_name<'args> {
                    self.first_name
                }
                pub fn into_parts(
                    self,
                ) -> (__EnhancedQuery_amount<'args>, __EnhancedQuery_first_name<'args>) {
                    (self.amount, self.first_name)
                }
            }
            impl<'args> sql_forge::EnhancedQueryGroup for __EnhancedQueryGroup<'args> {
                type Db = sqlx::MySql;
            }
            struct __EnhancedQueryGroupKey_amount;
            impl<
                'args,
            > sql_forge::EnhancedQueryGroupGet<__EnhancedQueryGroupKey_amount, i64>
            for __EnhancedQueryGroup<'args> {
                type Query = __EnhancedQuery_amount<'args>;
                fn get(self, _: __EnhancedQueryGroupKey_amount) -> Self::Query {
                    self.amount
                }
            }
            struct __EnhancedQueryGroupKey_first_name;
            impl<
                'args,
            > sql_forge::EnhancedQueryGroupGet<
                __EnhancedQueryGroupKey_first_name,
                String,
            > for __EnhancedQueryGroup<'args> {
                type Query = __EnhancedQuery_first_name<'args>;
                fn get(self, _: __EnhancedQueryGroupKey_first_name) -> Self::Query {
                    self.first_name
                }
            }
            __EnhancedQueryGroup {
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
        start_line: 362usize,
        start_col: 10usize,
        end_line: 362usize,
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
                    type __EnhancedModel = Product;
                    {
                        let __enhanced_section_case_single_0_filter_category_cat = &(category
                            .unwrap());
                        let __enhanced_section_case_single_0_filter_price_min_price_min = &(price_min
                            .unwrap());
                        let __enhanced_section_case_single_0_filter_price_max_price_max = &(price_max
                            .unwrap());
                        let __enhanced_section_case_single_0_limit_offset = &(page
                            * page_size.unwrap());
                        let __enhanced_section_case_single_0_limit_size = &(page_size
                            .unwrap());
                        let _ = {
                            {
                                #[allow(clippy::all)]
                                {
                                    use ::sqlx::Arguments as _;
                                    let arg0 = &(__enhanced_section_case_single_0_filter_category_cat);
                                    let arg1 = &(__enhanced_section_case_single_0_filter_price_min_price_min);
                                    let arg2 = &(__enhanced_section_case_single_0_filter_price_max_price_max);
                                    let arg3 = &(__enhanced_section_case_single_0_limit_size);
                                    let arg4 = &(__enhanced_section_case_single_0_limit_offset);
                                    let mut query_args = <sqlx::mysql::MySql as ::sqlx::database::Database>::Arguments::<
                                        '_,
                                    >::default();
                                    query_args
                                        .reserve(
                                            5usize,
                                            0
                                                + ::sqlx::encode::Encode::<
                                                    sqlx::mysql::MySql,
                                                >::size_hint(arg0)
                                                + ::sqlx::encode::Encode::<
                                                    sqlx::mysql::MySql,
                                                >::size_hint(arg1)
                                                + ::sqlx::encode::Encode::<
                                                    sqlx::mysql::MySql,
                                                >::size_hint(arg2)
                                                + ::sqlx::encode::Encode::<
                                                    sqlx::mysql::MySql,
                                                >::size_hint(arg3)
                                                + ::sqlx::encode::Encode::<
                                                    sqlx::mysql::MySql,
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
                                        sqlx::mysql::MySql,
                                        _,
                                    >(
                                            "\n        SELECT\n            p.id,\n            p.name,\n            p.price,\n            p.stock,\n            p.category\n        FROM products p\n        WHERE 1 = 1\n         AND p.category = ? \n         AND p.price >= ? \n         AND p.price <= ? \n         AND p.stock > 0 \n         ORDER BY p.price ASC \n         LIMIT ? OFFSET ? \n        ",
                                            query_args,
                                        )
                                        .try_map(|row: sqlx::mysql::MySqlRow| {
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
                                                .try_get_unchecked::<i32, _>(3usize)?
                                                .into();
                                            #[allow(non_snake_case)]
                                            let sqlx_query_as_category = row
                                                .try_get_unchecked::<String, _>(4usize)?
                                                .into();
                                            ::std::result::Result::Ok(__EnhancedModel {
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
                                        <sqlx::mysql::MySql as ::sqlx::database::Database>::Arguments::<
                                            '_,
                                        >::default(),
                                    );
                                    ::sqlx::__query_with_result::<
                                        sqlx::mysql::MySql,
                                        _,
                                    >(
                                            "\n        SELECT\n            p.id,\n            p.name,\n            p.price,\n            p.stock,\n            p.category\n        FROM products p\n        WHERE 1 = 1\n        \n        \n        \n        \n         ORDER BY p.price DESC \n        \n        ",
                                            query_args,
                                        )
                                        .try_map(|row: sqlx::mysql::MySqlRow| {
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
                                                .try_get_unchecked::<i32, _>(3usize)?
                                                .into();
                                            #[allow(non_snake_case)]
                                            let sqlx_query_as_category = row
                                                .try_get_unchecked::<String, _>(4usize)?
                                                .into();
                                            ::std::result::Result::Ok(__EnhancedModel {
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
                        let __enhanced_section_case_single_2_filter_category_cat = &(category
                            .unwrap());
                        let __enhanced_section_case_single_2_filter_price_min_price_min = &(price_min
                            .unwrap());
                        let __enhanced_section_case_single_2_filter_price_max_price_max = &(price_max
                            .unwrap());
                        let __enhanced_section_case_single_2_limit_offset = &(page
                            * page_size.unwrap());
                        let __enhanced_section_case_single_2_limit_size = &(page_size
                            .unwrap());
                        let _ = {
                            {
                                #[allow(clippy::all)]
                                {
                                    use ::sqlx::Arguments as _;
                                    let arg0 = &(__enhanced_section_case_single_2_filter_category_cat);
                                    let arg1 = &(__enhanced_section_case_single_2_filter_price_min_price_min);
                                    let arg2 = &(__enhanced_section_case_single_2_filter_price_max_price_max);
                                    let arg3 = &(__enhanced_section_case_single_2_limit_size);
                                    let arg4 = &(__enhanced_section_case_single_2_limit_offset);
                                    let mut query_args = <sqlx::mysql::MySql as ::sqlx::database::Database>::Arguments::<
                                        '_,
                                    >::default();
                                    query_args
                                        .reserve(
                                            5usize,
                                            0
                                                + ::sqlx::encode::Encode::<
                                                    sqlx::mysql::MySql,
                                                >::size_hint(arg0)
                                                + ::sqlx::encode::Encode::<
                                                    sqlx::mysql::MySql,
                                                >::size_hint(arg1)
                                                + ::sqlx::encode::Encode::<
                                                    sqlx::mysql::MySql,
                                                >::size_hint(arg2)
                                                + ::sqlx::encode::Encode::<
                                                    sqlx::mysql::MySql,
                                                >::size_hint(arg3)
                                                + ::sqlx::encode::Encode::<
                                                    sqlx::mysql::MySql,
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
                                        sqlx::mysql::MySql,
                                        _,
                                    >(
                                            "\n        SELECT\n            p.id,\n            p.name,\n            p.price,\n            p.stock,\n            p.category\n        FROM products p\n        WHERE 1 = 1\n         AND p.category = ? \n         AND p.price >= ? \n         AND p.price <= ? \n         AND p.stock > 0 \n         ORDER BY p.id ASC \n         LIMIT ? OFFSET ? \n        ",
                                            query_args,
                                        )
                                        .try_map(|row: sqlx::mysql::MySqlRow| {
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
                                                .try_get_unchecked::<i32, _>(3usize)?
                                                .into();
                                            #[allow(non_snake_case)]
                                            let sqlx_query_as_category = row
                                                .try_get_unchecked::<String, _>(4usize)?
                                                .into();
                                            ::std::result::Result::Ok(__EnhancedModel {
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
            struct __EnhancedQuery_single<'args> {
                inner: sqlx::QueryBuilder<'args, sqlx::MySql>,
            }
            impl<'args> __EnhancedQuery_single<'args> {
                async fn fetch_all<'e, E>(
                    mut self,
                    executor: E,
                ) -> Result<Vec<Product>, sqlx::Error>
                where
                    E: sqlx::Executor<'e, Database = sqlx::MySql>,
                {
                    self.inner.build_query_as::<Product>().fetch_all(executor).await
                }
                async fn fetch_one<'e, E>(
                    mut self,
                    executor: E,
                ) -> Result<Product, sqlx::Error>
                where
                    E: sqlx::Executor<'e, Database = sqlx::MySql>,
                {
                    self.inner.build_query_as::<Product>().fetch_one(executor).await
                }
                async fn fetch_optional<'e, E>(
                    mut self,
                    executor: E,
                ) -> Result<Option<Product>, sqlx::Error>
                where
                    E: sqlx::Executor<'e, Database = sqlx::MySql>,
                {
                    self.inner.build_query_as::<Product>().fetch_optional(executor).await
                }
                async fn execute<'e, E>(
                    mut self,
                    executor: E,
                ) -> Result<<sqlx::MySql as sqlx::Database>::QueryResult, sqlx::Error>
                where
                    E: sqlx::Executor<'e, Database = sqlx::MySql>,
                {
                    self.inner.build().execute(executor).await
                }
            }
            impl<'args> sql_forge::EnhancedQuery<Product>
            for __EnhancedQuery_single<'args> {
                type Db = sqlx::MySql;
                fn fetch_all<'e, E>(
                    self,
                    executor: E,
                ) -> impl std::future::Future<
                    Output = Result<Vec<Product>, sqlx::Error>,
                > + Send + 'e
                where
                    Self: Sized + 'e,
                    E: sqlx::Executor<'e, Database = sqlx::MySql> + Send + 'e,
                    sqlx::MySql: 'e,
                {
                    __EnhancedQuery_single::fetch_all(self, executor)
                }
                fn fetch_one<'e, E>(
                    self,
                    executor: E,
                ) -> impl std::future::Future<
                    Output = Result<Product, sqlx::Error>,
                > + Send + 'e
                where
                    Self: Sized + 'e,
                    E: sqlx::Executor<'e, Database = sqlx::MySql> + Send + 'e,
                    sqlx::MySql: 'e,
                {
                    __EnhancedQuery_single::fetch_one(self, executor)
                }
                fn fetch_optional<'e, E>(
                    self,
                    executor: E,
                ) -> impl std::future::Future<
                    Output = Result<Option<Product>, sqlx::Error>,
                > + Send + 'e
                where
                    Self: Sized + 'e,
                    E: sqlx::Executor<'e, Database = sqlx::MySql> + Send + 'e,
                    sqlx::MySql: 'e,
                {
                    __EnhancedQuery_single::fetch_optional(self, executor)
                }
                fn execute<'e, E>(
                    self,
                    executor: E,
                ) -> impl std::future::Future<
                    Output = Result<
                        <sqlx::MySql as sqlx::Database>::QueryResult,
                        sqlx::Error,
                    >,
                > + Send + 'e
                where
                    Self: Sized + 'e,
                    E: sqlx::Executor<'e, Database = sqlx::MySql> + Send + 'e,
                    sqlx::MySql: 'e,
                {
                    __EnhancedQuery_single::execute(self, executor)
                }
            }
            let mut __builder: sqlx::QueryBuilder<sqlx::MySql> = sqlx::QueryBuilder::new(
                "",
            );
            __builder
                .push(
                    "\n        SELECT\n            p.id,\n            p.name,\n            p.price,\n            p.stock,\n            p.category\n        FROM products p\n        WHERE 1 = 1\n        ",
                );
            match category.is_some() {
                true => {
                    let __enhanced_section_filter_category_0_cat = category.unwrap();
                    __builder.push(" AND p.category = ");
                    __builder.push_bind(__enhanced_section_filter_category_0_cat);
                    __builder.push(" ");
                }
                false => {}
            }
            __builder.push("\n        ");
            match price_min.is_some() {
                true => {
                    let __enhanced_section_filter_price_min_0_price_min = price_min
                        .unwrap();
                    __builder.push(" AND p.price >= ");
                    __builder.push_bind(__enhanced_section_filter_price_min_0_price_min);
                    __builder.push(" ");
                }
                false => {}
            }
            __builder.push("\n        ");
            match price_max.is_some() {
                true => {
                    let __enhanced_section_filter_price_max_0_price_max = price_max
                        .unwrap();
                    __builder.push(" AND p.price <= ");
                    __builder.push_bind(__enhanced_section_filter_price_max_0_price_max);
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
                    let __enhanced_section_limit_0_offset = page * page_size.unwrap();
                    let __enhanced_section_limit_0_size = page_size.unwrap();
                    __builder.push(" LIMIT ");
                    __builder.push_bind(__enhanced_section_limit_0_size);
                    __builder.push(" OFFSET ");
                    __builder.push_bind(__enhanced_section_limit_0_offset);
                    __builder.push(" ");
                }
                false => {}
            }
            __builder.push("\n        ");
            let __sql_forge_value_single = __EnhancedQuery_single {
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
        start_line: 449usize,
        start_col: 10usize,
        end_line: 449usize,
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
                let __enhanced_top_level_id = &(1i64);
                {
                    {
                        let _ = {
                            #[allow(clippy::all)]
                            {
                                use ::sqlx::Arguments as _;
                                let arg0 = &(__enhanced_top_level_id);
                                let mut query_args = <sqlx::mysql::MySql as ::sqlx::database::Database>::Arguments::<
                                    '_,
                                >::default();
                                query_args
                                    .reserve(
                                        1usize,
                                        0
                                            + ::sqlx::encode::Encode::<
                                                sqlx::mysql::MySql,
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
                                    sqlx::mysql::MySql,
                                    _,
                                >("UPDATE products SET stock = 50 WHERE id = ?", query_args)
                            }
                        };
                    }
                }
            };
            struct __EnhancedQuery_single<'args> {
                inner: sqlx::QueryBuilder<'args, sqlx::MySql>,
            }
            impl<'args> __EnhancedQuery_single<'args> {
                async fn execute<'e, E>(
                    mut self,
                    executor: E,
                ) -> Result<<sqlx::MySql as sqlx::Database>::QueryResult, sqlx::Error>
                where
                    E: sqlx::Executor<'e, Database = sqlx::MySql>,
                {
                    self.inner.build().execute(executor).await
                }
            }
            impl<'args> sql_forge::EnhancedQueryExecute
            for __EnhancedQuery_single<'args> {
                type Db = sqlx::MySql;
                fn execute<'e, E>(
                    self,
                    executor: E,
                ) -> impl std::future::Future<
                    Output = Result<
                        <sqlx::MySql as sqlx::Database>::QueryResult,
                        sqlx::Error,
                    >,
                > + Send + 'e
                where
                    Self: Sized + 'e,
                    E: sqlx::Executor<'e, Database = sqlx::MySql> + Send + 'e,
                    sqlx::MySql: 'e,
                {
                    __EnhancedQuery_single::execute(self, executor)
                }
            }
            let __enhanced_runtime_id = 1i64;
            let mut __builder: sqlx::QueryBuilder<sqlx::MySql> = sqlx::QueryBuilder::new(
                "",
            );
            __builder.push("UPDATE products SET stock = 50 WHERE id = ");
            __builder.push_bind(__enhanced_runtime_id);
            let __sql_forge_value_single = __EnhancedQuery_single {
                inner: __builder,
            };
            __sql_forge_value_single
        }
            .execute(&pool)
            .await
            .expect("reset stock failed");
        {
            let _sql_forge_validator = || {
                let __enhanced_top_level_id = &(1i64);
                {
                    {
                        let _ = {
                            #[allow(clippy::all)]
                            {
                                use ::sqlx::Arguments as _;
                                let arg0 = &(__enhanced_top_level_id);
                                let mut query_args = <sqlx::mysql::MySql as ::sqlx::database::Database>::Arguments::<
                                    '_,
                                >::default();
                                query_args
                                    .reserve(
                                        1usize,
                                        0
                                            + ::sqlx::encode::Encode::<
                                                sqlx::mysql::MySql,
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
                                    sqlx::mysql::MySql,
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
            struct __EnhancedQuery_single<'args> {
                inner: sqlx::QueryBuilder<'args, sqlx::MySql>,
            }
            impl<'args> __EnhancedQuery_single<'args> {
                async fn execute<'e, E>(
                    mut self,
                    executor: E,
                ) -> Result<<sqlx::MySql as sqlx::Database>::QueryResult, sqlx::Error>
                where
                    E: sqlx::Executor<'e, Database = sqlx::MySql>,
                {
                    self.inner.build().execute(executor).await
                }
            }
            impl<'args> sql_forge::EnhancedQueryExecute
            for __EnhancedQuery_single<'args> {
                type Db = sqlx::MySql;
                fn execute<'e, E>(
                    self,
                    executor: E,
                ) -> impl std::future::Future<
                    Output = Result<
                        <sqlx::MySql as sqlx::Database>::QueryResult,
                        sqlx::Error,
                    >,
                > + Send + 'e
                where
                    Self: Sized + 'e,
                    E: sqlx::Executor<'e, Database = sqlx::MySql> + Send + 'e,
                    sqlx::MySql: 'e,
                {
                    __EnhancedQuery_single::execute(self, executor)
                }
            }
            let __enhanced_runtime_id = 1i64;
            let mut __builder: sqlx::QueryBuilder<sqlx::MySql> = sqlx::QueryBuilder::new(
                "",
            );
            __builder
                .push("\n        UPDATE products SET stock = stock + 1 WHERE id = ");
            __builder.push_bind(__enhanced_runtime_id);
            __builder.push("\n        ");
            let __sql_forge_value_single = __EnhancedQuery_single {
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
        start_line: 481usize,
        start_col: 10usize,
        end_line: 481usize,
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
                let __enhanced_top_level_category = &("Temporary");
                {
                    {
                        let _ = {
                            #[allow(clippy::all)]
                            {
                                use ::sqlx::Arguments as _;
                                let arg0 = &(__enhanced_top_level_category);
                                let mut query_args = <sqlx::mysql::MySql as ::sqlx::database::Database>::Arguments::<
                                    '_,
                                >::default();
                                query_args
                                    .reserve(
                                        1usize,
                                        0
                                            + ::sqlx::encode::Encode::<
                                                sqlx::mysql::MySql,
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
                                    sqlx::mysql::MySql,
                                    _,
                                >("DELETE FROM products WHERE category = ?", query_args)
                            }
                        };
                    }
                }
            };
            struct __EnhancedQuery_single<'args> {
                inner: sqlx::QueryBuilder<'args, sqlx::MySql>,
            }
            impl<'args> __EnhancedQuery_single<'args> {
                async fn execute<'e, E>(
                    mut self,
                    executor: E,
                ) -> Result<<sqlx::MySql as sqlx::Database>::QueryResult, sqlx::Error>
                where
                    E: sqlx::Executor<'e, Database = sqlx::MySql>,
                {
                    self.inner.build().execute(executor).await
                }
            }
            impl<'args> sql_forge::EnhancedQueryExecute
            for __EnhancedQuery_single<'args> {
                type Db = sqlx::MySql;
                fn execute<'e, E>(
                    self,
                    executor: E,
                ) -> impl std::future::Future<
                    Output = Result<
                        <sqlx::MySql as sqlx::Database>::QueryResult,
                        sqlx::Error,
                    >,
                > + Send + 'e
                where
                    Self: Sized + 'e,
                    E: sqlx::Executor<'e, Database = sqlx::MySql> + Send + 'e,
                    sqlx::MySql: 'e,
                {
                    __EnhancedQuery_single::execute(self, executor)
                }
            }
            let __enhanced_runtime_category = "Temporary";
            let mut __builder: sqlx::QueryBuilder<sqlx::MySql> = sqlx::QueryBuilder::new(
                "",
            );
            __builder.push("DELETE FROM products WHERE category = ");
            __builder.push_bind(__enhanced_runtime_category);
            let __sql_forge_value_single = __EnhancedQuery_single {
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
                    let __enhanced_top_level_name = &(name);
                    let __enhanced_top_level_price = &(price_inc(
                        &base_price,
                        i as i64,
                        2,
                    ));
                    let __enhanced_top_level_stock = &(10i64);
                    let __enhanced_top_level_category = &("Temporary");
                    {
                        {
                            let _ = {
                                #[allow(clippy::all)]
                                {
                                    use ::sqlx::Arguments as _;
                                    let arg0 = &(__enhanced_top_level_name);
                                    let arg1 = &(__enhanced_top_level_price);
                                    let arg2 = &(__enhanced_top_level_stock);
                                    let arg3 = &(__enhanced_top_level_category);
                                    let mut query_args = <sqlx::mysql::MySql as ::sqlx::database::Database>::Arguments::<
                                        '_,
                                    >::default();
                                    query_args
                                        .reserve(
                                            4usize,
                                            0
                                                + ::sqlx::encode::Encode::<
                                                    sqlx::mysql::MySql,
                                                >::size_hint(arg0)
                                                + ::sqlx::encode::Encode::<
                                                    sqlx::mysql::MySql,
                                                >::size_hint(arg1)
                                                + ::sqlx::encode::Encode::<
                                                    sqlx::mysql::MySql,
                                                >::size_hint(arg2)
                                                + ::sqlx::encode::Encode::<
                                                    sqlx::mysql::MySql,
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
                                        sqlx::mysql::MySql,
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
                struct __EnhancedQuery_single<'args> {
                    inner: sqlx::QueryBuilder<'args, sqlx::MySql>,
                }
                impl<'args> __EnhancedQuery_single<'args> {
                    async fn execute<'e, E>(
                        mut self,
                        executor: E,
                    ) -> Result<
                        <sqlx::MySql as sqlx::Database>::QueryResult,
                        sqlx::Error,
                    >
                    where
                        E: sqlx::Executor<'e, Database = sqlx::MySql>,
                    {
                        self.inner.build().execute(executor).await
                    }
                }
                impl<'args> sql_forge::EnhancedQueryExecute
                for __EnhancedQuery_single<'args> {
                    type Db = sqlx::MySql;
                    fn execute<'e, E>(
                        self,
                        executor: E,
                    ) -> impl std::future::Future<
                        Output = Result<
                            <sqlx::MySql as sqlx::Database>::QueryResult,
                            sqlx::Error,
                        >,
                    > + Send + 'e
                    where
                        Self: Sized + 'e,
                        E: sqlx::Executor<'e, Database = sqlx::MySql> + Send + 'e,
                        sqlx::MySql: 'e,
                    {
                        __EnhancedQuery_single::execute(self, executor)
                    }
                }
                let __enhanced_runtime_name = name;
                let __enhanced_runtime_price = price_inc(&base_price, i as i64, 2);
                let __enhanced_runtime_stock = 10i64;
                let __enhanced_runtime_category = "Temporary";
                let mut __builder: sqlx::QueryBuilder<sqlx::MySql> = sqlx::QueryBuilder::new(
                    "",
                );
                __builder
                    .push(
                        "\n            INSERT INTO products (name, price, stock, category)\n            VALUES (",
                    );
                __builder.push_bind(__enhanced_runtime_name);
                __builder.push(", ");
                __builder.push_bind(__enhanced_runtime_price);
                __builder.push(", ");
                __builder.push_bind(__enhanced_runtime_stock);
                __builder.push(", ");
                __builder.push_bind(__enhanced_runtime_category);
                __builder.push(")\n            ");
                let __sql_forge_value_single = __EnhancedQuery_single {
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
                let __enhanced_top_level_new_price = &(price_new(4999, 2));
                let __enhanced_top_level_category = &("Temporary");
                let __enhanced_top_level_name = &("Temp B");
                {
                    {
                        let _ = {
                            #[allow(clippy::all)]
                            {
                                use ::sqlx::Arguments as _;
                                let arg0 = &(__enhanced_top_level_new_price);
                                let arg1 = &(__enhanced_top_level_category);
                                let arg2 = &(__enhanced_top_level_name);
                                let mut query_args = <sqlx::mysql::MySql as ::sqlx::database::Database>::Arguments::<
                                    '_,
                                >::default();
                                query_args
                                    .reserve(
                                        3usize,
                                        0
                                            + ::sqlx::encode::Encode::<
                                                sqlx::mysql::MySql,
                                            >::size_hint(arg0)
                                            + ::sqlx::encode::Encode::<
                                                sqlx::mysql::MySql,
                                            >::size_hint(arg1)
                                            + ::sqlx::encode::Encode::<
                                                sqlx::mysql::MySql,
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
                                    sqlx::mysql::MySql,
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
            struct __EnhancedQuery_single<'args> {
                inner: sqlx::QueryBuilder<'args, sqlx::MySql>,
            }
            impl<'args> __EnhancedQuery_single<'args> {
                async fn execute<'e, E>(
                    mut self,
                    executor: E,
                ) -> Result<<sqlx::MySql as sqlx::Database>::QueryResult, sqlx::Error>
                where
                    E: sqlx::Executor<'e, Database = sqlx::MySql>,
                {
                    self.inner.build().execute(executor).await
                }
            }
            impl<'args> sql_forge::EnhancedQueryExecute
            for __EnhancedQuery_single<'args> {
                type Db = sqlx::MySql;
                fn execute<'e, E>(
                    self,
                    executor: E,
                ) -> impl std::future::Future<
                    Output = Result<
                        <sqlx::MySql as sqlx::Database>::QueryResult,
                        sqlx::Error,
                    >,
                > + Send + 'e
                where
                    Self: Sized + 'e,
                    E: sqlx::Executor<'e, Database = sqlx::MySql> + Send + 'e,
                    sqlx::MySql: 'e,
                {
                    __EnhancedQuery_single::execute(self, executor)
                }
            }
            let __enhanced_runtime_new_price = price_new(4999, 2);
            let __enhanced_runtime_category = "Temporary";
            let __enhanced_runtime_name = "Temp B";
            let mut __builder: sqlx::QueryBuilder<sqlx::MySql> = sqlx::QueryBuilder::new(
                "",
            );
            __builder.push("\n        UPDATE products\n        SET price = ");
            __builder.push_bind(__enhanced_runtime_new_price);
            __builder.push("\n        WHERE category = ");
            __builder.push_bind(__enhanced_runtime_category);
            __builder.push(" AND name = ");
            __builder.push_bind(__enhanced_runtime_name);
            __builder.push("\n        ");
            let __sql_forge_value_single = __EnhancedQuery_single {
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
                let __enhanced_top_level_cat = &("Temporary");
                {
                    type __EnhancedModel = TempRow;
                    {
                        let _ = {
                            {
                                #[allow(clippy::all)]
                                {
                                    use ::sqlx::Arguments as _;
                                    let arg0 = &(__enhanced_top_level_cat);
                                    let mut query_args = <sqlx::mysql::MySql as ::sqlx::database::Database>::Arguments::<
                                        '_,
                                    >::default();
                                    query_args
                                        .reserve(
                                            1usize,
                                            0
                                                + ::sqlx::encode::Encode::<
                                                    sqlx::mysql::MySql,
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
                                        sqlx::mysql::MySql,
                                        _,
                                    >(
                                            "\n        SELECT name, price FROM products\n        WHERE category = ?\n        ORDER BY id\n        ",
                                            query_args,
                                        )
                                        .try_map(|row: sqlx::mysql::MySqlRow| {
                                            use ::sqlx::Row as _;
                                            #[allow(non_snake_case)]
                                            let sqlx_query_as_name = row
                                                .try_get_unchecked::<String, _>(0usize)?
                                                .into();
                                            #[allow(non_snake_case)]
                                            let sqlx_query_as_price = row
                                                .try_get_unchecked::<i64, _>(1usize)?
                                                .into();
                                            ::std::result::Result::Ok(__EnhancedModel {
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
            struct __EnhancedQuery_single<'args> {
                inner: sqlx::QueryBuilder<'args, sqlx::MySql>,
            }
            impl<'args> __EnhancedQuery_single<'args> {
                async fn fetch_all<'e, E>(
                    mut self,
                    executor: E,
                ) -> Result<Vec<TempRow>, sqlx::Error>
                where
                    E: sqlx::Executor<'e, Database = sqlx::MySql>,
                {
                    self.inner.build_query_as::<TempRow>().fetch_all(executor).await
                }
                async fn fetch_one<'e, E>(
                    mut self,
                    executor: E,
                ) -> Result<TempRow, sqlx::Error>
                where
                    E: sqlx::Executor<'e, Database = sqlx::MySql>,
                {
                    self.inner.build_query_as::<TempRow>().fetch_one(executor).await
                }
                async fn fetch_optional<'e, E>(
                    mut self,
                    executor: E,
                ) -> Result<Option<TempRow>, sqlx::Error>
                where
                    E: sqlx::Executor<'e, Database = sqlx::MySql>,
                {
                    self.inner.build_query_as::<TempRow>().fetch_optional(executor).await
                }
                async fn execute<'e, E>(
                    mut self,
                    executor: E,
                ) -> Result<<sqlx::MySql as sqlx::Database>::QueryResult, sqlx::Error>
                where
                    E: sqlx::Executor<'e, Database = sqlx::MySql>,
                {
                    self.inner.build().execute(executor).await
                }
            }
            impl<'args> sql_forge::EnhancedQuery<TempRow>
            for __EnhancedQuery_single<'args> {
                type Db = sqlx::MySql;
                fn fetch_all<'e, E>(
                    self,
                    executor: E,
                ) -> impl std::future::Future<
                    Output = Result<Vec<TempRow>, sqlx::Error>,
                > + Send + 'e
                where
                    Self: Sized + 'e,
                    E: sqlx::Executor<'e, Database = sqlx::MySql> + Send + 'e,
                    sqlx::MySql: 'e,
                {
                    __EnhancedQuery_single::fetch_all(self, executor)
                }
                fn fetch_one<'e, E>(
                    self,
                    executor: E,
                ) -> impl std::future::Future<
                    Output = Result<TempRow, sqlx::Error>,
                > + Send + 'e
                where
                    Self: Sized + 'e,
                    E: sqlx::Executor<'e, Database = sqlx::MySql> + Send + 'e,
                    sqlx::MySql: 'e,
                {
                    __EnhancedQuery_single::fetch_one(self, executor)
                }
                fn fetch_optional<'e, E>(
                    self,
                    executor: E,
                ) -> impl std::future::Future<
                    Output = Result<Option<TempRow>, sqlx::Error>,
                > + Send + 'e
                where
                    Self: Sized + 'e,
                    E: sqlx::Executor<'e, Database = sqlx::MySql> + Send + 'e,
                    sqlx::MySql: 'e,
                {
                    __EnhancedQuery_single::fetch_optional(self, executor)
                }
                fn execute<'e, E>(
                    self,
                    executor: E,
                ) -> impl std::future::Future<
                    Output = Result<
                        <sqlx::MySql as sqlx::Database>::QueryResult,
                        sqlx::Error,
                    >,
                > + Send + 'e
                where
                    Self: Sized + 'e,
                    E: sqlx::Executor<'e, Database = sqlx::MySql> + Send + 'e,
                    sqlx::MySql: 'e,
                {
                    __EnhancedQuery_single::execute(self, executor)
                }
            }
            let __enhanced_runtime_cat = "Temporary";
            let mut __builder: sqlx::QueryBuilder<sqlx::MySql> = sqlx::QueryBuilder::new(
                "",
            );
            __builder
                .push(
                    "\n        SELECT name, price FROM products\n        WHERE category = ",
                );
            __builder.push_bind(__enhanced_runtime_cat);
            __builder.push("\n        ORDER BY id\n        ");
            let __sql_forge_value_single = __EnhancedQuery_single {
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
                let __enhanced_top_level_category = &("Temporary");
                {
                    {
                        let _ = {
                            #[allow(clippy::all)]
                            {
                                use ::sqlx::Arguments as _;
                                let arg0 = &(__enhanced_top_level_category);
                                let mut query_args = <sqlx::mysql::MySql as ::sqlx::database::Database>::Arguments::<
                                    '_,
                                >::default();
                                query_args
                                    .reserve(
                                        1usize,
                                        0
                                            + ::sqlx::encode::Encode::<
                                                sqlx::mysql::MySql,
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
                                    sqlx::mysql::MySql,
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
            struct __EnhancedQuery_single<'args> {
                inner: sqlx::QueryBuilder<'args, sqlx::MySql>,
            }
            impl<'args> __EnhancedQuery_single<'args> {
                async fn execute<'e, E>(
                    mut self,
                    executor: E,
                ) -> Result<<sqlx::MySql as sqlx::Database>::QueryResult, sqlx::Error>
                where
                    E: sqlx::Executor<'e, Database = sqlx::MySql>,
                {
                    self.inner.build().execute(executor).await
                }
            }
            impl<'args> sql_forge::EnhancedQueryExecute
            for __EnhancedQuery_single<'args> {
                type Db = sqlx::MySql;
                fn execute<'e, E>(
                    self,
                    executor: E,
                ) -> impl std::future::Future<
                    Output = Result<
                        <sqlx::MySql as sqlx::Database>::QueryResult,
                        sqlx::Error,
                    >,
                > + Send + 'e
                where
                    Self: Sized + 'e,
                    E: sqlx::Executor<'e, Database = sqlx::MySql> + Send + 'e,
                    sqlx::MySql: 'e,
                {
                    __EnhancedQuery_single::execute(self, executor)
                }
            }
            let __enhanced_runtime_category = "Temporary";
            let mut __builder: sqlx::QueryBuilder<sqlx::MySql> = sqlx::QueryBuilder::new(
                "",
            );
            __builder.push("\n        DELETE FROM products\n        WHERE category = ");
            __builder.push_bind(__enhanced_runtime_category);
            __builder.push("\n        ");
            let __sql_forge_value_single = __EnhancedQuery_single {
                inner: __builder,
            };
            __sql_forge_value_single
        }
            .execute(&pool)
            .await
            .expect("delete failed");
        let remaining: i64 = {
            let _sql_forge_validator = || {
                let __enhanced_top_level_cat = &("Temporary");
                {
                    {
                        let _ = {
                            #[allow(clippy::all)]
                            {
                                use ::sqlx::Arguments as _;
                                let arg0 = &(__enhanced_top_level_cat);
                                let mut query_args = <sqlx::mysql::MySql as ::sqlx::database::Database>::Arguments::<
                                    '_,
                                >::default();
                                query_args
                                    .reserve(
                                        1usize,
                                        0
                                            + ::sqlx::encode::Encode::<
                                                sqlx::mysql::MySql,
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
                                    sqlx::mysql::MySql,
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
            struct __EnhancedQuery_single<'args> {
                inner: sqlx::QueryBuilder<'args, sqlx::MySql>,
            }
            impl<'args> __EnhancedQuery_single<'args> {
                async fn fetch_all<'e, E>(
                    mut self,
                    executor: E,
                ) -> Result<Vec<i64>, sqlx::Error>
                where
                    E: sqlx::Executor<'e, Database = sqlx::MySql>,
                {
                    self.inner.build_query_scalar::<i64>().fetch_all(executor).await
                }
                async fn fetch_one<'e, E>(
                    mut self,
                    executor: E,
                ) -> Result<i64, sqlx::Error>
                where
                    E: sqlx::Executor<'e, Database = sqlx::MySql>,
                {
                    self.inner.build_query_scalar::<i64>().fetch_one(executor).await
                }
                async fn fetch_optional<'e, E>(
                    mut self,
                    executor: E,
                ) -> Result<Option<i64>, sqlx::Error>
                where
                    E: sqlx::Executor<'e, Database = sqlx::MySql>,
                {
                    self.inner.build_query_scalar::<i64>().fetch_optional(executor).await
                }
                async fn execute<'e, E>(
                    mut self,
                    executor: E,
                ) -> Result<<sqlx::MySql as sqlx::Database>::QueryResult, sqlx::Error>
                where
                    E: sqlx::Executor<'e, Database = sqlx::MySql>,
                {
                    self.inner.build().execute(executor).await
                }
            }
            impl<'args> sql_forge::EnhancedQuery<i64> for __EnhancedQuery_single<'args> {
                type Db = sqlx::MySql;
                fn fetch_all<'e, E>(
                    self,
                    executor: E,
                ) -> impl std::future::Future<
                    Output = Result<Vec<i64>, sqlx::Error>,
                > + Send + 'e
                where
                    Self: Sized + 'e,
                    E: sqlx::Executor<'e, Database = sqlx::MySql> + Send + 'e,
                    sqlx::MySql: 'e,
                {
                    __EnhancedQuery_single::fetch_all(self, executor)
                }
                fn fetch_one<'e, E>(
                    self,
                    executor: E,
                ) -> impl std::future::Future<
                    Output = Result<i64, sqlx::Error>,
                > + Send + 'e
                where
                    Self: Sized + 'e,
                    E: sqlx::Executor<'e, Database = sqlx::MySql> + Send + 'e,
                    sqlx::MySql: 'e,
                {
                    __EnhancedQuery_single::fetch_one(self, executor)
                }
                fn fetch_optional<'e, E>(
                    self,
                    executor: E,
                ) -> impl std::future::Future<
                    Output = Result<Option<i64>, sqlx::Error>,
                > + Send + 'e
                where
                    Self: Sized + 'e,
                    E: sqlx::Executor<'e, Database = sqlx::MySql> + Send + 'e,
                    sqlx::MySql: 'e,
                {
                    __EnhancedQuery_single::fetch_optional(self, executor)
                }
                fn execute<'e, E>(
                    self,
                    executor: E,
                ) -> impl std::future::Future<
                    Output = Result<
                        <sqlx::MySql as sqlx::Database>::QueryResult,
                        sqlx::Error,
                    >,
                > + Send + 'e
                where
                    Self: Sized + 'e,
                    E: sqlx::Executor<'e, Database = sqlx::MySql> + Send + 'e,
                    sqlx::MySql: 'e,
                {
                    __EnhancedQuery_single::execute(self, executor)
                }
            }
            let __enhanced_runtime_cat = "Temporary";
            let mut __builder: sqlx::QueryBuilder<sqlx::MySql> = sqlx::QueryBuilder::new(
                "",
            );
            __builder.push("SELECT COUNT(*) FROM products WHERE category = ");
            __builder.push_bind(__enhanced_runtime_cat);
            let __sql_forge_value_single = __EnhancedQuery_single {
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
        start_line: 587usize,
        start_col: 10usize,
        end_line: 587usize,
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
                let __enhanced_top_level_category = &("Batch");
                {
                    {
                        let _ = {
                            #[allow(clippy::all)]
                            {
                                use ::sqlx::Arguments as _;
                                let arg0 = &(__enhanced_top_level_category);
                                let mut query_args = <sqlx::mysql::MySql as ::sqlx::database::Database>::Arguments::<
                                    '_,
                                >::default();
                                query_args
                                    .reserve(
                                        1usize,
                                        0
                                            + ::sqlx::encode::Encode::<
                                                sqlx::mysql::MySql,
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
                                    sqlx::mysql::MySql,
                                    _,
                                >("DELETE FROM products WHERE category = ?", query_args)
                            }
                        };
                    }
                }
            };
            struct __EnhancedQuery_single<'args> {
                inner: sqlx::QueryBuilder<'args, sqlx::MySql>,
            }
            impl<'args> __EnhancedQuery_single<'args> {
                async fn execute<'e, E>(
                    mut self,
                    executor: E,
                ) -> Result<<sqlx::MySql as sqlx::Database>::QueryResult, sqlx::Error>
                where
                    E: sqlx::Executor<'e, Database = sqlx::MySql>,
                {
                    self.inner.build().execute(executor).await
                }
            }
            impl<'args> sql_forge::EnhancedQueryExecute
            for __EnhancedQuery_single<'args> {
                type Db = sqlx::MySql;
                fn execute<'e, E>(
                    self,
                    executor: E,
                ) -> impl std::future::Future<
                    Output = Result<
                        <sqlx::MySql as sqlx::Database>::QueryResult,
                        sqlx::Error,
                    >,
                > + Send + 'e
                where
                    Self: Sized + 'e,
                    E: sqlx::Executor<'e, Database = sqlx::MySql> + Send + 'e,
                    sqlx::MySql: 'e,
                {
                    __EnhancedQuery_single::execute(self, executor)
                }
            }
            let __enhanced_runtime_category = "Batch";
            let mut __builder: sqlx::QueryBuilder<sqlx::MySql> = sqlx::QueryBuilder::new(
                "",
            );
            __builder.push("DELETE FROM products WHERE category = ");
            __builder.push_bind(__enhanced_runtime_category);
            let __sql_forge_value_single = __EnhancedQuery_single {
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
                                let mut query_args = <sqlx::mysql::MySql as ::sqlx::database::Database>::Arguments::<
                                    '_,
                                >::default();
                                query_args
                                    .reserve(
                                        6usize,
                                        0
                                            + ::sqlx::encode::Encode::<
                                                sqlx::mysql::MySql,
                                            >::size_hint(arg0)
                                            + ::sqlx::encode::Encode::<
                                                sqlx::mysql::MySql,
                                            >::size_hint(arg1)
                                            + ::sqlx::encode::Encode::<
                                                sqlx::mysql::MySql,
                                            >::size_hint(arg2)
                                            + ::sqlx::encode::Encode::<
                                                sqlx::mysql::MySql,
                                            >::size_hint(arg3)
                                            + ::sqlx::encode::Encode::<
                                                sqlx::mysql::MySql,
                                            >::size_hint(arg4)
                                            + ::sqlx::encode::Encode::<
                                                sqlx::mysql::MySql,
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
                                    sqlx::mysql::MySql,
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
            struct __EnhancedQuery_single<'args> {
                inner: sqlx::QueryBuilder<'args, sqlx::MySql>,
            }
            impl<'args> __EnhancedQuery_single<'args> {
                async fn execute<'e, E>(
                    mut self,
                    executor: E,
                ) -> Result<<sqlx::MySql as sqlx::Database>::QueryResult, sqlx::Error>
                where
                    E: sqlx::Executor<'e, Database = sqlx::MySql>,
                {
                    self.inner.build().execute(executor).await
                }
            }
            impl<'args> sql_forge::EnhancedQueryExecute
            for __EnhancedQuery_single<'args> {
                type Db = sqlx::MySql;
                fn execute<'e, E>(
                    self,
                    executor: E,
                ) -> impl std::future::Future<
                    Output = Result<
                        <sqlx::MySql as sqlx::Database>::QueryResult,
                        sqlx::Error,
                    >,
                > + Send + 'e
                where
                    Self: Sized + 'e,
                    E: sqlx::Executor<'e, Database = sqlx::MySql> + Send + 'e,
                    sqlx::MySql: 'e,
                {
                    __EnhancedQuery_single::execute(self, executor)
                }
            }
            let mut __builder: sqlx::QueryBuilder<sqlx::MySql> = sqlx::QueryBuilder::new(
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
            let __sql_forge_value_single = __EnhancedQuery_single {
                inner: __builder,
            };
            __sql_forge_value_single
        }
            .execute(&pool)
            .await
            .expect("batch insert failed");
        let rows: Vec<BatchItem> = {
            let _sql_forge_validator = || {
                let __enhanced_top_level_cat = &("Batch");
                {
                    type __EnhancedModel = BatchItem;
                    {
                        let _ = {
                            {
                                #[allow(clippy::all)]
                                {
                                    use ::sqlx::Arguments as _;
                                    let arg0 = &(__enhanced_top_level_cat);
                                    let mut query_args = <sqlx::mysql::MySql as ::sqlx::database::Database>::Arguments::<
                                        '_,
                                    >::default();
                                    query_args
                                        .reserve(
                                            1usize,
                                            0
                                                + ::sqlx::encode::Encode::<
                                                    sqlx::mysql::MySql,
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
                                        sqlx::mysql::MySql,
                                        _,
                                    >(
                                            "\n        SELECT name, price FROM products\n        WHERE category = ?\n        ORDER BY id\n        ",
                                            query_args,
                                        )
                                        .try_map(|row: sqlx::mysql::MySqlRow| {
                                            use ::sqlx::Row as _;
                                            #[allow(non_snake_case)]
                                            let sqlx_query_as_name = row
                                                .try_get_unchecked::<String, _>(0usize)?
                                                .into();
                                            #[allow(non_snake_case)]
                                            let sqlx_query_as_price = row
                                                .try_get_unchecked::<i64, _>(1usize)?
                                                .into();
                                            ::std::result::Result::Ok(__EnhancedModel {
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
            struct __EnhancedQuery_single<'args> {
                inner: sqlx::QueryBuilder<'args, sqlx::MySql>,
            }
            impl<'args> __EnhancedQuery_single<'args> {
                async fn fetch_all<'e, E>(
                    mut self,
                    executor: E,
                ) -> Result<Vec<BatchItem>, sqlx::Error>
                where
                    E: sqlx::Executor<'e, Database = sqlx::MySql>,
                {
                    self.inner.build_query_as::<BatchItem>().fetch_all(executor).await
                }
                async fn fetch_one<'e, E>(
                    mut self,
                    executor: E,
                ) -> Result<BatchItem, sqlx::Error>
                where
                    E: sqlx::Executor<'e, Database = sqlx::MySql>,
                {
                    self.inner.build_query_as::<BatchItem>().fetch_one(executor).await
                }
                async fn fetch_optional<'e, E>(
                    mut self,
                    executor: E,
                ) -> Result<Option<BatchItem>, sqlx::Error>
                where
                    E: sqlx::Executor<'e, Database = sqlx::MySql>,
                {
                    self.inner
                        .build_query_as::<BatchItem>()
                        .fetch_optional(executor)
                        .await
                }
                async fn execute<'e, E>(
                    mut self,
                    executor: E,
                ) -> Result<<sqlx::MySql as sqlx::Database>::QueryResult, sqlx::Error>
                where
                    E: sqlx::Executor<'e, Database = sqlx::MySql>,
                {
                    self.inner.build().execute(executor).await
                }
            }
            impl<'args> sql_forge::EnhancedQuery<BatchItem>
            for __EnhancedQuery_single<'args> {
                type Db = sqlx::MySql;
                fn fetch_all<'e, E>(
                    self,
                    executor: E,
                ) -> impl std::future::Future<
                    Output = Result<Vec<BatchItem>, sqlx::Error>,
                > + Send + 'e
                where
                    Self: Sized + 'e,
                    E: sqlx::Executor<'e, Database = sqlx::MySql> + Send + 'e,
                    sqlx::MySql: 'e,
                {
                    __EnhancedQuery_single::fetch_all(self, executor)
                }
                fn fetch_one<'e, E>(
                    self,
                    executor: E,
                ) -> impl std::future::Future<
                    Output = Result<BatchItem, sqlx::Error>,
                > + Send + 'e
                where
                    Self: Sized + 'e,
                    E: sqlx::Executor<'e, Database = sqlx::MySql> + Send + 'e,
                    sqlx::MySql: 'e,
                {
                    __EnhancedQuery_single::fetch_one(self, executor)
                }
                fn fetch_optional<'e, E>(
                    self,
                    executor: E,
                ) -> impl std::future::Future<
                    Output = Result<Option<BatchItem>, sqlx::Error>,
                > + Send + 'e
                where
                    Self: Sized + 'e,
                    E: sqlx::Executor<'e, Database = sqlx::MySql> + Send + 'e,
                    sqlx::MySql: 'e,
                {
                    __EnhancedQuery_single::fetch_optional(self, executor)
                }
                fn execute<'e, E>(
                    self,
                    executor: E,
                ) -> impl std::future::Future<
                    Output = Result<
                        <sqlx::MySql as sqlx::Database>::QueryResult,
                        sqlx::Error,
                    >,
                > + Send + 'e
                where
                    Self: Sized + 'e,
                    E: sqlx::Executor<'e, Database = sqlx::MySql> + Send + 'e,
                    sqlx::MySql: 'e,
                {
                    __EnhancedQuery_single::execute(self, executor)
                }
            }
            let __enhanced_runtime_cat = "Batch";
            let mut __builder: sqlx::QueryBuilder<sqlx::MySql> = sqlx::QueryBuilder::new(
                "",
            );
            __builder
                .push(
                    "\n        SELECT name, price FROM products\n        WHERE category = ",
                );
            __builder.push_bind(__enhanced_runtime_cat);
            __builder.push("\n        ORDER BY id\n        ");
            let __sql_forge_value_single = __EnhancedQuery_single {
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
                let __enhanced_top_level_category = &("Batch");
                {
                    {
                        let _ = {
                            #[allow(clippy::all)]
                            {
                                use ::sqlx::Arguments as _;
                                let arg0 = &(__enhanced_top_level_category);
                                let mut query_args = <sqlx::mysql::MySql as ::sqlx::database::Database>::Arguments::<
                                    '_,
                                >::default();
                                query_args
                                    .reserve(
                                        1usize,
                                        0
                                            + ::sqlx::encode::Encode::<
                                                sqlx::mysql::MySql,
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
                                    sqlx::mysql::MySql,
                                    _,
                                >("DELETE FROM products WHERE category = ?", query_args)
                            }
                        };
                    }
                }
            };
            struct __EnhancedQuery_single<'args> {
                inner: sqlx::QueryBuilder<'args, sqlx::MySql>,
            }
            impl<'args> __EnhancedQuery_single<'args> {
                async fn execute<'e, E>(
                    mut self,
                    executor: E,
                ) -> Result<<sqlx::MySql as sqlx::Database>::QueryResult, sqlx::Error>
                where
                    E: sqlx::Executor<'e, Database = sqlx::MySql>,
                {
                    self.inner.build().execute(executor).await
                }
            }
            impl<'args> sql_forge::EnhancedQueryExecute
            for __EnhancedQuery_single<'args> {
                type Db = sqlx::MySql;
                fn execute<'e, E>(
                    self,
                    executor: E,
                ) -> impl std::future::Future<
                    Output = Result<
                        <sqlx::MySql as sqlx::Database>::QueryResult,
                        sqlx::Error,
                    >,
                > + Send + 'e
                where
                    Self: Sized + 'e,
                    E: sqlx::Executor<'e, Database = sqlx::MySql> + Send + 'e,
                    sqlx::MySql: 'e,
                {
                    __EnhancedQuery_single::execute(self, executor)
                }
            }
            let __enhanced_runtime_category = "Batch";
            let mut __builder: sqlx::QueryBuilder<sqlx::MySql> = sqlx::QueryBuilder::new(
                "",
            );
            __builder.push("DELETE FROM products WHERE category = ");
            __builder.push_bind(__enhanced_runtime_category);
            let __sql_forge_value_single = __EnhancedQuery_single {
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
        start_line: 663usize,
        start_col: 10usize,
        end_line: 663usize,
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
                let __enhanced_top_level_category = &("BatchFull");
                {
                    {
                        let _ = {
                            #[allow(clippy::all)]
                            {
                                use ::sqlx::Arguments as _;
                                let arg0 = &(__enhanced_top_level_category);
                                let mut query_args = <sqlx::mysql::MySql as ::sqlx::database::Database>::Arguments::<
                                    '_,
                                >::default();
                                query_args
                                    .reserve(
                                        1usize,
                                        0
                                            + ::sqlx::encode::Encode::<
                                                sqlx::mysql::MySql,
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
                                    sqlx::mysql::MySql,
                                    _,
                                >("DELETE FROM products WHERE category = ?", query_args)
                            }
                        };
                    }
                }
            };
            struct __EnhancedQuery_single<'args> {
                inner: sqlx::QueryBuilder<'args, sqlx::MySql>,
            }
            impl<'args> __EnhancedQuery_single<'args> {
                async fn execute<'e, E>(
                    mut self,
                    executor: E,
                ) -> Result<<sqlx::MySql as sqlx::Database>::QueryResult, sqlx::Error>
                where
                    E: sqlx::Executor<'e, Database = sqlx::MySql>,
                {
                    self.inner.build().execute(executor).await
                }
            }
            impl<'args> sql_forge::EnhancedQueryExecute
            for __EnhancedQuery_single<'args> {
                type Db = sqlx::MySql;
                fn execute<'e, E>(
                    self,
                    executor: E,
                ) -> impl std::future::Future<
                    Output = Result<
                        <sqlx::MySql as sqlx::Database>::QueryResult,
                        sqlx::Error,
                    >,
                > + Send + 'e
                where
                    Self: Sized + 'e,
                    E: sqlx::Executor<'e, Database = sqlx::MySql> + Send + 'e,
                    sqlx::MySql: 'e,
                {
                    __EnhancedQuery_single::execute(self, executor)
                }
            }
            let __enhanced_runtime_category = "BatchFull";
            let mut __builder: sqlx::QueryBuilder<sqlx::MySql> = sqlx::QueryBuilder::new(
                "",
            );
            __builder.push("DELETE FROM products WHERE category = ");
            __builder.push_bind(__enhanced_runtime_category);
            let __sql_forge_value_single = __EnhancedQuery_single {
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
                                let mut query_args = <sqlx::mysql::MySql as ::sqlx::database::Database>::Arguments::<
                                    '_,
                                >::default();
                                query_args
                                    .reserve(
                                        12usize,
                                        0
                                            + ::sqlx::encode::Encode::<
                                                sqlx::mysql::MySql,
                                            >::size_hint(arg0)
                                            + ::sqlx::encode::Encode::<
                                                sqlx::mysql::MySql,
                                            >::size_hint(arg1)
                                            + ::sqlx::encode::Encode::<
                                                sqlx::mysql::MySql,
                                            >::size_hint(arg2)
                                            + ::sqlx::encode::Encode::<
                                                sqlx::mysql::MySql,
                                            >::size_hint(arg3)
                                            + ::sqlx::encode::Encode::<
                                                sqlx::mysql::MySql,
                                            >::size_hint(arg4)
                                            + ::sqlx::encode::Encode::<
                                                sqlx::mysql::MySql,
                                            >::size_hint(arg5)
                                            + ::sqlx::encode::Encode::<
                                                sqlx::mysql::MySql,
                                            >::size_hint(arg6)
                                            + ::sqlx::encode::Encode::<
                                                sqlx::mysql::MySql,
                                            >::size_hint(arg7)
                                            + ::sqlx::encode::Encode::<
                                                sqlx::mysql::MySql,
                                            >::size_hint(arg8)
                                            + ::sqlx::encode::Encode::<
                                                sqlx::mysql::MySql,
                                            >::size_hint(arg9)
                                            + ::sqlx::encode::Encode::<
                                                sqlx::mysql::MySql,
                                            >::size_hint(arg10)
                                            + ::sqlx::encode::Encode::<
                                                sqlx::mysql::MySql,
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
                                    sqlx::mysql::MySql,
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
            struct __EnhancedQuery_single<'args> {
                inner: sqlx::QueryBuilder<'args, sqlx::MySql>,
            }
            impl<'args> __EnhancedQuery_single<'args> {
                async fn execute<'e, E>(
                    mut self,
                    executor: E,
                ) -> Result<<sqlx::MySql as sqlx::Database>::QueryResult, sqlx::Error>
                where
                    E: sqlx::Executor<'e, Database = sqlx::MySql>,
                {
                    self.inner.build().execute(executor).await
                }
            }
            impl<'args> sql_forge::EnhancedQueryExecute
            for __EnhancedQuery_single<'args> {
                type Db = sqlx::MySql;
                fn execute<'e, E>(
                    self,
                    executor: E,
                ) -> impl std::future::Future<
                    Output = Result<
                        <sqlx::MySql as sqlx::Database>::QueryResult,
                        sqlx::Error,
                    >,
                > + Send + 'e
                where
                    Self: Sized + 'e,
                    E: sqlx::Executor<'e, Database = sqlx::MySql> + Send + 'e,
                    sqlx::MySql: 'e,
                {
                    __EnhancedQuery_single::execute(self, executor)
                }
            }
            let mut __builder: sqlx::QueryBuilder<sqlx::MySql> = sqlx::QueryBuilder::new(
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
            let __sql_forge_value_single = __EnhancedQuery_single {
                inner: __builder,
            };
            __sql_forge_value_single
        }
            .execute(&pool)
            .await
            .expect("batch insert failed");
        let rows: Vec<BatchFullItem> = {
            let _sql_forge_validator = || {
                let __enhanced_top_level_cat = &("BatchFull");
                {
                    type __EnhancedModel = BatchFullItem;
                    {
                        let _ = {
                            {
                                #[allow(clippy::all)]
                                {
                                    use ::sqlx::Arguments as _;
                                    let arg0 = &(__enhanced_top_level_cat);
                                    let mut query_args = <sqlx::mysql::MySql as ::sqlx::database::Database>::Arguments::<
                                        '_,
                                    >::default();
                                    query_args
                                        .reserve(
                                            1usize,
                                            0
                                                + ::sqlx::encode::Encode::<
                                                    sqlx::mysql::MySql,
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
                                        sqlx::mysql::MySql,
                                        _,
                                    >(
                                            "\n        SELECT name, price, stock, category FROM products\n        WHERE category = ?\n        ORDER BY id\n        ",
                                            query_args,
                                        )
                                        .try_map(|row: sqlx::mysql::MySqlRow| {
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
                                                .try_get_unchecked::<i32, _>(2usize)?
                                                .into();
                                            #[allow(non_snake_case)]
                                            let sqlx_query_as_category = row
                                                .try_get_unchecked::<String, _>(3usize)?
                                                .into();
                                            ::std::result::Result::Ok(__EnhancedModel {
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
            struct __EnhancedQuery_single<'args> {
                inner: sqlx::QueryBuilder<'args, sqlx::MySql>,
            }
            impl<'args> __EnhancedQuery_single<'args> {
                async fn fetch_all<'e, E>(
                    mut self,
                    executor: E,
                ) -> Result<Vec<BatchFullItem>, sqlx::Error>
                where
                    E: sqlx::Executor<'e, Database = sqlx::MySql>,
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
                    E: sqlx::Executor<'e, Database = sqlx::MySql>,
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
                    E: sqlx::Executor<'e, Database = sqlx::MySql>,
                {
                    self.inner
                        .build_query_as::<BatchFullItem>()
                        .fetch_optional(executor)
                        .await
                }
                async fn execute<'e, E>(
                    mut self,
                    executor: E,
                ) -> Result<<sqlx::MySql as sqlx::Database>::QueryResult, sqlx::Error>
                where
                    E: sqlx::Executor<'e, Database = sqlx::MySql>,
                {
                    self.inner.build().execute(executor).await
                }
            }
            impl<'args> sql_forge::EnhancedQuery<BatchFullItem>
            for __EnhancedQuery_single<'args> {
                type Db = sqlx::MySql;
                fn fetch_all<'e, E>(
                    self,
                    executor: E,
                ) -> impl std::future::Future<
                    Output = Result<Vec<BatchFullItem>, sqlx::Error>,
                > + Send + 'e
                where
                    Self: Sized + 'e,
                    E: sqlx::Executor<'e, Database = sqlx::MySql> + Send + 'e,
                    sqlx::MySql: 'e,
                {
                    __EnhancedQuery_single::fetch_all(self, executor)
                }
                fn fetch_one<'e, E>(
                    self,
                    executor: E,
                ) -> impl std::future::Future<
                    Output = Result<BatchFullItem, sqlx::Error>,
                > + Send + 'e
                where
                    Self: Sized + 'e,
                    E: sqlx::Executor<'e, Database = sqlx::MySql> + Send + 'e,
                    sqlx::MySql: 'e,
                {
                    __EnhancedQuery_single::fetch_one(self, executor)
                }
                fn fetch_optional<'e, E>(
                    self,
                    executor: E,
                ) -> impl std::future::Future<
                    Output = Result<Option<BatchFullItem>, sqlx::Error>,
                > + Send + 'e
                where
                    Self: Sized + 'e,
                    E: sqlx::Executor<'e, Database = sqlx::MySql> + Send + 'e,
                    sqlx::MySql: 'e,
                {
                    __EnhancedQuery_single::fetch_optional(self, executor)
                }
                fn execute<'e, E>(
                    self,
                    executor: E,
                ) -> impl std::future::Future<
                    Output = Result<
                        <sqlx::MySql as sqlx::Database>::QueryResult,
                        sqlx::Error,
                    >,
                > + Send + 'e
                where
                    Self: Sized + 'e,
                    E: sqlx::Executor<'e, Database = sqlx::MySql> + Send + 'e,
                    sqlx::MySql: 'e,
                {
                    __EnhancedQuery_single::execute(self, executor)
                }
            }
            let __enhanced_runtime_cat = "BatchFull";
            let mut __builder: sqlx::QueryBuilder<sqlx::MySql> = sqlx::QueryBuilder::new(
                "",
            );
            __builder
                .push(
                    "\n        SELECT name, price, stock, category FROM products\n        WHERE category = ",
                );
            __builder.push_bind(__enhanced_runtime_cat);
            __builder.push("\n        ORDER BY id\n        ");
            let __sql_forge_value_single = __EnhancedQuery_single {
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
                let __enhanced_top_level_category = &("BatchFull");
                {
                    {
                        let _ = {
                            #[allow(clippy::all)]
                            {
                                use ::sqlx::Arguments as _;
                                let arg0 = &(__enhanced_top_level_category);
                                let mut query_args = <sqlx::mysql::MySql as ::sqlx::database::Database>::Arguments::<
                                    '_,
                                >::default();
                                query_args
                                    .reserve(
                                        1usize,
                                        0
                                            + ::sqlx::encode::Encode::<
                                                sqlx::mysql::MySql,
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
                                    sqlx::mysql::MySql,
                                    _,
                                >("DELETE FROM products WHERE category = ?", query_args)
                            }
                        };
                    }
                }
            };
            struct __EnhancedQuery_single<'args> {
                inner: sqlx::QueryBuilder<'args, sqlx::MySql>,
            }
            impl<'args> __EnhancedQuery_single<'args> {
                async fn execute<'e, E>(
                    mut self,
                    executor: E,
                ) -> Result<<sqlx::MySql as sqlx::Database>::QueryResult, sqlx::Error>
                where
                    E: sqlx::Executor<'e, Database = sqlx::MySql>,
                {
                    self.inner.build().execute(executor).await
                }
            }
            impl<'args> sql_forge::EnhancedQueryExecute
            for __EnhancedQuery_single<'args> {
                type Db = sqlx::MySql;
                fn execute<'e, E>(
                    self,
                    executor: E,
                ) -> impl std::future::Future<
                    Output = Result<
                        <sqlx::MySql as sqlx::Database>::QueryResult,
                        sqlx::Error,
                    >,
                > + Send + 'e
                where
                    Self: Sized + 'e,
                    E: sqlx::Executor<'e, Database = sqlx::MySql> + Send + 'e,
                    sqlx::MySql: 'e,
                {
                    __EnhancedQuery_single::execute(self, executor)
                }
            }
            let __enhanced_runtime_category = "BatchFull";
            let mut __builder: sqlx::QueryBuilder<sqlx::MySql> = sqlx::QueryBuilder::new(
                "",
            );
            __builder.push("DELETE FROM products WHERE category = ");
            __builder.push_bind(__enhanced_runtime_category);
            let __sql_forge_value_single = __EnhancedQuery_single {
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
#[rustc_main]
#[coverage(off)]
#[doc(hidden)]
pub fn main() -> () {
    extern crate test;
    test::test_main_static(
        &[
            &basic_query_with_inline_params,
            &combining_features_example,
            &execute_batch,
            &execute_batch_full,
            &execute_only_insert_update_delete,
            &execute_only_query,
            &grouped_sections,
            &list_parameter_in_clause,
            &list_parameter_with_empty_guard,
            &multiple_results_group,
            &multiple_results_scalar_key,
            &scalar_output,
            &section_dynamic_where,
            &section_with_local_params,
            &struct_source_params,
        ],
    )
}
