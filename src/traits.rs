pub trait SqlForgeValidatorValue<Expected> {
    fn sql_forge_validator_value(&self) -> Expected;
}

macro_rules! impl_identity_validator_value {
    ($($ty:ty),* $(,)?) => {
        $(
            impl SqlForgeValidatorValue<$ty> for $ty {
                fn sql_forge_validator_value(&self) -> $ty {
                    self.clone()
                }
            }
        )*
    };
}

impl_identity_validator_value!(
    i8,
    i16,
    i32,
    i64,
    isize,
    u8,
    u16,
    u32,
    u64,
    usize,
    f32,
    f64,
    bool,
    String,
    Vec<u8>
);

impl<'a> SqlForgeValidatorValue<&'a str> for &'a str {
    fn sql_forge_validator_value(&self) -> &'a str {
        self
    }
}

impl<T, Expected> SqlForgeValidatorValue<Expected> for &T
where
    T: SqlForgeValidatorValue<Expected>,
{
    fn sql_forge_validator_value(&self) -> Expected {
        <T as SqlForgeValidatorValue<Expected>>::sql_forge_validator_value(*self)
    }
}

impl<T, U> SqlForgeValidatorValue<Option<U>> for Option<T>
where
    T: SqlForgeValidatorValue<U>,
{
    fn sql_forge_validator_value(&self) -> Option<U> {
        self.as_ref().map(|value| value.sql_forge_validator_value())
    }
}

pub fn sql_forge_validator_value<Expected, T>(value: &T) -> Expected
where
    T: SqlForgeValidatorValue<Expected>,
{
    <T as SqlForgeValidatorValue<Expected>>::sql_forge_validator_value(value)
}

pub trait SqlForgeQuery<Output> {
    type Db: sqlx::Database;

    fn fetch_all<'e, E>(
        self,
        executor: E,
    ) -> impl std::future::Future<Output = Result<Vec<Output>, sqlx::Error>> + Send + 'e
    where
        Self: Sized + 'e,
        E: sqlx::Executor<'e, Database = Self::Db> + Send + 'e,
        Self::Db: 'e;

    fn fetch_one<'e, E>(
        self,
        executor: E,
    ) -> impl std::future::Future<Output = Result<Output, sqlx::Error>> + Send + 'e
    where
        Self: Sized + 'e,
        E: sqlx::Executor<'e, Database = Self::Db> + Send + 'e,
        Self::Db: 'e;

    fn fetch_optional<'e, E>(
        self,
        executor: E,
    ) -> impl std::future::Future<Output = Result<Option<Output>, sqlx::Error>> + Send + 'e
    where
        Self: Sized + 'e,
        E: sqlx::Executor<'e, Database = Self::Db> + Send + 'e,
        Self::Db: 'e;

    fn execute<'e, E>(
        self,
        executor: E,
    ) -> impl std::future::Future<
        Output = Result<<Self::Db as sqlx::Database>::QueryResult, sqlx::Error>,
    > + Send
           + 'e
    where
        Self: Sized + 'e,
        E: sqlx::Executor<'e, Database = Self::Db> + Send + 'e,
        Self::Db: 'e;
}

pub trait SqlForgeQueryExecute {
    type Db: sqlx::Database;

    fn execute<'e, E>(
        self,
        executor: E,
    ) -> impl std::future::Future<
        Output = Result<<Self::Db as sqlx::Database>::QueryResult, sqlx::Error>,
    > + Send
           + 'e
    where
        Self: Sized + 'e,
        E: sqlx::Executor<'e, Database = Self::Db> + Send + 'e,
        Self::Db: 'e;
}

pub trait SqlForgeQueryGroup {
    type Db: sqlx::Database;
}

pub trait SqlForgeQueryGroupGet<Key, Output>: SqlForgeQueryGroup {
    type Query: SqlForgeQuery<Output, Db = Self::Db>;

    fn get(self, _: Key) -> Self::Query;
}
