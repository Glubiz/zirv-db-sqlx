use std::sync::OnceLock;
use sqlx::{Pool, MySql, mysql::MySqlPoolOptions};
use zirv_config::read_config;

// Our global, one-time-initialized pool
static DB_POOL: OnceLock<Pool<MySql>> = OnceLock::new();

/// Initializes the global database pool exactly once.
/// 
/// This function should be called early in your application's lifecycle (for example, in your `main` function).
/// It reads the configuration for the maximum number of database connections and the database URL.
/// If no value is provided for the maximum connections, it defaults to 10.
/// 
/// # Panics
/// - If no database URL is provided in the configuration.
/// - If the pool fails to be created.
/// - If the global pool is already initialized.
pub async fn init_db_pool() {
    let max_database_connections: u32 = read_config!("database.max_connections", u32).unwrap_or(10);
    let database_url = read_config!("database.url", String).unwrap();

    let pool = MySqlPoolOptions::new()
        .max_connections(max_database_connections)
        .connect(&database_url)
        .await
        .expect("Failed to create MySQL pool.");

    // Attempt to set the static DB_POOL. If `set` fails, the pool was already set.
    DB_POOL
        .set(pool)
        .expect("DB_POOL can only be initialized once!");
}

/// Retrieves a reference to the global database pool.
///
/// # Panics
/// Panics if `init_db_pool` has not been called, as the pool will not be initialized.
/// 
/// # Returns
/// A reference to the initialized `Pool<MySql>`.
pub fn get_db_pool() -> &'static Pool<MySql> {
    DB_POOL
        .get()
        .expect("DB pool not initialized! Call init_db_pool first.")
}
