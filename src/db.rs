use tokio_postgres::{Client, NoTls, Error};


pub async fn establish_connection() -> Result<Client, Error> {
    // Replace "localhost" with "db", which is the service name in docker-compose.yml
    let (client, connection) = tokio_postgres::connect("host=db user=user password=password dbname=mydatabase", NoTls).await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("Connection error: {}", e);
        }
    });

    Ok(client)
}


pub async fn create_urls_table(client: &Client) -> Result<(), Error> {
    let statement = "
        CREATE TABLE IF NOT EXISTS urls (
            id SERIAL PRIMARY KEY,
            short_url VARCHAR(255) UNIQUE NOT NULL,
            original_url TEXT NOT NULL,
            created_at TIMESTAMP WITHOUT TIME ZONE DEFAULT CURRENT_TIMESTAMP
        );
    ";
    client.execute(statement, &[]).await?;
    Ok(())
}

pub async fn insert_url(client: &Client, original_url: &str, short_url: &str) -> Result<(), Error> {
    let stmt = "INSERT INTO urls (original_url, short_url) VALUES ($1, $2)";
    client.execute(stmt, &[&original_url, &short_url]).await?;
    Ok(())
}

pub async fn check_long_url_existence(client: &Client, original_url: &str) -> Result<Option<String>, Error> {
    let stmt = "SELECT short_url FROM urls WHERE original_url = $1";
    let rows = client.query(stmt, &[&original_url]).await?;

    if let Some(row) = rows.into_iter().next() {
        Ok(Some(row.get("short_url")))
    } else {
        Ok(None)
    }
}

pub async fn find_original_url(client: &Client, short_url: &str) -> Result<Option<String>, Error> {
    let stmt = "SELECT original_url FROM urls WHERE short_url = $1";
    let rows = client.query(stmt, &[&short_url]).await?;

    Ok(rows.into_iter().next().map(|row| row.get("original_url")))
}