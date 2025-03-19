# zirv-sqlx

**zirv-sqlx** is a lightweight convenience wrapper for [SQLx](https://github.com/launchbadge/sqlx) that simplifies global database pool management and transaction handling in Rust.

This library provides a set of easy-to-use macros for:
- Initializing and retrieving a global database pool.
- Handling transactions with minimal boilerplate.
  
The internal implementation leverages the functionality in [`src/db.rs`](src/db.rs) for pool initialization and retrieval and exposes a friendly macro API in [`src/lib.rs`](src/lib.rs).

## Features

- **Global Database Pool Management:**
  - **`init_db_pool!()`**  
    Initializes the global SQLx database pool asynchronously. Under the hood, it calls [`db::init_db_pool`](src/db.rs) to set up a connection pool based on configuration values (like the database URL and maximum connections).

  - **`get_db_pool!()`**  
    Retrieves a reference to the globally initialized database pool. This macro wraps a call to [`db::get_db_pool`](src/db.rs) and will panic if the pool has not yet been initialized.

- **Transaction Helpers:**
  - **`start_transaction!()`**  
    Begins a new transaction using the global pool. If starting the transaction fails, the error is logged and returned.
    
  - **`commit_transaction!()`**  
    Commits an active transaction. If the commit fails, the error is logged and returned.
    
  - **`rollback_transaction!()`**  
    Rolls back an active transaction. If the rollback fails, the error is logged and returned.

Using these macros helps standardize your database operations and reduces repetitive code when integrating with SQLx.

## Installation

Add **zirv-sqlx** as a dependency in your project's `Cargo.toml`:

```sh
cargo add zirv-db-sqlx
```

## Usage

### Example: Setting Up and Using the Database Pool

```rust
// Import the macros
use zirv_db_sqlx::{init_db_pool, get_db_pool, start_transaction, commit_transaction, rollback_transaction};

// Initialize the database pool (usually done at application startup)
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    init_db_pool!("DATABASE_URL", 5).await?;

    // Retrieve the database pool
    let pool = get_db_pool!();

    // Start a transaction
    let mut transaction = start_transaction!()?;

    // Perform some database operations here
    // Example: sqlx::query!("INSERT INTO users (name) VALUES ($1)", "John Doe").execute(&mut transaction).await?;

    // Commit the transaction
    commit_transaction!(transaction)?;

    Ok(())
}
```

### Example: Error Handling with Transactions

```rust
use zirv_db_sqlx::{start_transaction, commit_transaction, rollback_transaction};
use sqlx::PgPool;

async fn perform_db_operations(pool: &PgPool) -> Result<(), sqlx::Error> {
    let mut transaction = start_transaction!()?;

    if let Err(e) = sqlx::query!("INSERT INTO users (name) VALUES ($1)", "Jane Doe")
        .execute(&mut transaction)
        .await
    {
        // Rollback the transaction on error
        rollback_transaction!(transaction)?;
        return Err(e);
    }

    // Commit the transaction if everything succeeds
    commit_transaction!(transaction)?;
    Ok(())
}
```

## Contributing

Contributions are welcome! Please open an issue or submit a pull request if you have ideas for improvements or new features.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

