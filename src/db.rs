use bcrypt;
use deadpool_postgres;
use tokio_pg_mapper::FromTokioPostgresRow;

type Returns<T> = Result<T, crate::errors::Errors>;

/// Register user
pub async fn register_user(
    client: &deadpool_postgres::Client,
    username: &str,
    plain_password: &str,
) -> Returns<()> {
    if check_exists_user(client, username).await? {
        return Err(crate::errors::Errors::UsernameTaken)
    }
    let hashed_password = bcrypt::hash(plain_password, bcrypt::DEFAULT_COST)?;
    client
        .execute(
            "INSERT INTO users(username, hashed_password) VALUES ($1, $2)",
            &[&username, &&hashed_password],
        )
        .await?;
    return Ok(());
}

/// Logins the user
pub async fn login_user(
    client: &deadpool_postgres::Client,
    username: &str,
    plain_password: &str,
) -> Returns<()> {
    let user_row = client
        .query_one(
            "SELECT hashed_password FROM users WHERE username = $1",
            &[&username],
        )
        .await?;
    if user_row.is_empty() {
        return Err(crate::errors::Errors::UserNotFound);
    }
    let hashed_password = crate::models::User::from_row(user_row)?.hashed_password;
    if bcrypt::verify(plain_password, &hashed_password)? {
        return Ok(());
    }
    return Err(crate::errors::Errors::InvalidPassword);
}

/// Checks if the username exists in database
pub async fn check_exists_user(
    client: &deadpool_postgres::Client,
    username: &str,
) -> Returns<bool> {
    let user_row = client
        .query_one(
            "SELECT * FROM users WHERE username = $1",
            &[&username],
        )
        .await?;
    return Ok(user_row.is_empty())
}
