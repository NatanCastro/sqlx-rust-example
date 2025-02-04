use sqlx::{query::QueryAs, sqlite, FromRow, SqlitePool};

#[derive(Debug, sqlx::FromRow)]
pub struct User {
    pub id: i32,
    pub name: String,
}

impl User {
    pub fn new(id: i32, name: String) -> Self {
        Self { id, name }
    }
}

fn bind_params<'a, T, P>(
    query: QueryAs<'a, sqlx::Sqlite, T, sqlite::SqliteArguments<'a>>,
    params: impl IntoIterator<Item = P>,
) -> QueryAs<'a, sqlx::Sqlite, T, sqlite::SqliteArguments<'a>>
where
    T: for<'r> FromRow<'r, sqlite::SqliteRow>,
    P: sqlx::Encode<'a, sqlx::Sqlite> + sqlx::Type<sqlx::Sqlite> + Send + 'a,
{
    let mut query = query;
    for param in params {
        query = query.bind(param);
    }
    query
}

fn build_query_with_params<'a, T, P>(
    query: &'a str,
    params: impl IntoIterator<Item = P>,
) -> QueryAs<'a, sqlx::Sqlite, T, sqlite::SqliteArguments<'a>>
where
    T: for<'r> FromRow<'r, sqlite::SqliteRow>,
    P: sqlx::Encode<'a, sqlite::Sqlite> + sqlx::Type<sqlite::Sqlite> + Send + 'a,
{
    let query = sqlx::query_as(query);
    bind_params(query, params)
}

#[tokio::main]
async fn main() {
    let pool = SqlitePool::connect(":memory:").await.unwrap();
    sqlx::query("CREATE TABLE users (id INTEGER PRIMARY KEY, name TEXT)")
        .execute(&pool)
        .await
        .unwrap();
    let user: User = build_query_with_params("INSERT INTO users (name) VALUES (?)", ["Natan"])
        .fetch_one(&pool)
        .await
        .unwrap();
    println!("{:?}", user);
    let users: Vec<User> = sqlx::query_as("SELECT * FROM users")
        .fetch_all(&pool)
        .await
        .unwrap();
    println!("{:?}", users);
}
