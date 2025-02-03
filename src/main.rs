use sqlx::{
    pool,
    query::QueryAs,
    query_as,
    sqlite::{Sqlite, SqliteArguments, SqliteRow},
    FromRow,
};

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

fn bind_params<'a, T>(
    query: QueryAs<'a, Sqlite, T, SqliteArguments<'a>>,
    mut params: impl Iterator<Item = String>,
) -> QueryAs<'a, Sqlite, T, SqliteArguments<'a>> {
    match params.next() {
        Some(param) => {
            let new_query = query.bind(param);
            bind_params(new_query, params)
        }
        None => query,
    }
}

fn build_query_with_params<'a, T>(
    query: &'a str,
    params: Vec<String>,
) -> QueryAs<'a, Sqlite, T, SqliteArguments<'a>>
where
    T: for<'r> FromRow<'r, SqliteRow>,
{
    bind_params(sqlx::query_as(query), params.into_iter())
}

#[tokio::main]
async fn main() {
    let pool = sqlx::sqlite::SqlitePool::connect(":memory:").await.unwrap();
    sqlx::query("CREATE TABLE users (id INTEGER PRIMARY KEY, name TEXT)")
        .execute(&pool)
        .await
        .unwrap();
    let user: User = build_query_with_params(
        "INSERT INTO users (name) VALUES (?)",
        vec!["Natan".to_string()],
    )
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
