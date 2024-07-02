# Sqlite Database Setup

1. Add the following configuration to the `[dependencies]` section of the `Cargo.toml` file.

    ```toml
    sqlx = { version = "0.7.4", features = [
      "runtime-tokio",
      "sqlite",
    ], optional = true }
    uuid = { version = "1.5.0", optional = true, features = ["v4"] }
    whoami = { version = "1.4.1", features = ["default"] }
    ```

2. In the `ssr` field of the `[features]` section, add the following.

    ```toml
    "dep:sqlx",
    "dep:uuid",
    ```

3. Open a terminal window, ensure you are in the `tools-app` project folder. Run the following command to set the database name.

    ```bash
    export DATABASE_URL="sqlite:toolsapp.sqlite"
    ```

4. Create the database with the following command:

    ```bash
    sqlx database create
    ```

5. Create a migration for the `colors` table.

    ```bash
    sqlx migrate add color
    ```

6. Open the new migration file named `##############_color.sql` created in the `migrations` folder. Add the following SQL code to the file. The migration will be applied when the application runs.

    ```sql
    CREATE TABLE color (
      id VARCHAR NOT NULL PRIMARY KEY,
      name VARCHAR NOT NULL,
      hex_code VARCHAR NOT NULL
    );

    INSERT INTO color (id, name, hex_code) VALUES ('133191a0-d9f1-43fe-8b33-b0eff1439173', 'Red', 'FF0000');
    INSERT INTO color (id, name, hex_code) VALUES ('e7651096-2609-45a5-b5a1-b3323f70d4f1', 'Green', '00FF00');
    INSERT INTO color (id, name, hex_code) VALUES ('8d6f7327-da2f-45cc-9365-a720e06f4358', 'Blue', '0000FF');
    ```

7. Open the `src/main.rs` file. Inside the first `main` function, add the following code to end the `use` section.

    ```rust
    use sqlx::{migrate, sqlite::SqlitePoolOptions};
    ```

8. Inside the first `main` function, add the following code just before the call to `HttpServer::new`.

    ```rust
    let db_pool = SqlitePoolOptions::new()
        .connect("sqlite:/FULL_PATH_TO_DB_FILE/toolsapp.db")
        .await
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

    migrate!("./migrations")
        .run(&db_pool)
        .await
        .unwrap_or_else(|_| panic!("could not run sqlx migration {}", whoami::username().as_str()));
    ```

9. Finally, in the first `main` function, add the following line of code right after `App::new()`.

    ```rust
    .app_data(web::Data::new(db_pool.clone()))
    ```

10. When the application runs, the migrations will be applied, and the database connection will be made available for the server functions to extract.
