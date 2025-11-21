use std::env;
use tiberius::{AuthMethod, Client, Config};
use tokio::net::TcpStream;
use tokio_util::compat::TokioAsyncWriteCompatExt;

/// Database connection wrapper
#[derive(Clone)]
pub struct Database {
    host: String,
    port: u16,
    user: String,
    password: String,
    database: String,
}

impl Database {
    /// Create a new database connection configuration
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let host = env::var("DB_HOST").unwrap_or_else(|_| "sqlserver".to_string());
        let port = env::var("DB_PORT")
            .unwrap_or_else(|_| "1433".to_string())
            .parse::<u16>()?;
        let user = env::var("DB_USER").unwrap_or_else(|_| "SA".to_string());
        let password = env::var("DB_PASSWORD").expect("DB_PASSWORD must be set");
        let database = env::var("DB_NAME").unwrap_or_else(|_| "zoo_db".to_string());

        println!("Database config: {}:{}/{}", host, port, database);

        Ok(Self {
            host,
            port,
            user,
            password,
            database,
        })
    }

    /// Create a new database connection
    pub async fn connect(
        &self,
    ) -> Result<Client<tokio_util::compat::Compat<TcpStream>>, Box<dyn std::error::Error>> {
        let mut config = Config::new();
        config.host(&self.host);
        config.port(self.port);
        config.authentication(AuthMethod::sql_server(&self.user, &self.password));
        config.trust_cert();
        config.database(&self.database);

        let tcp = TcpStream::connect(format!("{}:{}", self.host, self.port)).await?;
        let client = Client::connect(config, tcp.compat_write()).await?;

        println!("Connected to SQL Server successfully");
        Ok(client)
    }

    /// Test the database connection
    pub async fn test_connection(&self) -> Result<(), Box<dyn std::error::Error>> {
        let mut client = self.connect().await?;

        // Simple query to test connection
        client.simple_query("SELECT 1").await?.into_row().await?;

        println!("Database connection test successful");
        Ok(())
    }
}
