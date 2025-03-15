pub mod db;

/// Macro to initialize the global database pool.
///
/// This macro wraps the asynchronous function from the `db` module:
/// `$crate::db::init_db_pool().await`. It should be called early in your application
/// (e.g., in your `main` function) to set up the database connection pool.
///
/// # Example
/// ```rust
/// use zirv_db_sqlx::init_db_pool;
/// use zirv_config::register_config;
/// use serde::{Serialize, Deserialize};
/// 
/// #[derive(Serialize, Deserialize)]
/// struct DatabaseConfig {
///    url: String,
///   max_connections: u32,
/// }
/// 
/// # #[tokio::main]
/// # async fn main() {
/// 
/// register_config!("database", DatabaseConfig {
///     url: "mysql://root:password@localhost".to_owned(),
///    max_connections: 10,
/// });
/// init_db_pool!();
/// # }
/// ```
#[macro_export]
macro_rules! init_db_pool {
    () => {
        $crate::db::init_db_pool().await;
    }
}

/// Macro to retrieve a reference to the global database pool.
///
/// This macro wraps the call to `$crate::db::get_db_pool()`, which returns a reference
/// to the initialized pool. It panics if the pool has not been initialized yet (i.e., if
/// `init_db_pool!()` has not been called).
///
/// # Example
/// ```rust
/// use zirv_db_sqlx::get_db_pool;
/// 
/// fn perform_db_operations() {
///    let pool = get_db_pool!();
///   // Use `pool` for database operations...
/// }
/// ```
#[macro_export]
macro_rules! get_db_pool {
    () => {
        $crate::db::get_db_pool()
    }
}

/// Macro to start a new database transaction.
///
/// This macro begins a transaction using the global database pool by calling `begin()` on it.
/// In case of an error starting the transaction, the error is logged and returned.
///
/// # Example
/// ```rust
/// use zirv_db_sqlx::{start_transaction, commit_transaction};
/// 
/// async fn perform_db_operations() -> Result<(), sqlx::Error> {
///     let mut tx = start_transaction!();
///     // Execute operations within the transaction...
///     commit_transaction!(tx);
///     Ok(())
/// }
/// ```
#[macro_export]
macro_rules! start_transaction {
    () => {
        match $crate::db::get_db_pool().begin().await {
            Ok(tx) => tx,
            Err(e) => {
                eprintln!("Failed to start transaction: {:?}", e);
                return Err(e);
            }
        }
    };
}

/// Macro to commit an active transaction.
///
/// This macro takes a transaction handle as an argument and commits the transaction.
/// If the commit fails, it logs the error and returns it.
///
/// # Example
/// ```rust
/// use zirv_db_sqlx::{commit_transaction, start_transaction};
/// 
/// async fn perform_db_operations() -> Result<(), sqlx::Error> {
///    let mut tx = start_transaction!();
///    // Execute operations within the transaction...
///    commit_transaction!(tx);
///    Ok(())
/// }
/// ```
#[macro_export]
macro_rules! commit_transaction {
    ($tx:expr) => {
        match $tx.commit().await {
            Ok(_) => (),
            Err(e) => {
                eprintln!("Failed to commit transaction: {:?}", e);
                return Err(e);
            }
        }
    };
}

/// Macro to rollback an active transaction.
///
/// This macro takes a transaction handle as an argument and rolls back the transaction.
/// If the rollback fails, it logs the error and returns it.
///
/// # Example
/// ```rust
/// use zirv_db_sqlx::{rollback_transaction, start_transaction};
/// 
/// async fn perform_db_operations() -> Result<(), sqlx::Error> {
///    let mut tx = start_transaction!();
///    // Execute operations within the transaction...
///    rollback_transaction!(tx);
///    Ok(())
/// }
/// ```
#[macro_export]
macro_rules! rollback_transaction {
    ($tx:expr) => {
        match $tx.rollback().await {
            Ok(_) => (),
            Err(e) => {
                eprintln!("Failed to rollback transaction: {:?}", e);
                return Err(e);
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::query_as;
    use std::env;

    /// Helper function to check if a required environment variable is set.
    fn is_db_configured() -> bool {
        // Adjust this check according to how your `read_config!` macro obtains its configuration.
        // Here we simply check for the DATABASE_URL env variable.
        env::var("DATABASE_URL").is_ok()
    }

    /// Test that the database pool can be initialized and retrieved.
    #[tokio::test]
    async fn test_init_and_get_db_pool() {
        if !is_db_configured() {
            eprintln!("DATABASE_URL not set. Skipping test_init_and_get_db_pool.");
            return;
        }

        // Initialize the DB pool using the macro.
        init_db_pool!();

        // Retrieve the pool using the macro.
        let pool = get_db_pool!();

        // Execute a simple query to verify the connection.
        let row: (i32,) = query_as("SELECT 1")
            .fetch_one(pool)
            .await
            .expect("Failed to execute test query on DB pool");
        assert_eq!(row.0, 1);
    }
}
