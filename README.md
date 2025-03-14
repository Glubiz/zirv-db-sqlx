# zirv-sqlx

**zirv-sqlx** is a lightweight convenience wrapper for [SQLx](https://github.com/launchbadge/sqlx) that simplifies global database pool management and transaction handling in Rust. It provides a set of easy-to-use macros for initializing and retrieving a global database pool as well as starting, committing, and rolling back transactions.

## Features

- **Global Database Pool Management:**
  - `init_db_pool!()`  
    Initializes the global SQLx database pool asynchronously. This macro wraps the call to the underlying asynchronous function from the `db` module. It should be called early in your application (e.g. in `main()`) to set up the database connection pool.

  - `get_db_pool!()`  
    Retrieves a reference to the globally initialized database pool. This macro wraps a call to the `get_db_pool()` function, panicking if the pool has not been initialized.

- **Transaction Helpers:**
  - `start_transaction!()`  
    Begins a new transaction using the global pool. If starting the transaction fails, it logs the error and returns early with the error.
    
  - `commit_transaction!()`  
    Commits an active transaction. If the commit fails, the error is logged and returned.
    
  - `rollback_transaction!()`  
    Rolls back an active transaction. If the rollback fails, the error is logged and returned.

These macros help reduce boilerplate and standardize your database operations when using SQLx.

## Installation

Add **zirv-sqlx** as a dependency in your project's `Cargo.toml`. For example, if you are publishing or using it locally:

```sh
cargo add zirv-db-sqlx
``` 

