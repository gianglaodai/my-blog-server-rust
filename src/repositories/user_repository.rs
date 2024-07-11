use sqlx::Pool;
use crate::entities::user_entity::User;

pub async fn find_users<DB> (pool: &Pool<DB>) -> impl Fn () -> Result<Vec<User>, sqlx::Error>{
    async move || {
        let users = sqlx::query_as::<_, User>("SELECT * FROM users")
            .fetch_all(&pool)
            .await?;
        Ok(users)
    }
}