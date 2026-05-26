#[derive(sqlx::FromRow)]
pub struct User {
    pub id: i64,
    pub name: String,
}

#[derive(sqlx::FromRow)]
pub struct AmountResult {
    pub total: Option<i64>,
}

#[derive(sqlx::Type)]
#[sqlx(transparent)]
pub struct TransparentId(pub i64);

pub struct RawId(pub i64);

pub struct BatchName {
    pub name: String,
}

pub struct BatchNamePrice {
    pub name: String,
    pub price: i64,
}

pub struct BatchIdCategory {
    pub id: i64,
    pub category: String,
}