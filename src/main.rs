use std::error::Error;
use sqlx::Row;

#[derive(Debug)]
struct Book {
    pub title: String,
    pub author: String,
    pub isbn: String
}

async fn create(book: &Book, pool: &sqlx::PgPool) -> Result<(), Box<dyn Error>> {
    let query = "INSERT INTO book (title, author, isbn) VALUES ($1, $2, $3)";

    sqlx::query(query)
        .bind(&book.title)
        .bind(&book.author)
        .bind(&book.isbn)
        .execute(pool)
        .await?;
    
    Ok(())
}

async fn update(book: &Book, isbn: &str, pool: &sqlx::PgPool
) -> Result<(), Box<dyn Error>> {
    let query = "UPDATE book SET title = $1, author = $2 WHERE isbn = $3";

    sqlx::query(query)
        .bind(&book.title)
        .bind(&book.author)
        .bind(isbn)
        .execute(pool)
        .await?;
    
    Ok(())
}

async fn read(conn: &sqlx::PgPool) -> Result<Book, Box<dyn Error>> {
    let q = "SELECT title, author, isbn FROM book";
    let query = sqlx::query(q);

    let row = query.fetch_one(conn).await?;

    let book = Book{
        title: row.get("title"),
        author: row.get("author"),
        isbn: row.get("isbn")
    };
    Ok(book)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let url = "postgres://pred-zero:jtcmb=bl=1234@localhost:5432/bookstore";
    let pool = sqlx::postgres::PgPool::connect(url).await?;

    sqlx::migrate!("./migrations").run(&pool).await?;

    let book = Book{
        title: "Salem's Lot".to_string(),
        author: "Steven King".to_string(),
        isbn: "978-0-385-00751".to_string(),
    };

    //create(&book, &pool).await?;
      
    //update(&book, &book.isbn, &pool).await?;

    let result = read(&pool).await?;

    println!("{:?}",result);

    Ok(())
}
