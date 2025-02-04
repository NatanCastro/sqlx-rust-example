use sqlx::{query::QueryAs, sqlite, FromRow, SqlitePool};

#[derive(Debug, sqlx::FromRow)]
pub struct User {
    pub id: i32,
    pub name: String,
}

type SqliteQueryAs<'a, T> = QueryAs<'a, sqlx::Sqlite, T, sqlite::SqliteArguments<'a>>;

trait QueryParamT<'a>:
    sqlx::Encode<'a, sqlite::Sqlite> + sqlx::Type<sqlite::Sqlite> + Send + 'a
{
}
impl<'a, T: sqlx::Encode<'a, sqlite::Sqlite> + sqlx::Type<sqlite::Sqlite> + Send + 'a>
    QueryParamT<'a> for T
{
}

fn bind_params<'a, T, P>(
    query: SqliteQueryAs<'a, T>,
    params: impl IntoIterator<Item = P>,
) -> SqliteQueryAs<'a, T>
where
    T: for<'r> FromRow<'r, sqlite::SqliteRow>,
    P: QueryParamT<'a>,
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
) -> SqliteQueryAs<'a, T>
where
    T: for<'r> FromRow<'r, sqlite::SqliteRow>,
    P: QueryParamT<'a>,
{
    let query = sqlx::query_as(query);
    bind_params(query, params)
}

type QueryResult<T, E = sqlx::Error> = Result<T, E>;

async fn create_table(pool: &SqlitePool, query: &str) -> QueryResult<()> {
    match sqlx::query(query).execute(pool).await {
        Ok(result) => {
            println!("{:?}", result);
            Ok(())
        }
        Err(err) => Err(err),
    }
}

async fn insert_user(pool: &SqlitePool, name: &str) -> QueryResult<User> {
    build_query_with_params("INSERT INTO users (name) VALUES (?) RETURNING *", [name])
        .fetch_one(pool)
        .await
}

async fn find_users(pool: &SqlitePool) -> QueryResult<Vec<User>> {
    sqlx::query_as("SELECT * FROM users").fetch_all(pool).await
}

#[tokio::main]
async fn main() {
    let pool = SqlitePool::connect(":memory:").await.unwrap();

    create_table(
        &pool,
        "CREATE TABLE users (id INTEGER PRIMARY KEY, name TEXT)",
    )
    .await
    .unwrap();

    let result: Result<User, sqlx::Error> = insert_user(&pool, "Natan").await;
    match result {
        Ok(user) => println!("{:?}", user),
        Err(e) => println!("{:?}", e),
    }
    let result: Result<User, sqlx::Error> = insert_user(&pool, "Augusto").await;
    match result {
        Ok(user) => println!("{:?}", user),
        Err(e) => println!("{:?}", e),
    }

    let result: Result<Vec<User>, sqlx::Error> = find_users(&pool).await;
    match result {
        Ok(users) => println!("{:?}", users),
        Err(e) => println!("{:?}", e),
    }
}
