pub trait EnhancedQuery<Output> {
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

pub trait EnhancedQueryExecute {
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

pub trait EnhancedQueryGroup {
    type Db: sqlx::Database;
}

pub trait EnhancedQueryGroupGet<Key, Output>: EnhancedQueryGroup {
    type Query: EnhancedQuery<Output, Db = Self::Db>;

    fn get(self, _: Key) -> Self::Query;
}
