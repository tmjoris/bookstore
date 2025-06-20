use std::error::Error;
use sqlx::Row;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let url = "postgres://pred-zero:jtcmb=bl=1234@localhost:5432/bookstore";
    let pool = sqlx::postgres::PgPool::connect(url).await?;

    let res = sqlx::query("SELECT 1 + 1 as sum")
        .fetch_one(&pool)
        .await?;
    
    let sum: i32 = res.get("sum");
    println!("1 + 1 = {}", sum);
    Ok(())
}
