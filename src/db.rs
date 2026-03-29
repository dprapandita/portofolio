#[cfg(feature = "ssr")]
pub mod server {
    use sqlx::{sqlite::SqlitePoolOptions, SqlitePool};
    use bcrypt::{hash, DEFAULT_COST};

    pub async fn setup_db() -> Result<SqlitePool, Box<dyn std::error::Error>> {
        // Create an in-memory or file-based SQLite database
        let pool = SqlitePoolOptions::new()
            .max_connections(5)
            .connect("sqlite:portfolio.db?mode=rwc")
            .await?;

        // Create Users table
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS users (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                username TEXT UNIQUE NOT NULL,
                password_hash TEXT NOT NULL
            );"
        )
        .execute(&pool)
        .await?;

        // Create Projects table
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS projects (
                id TEXT PRIMARY KEY,
                title TEXT NOT NULL,
                category TEXT NOT NULL,
                client TEXT NOT NULL,
                work_category TEXT NOT NULL,
                date_text TEXT NOT NULL,
                tools TEXT NOT NULL,
                story TEXT NOT NULL,
                outcome TEXT NOT NULL,
                image_url TEXT NOT NULL,
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP
            );"
        )
        .execute(&pool)
        .await?;

        // Seed default admin user if no users exist
        let user_count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM users")
            .fetch_one(&pool)
            .await?;

        if user_count.0 == 0 {
            let hashed_pass = hash("admin123", DEFAULT_COST)?;
            sqlx::query("INSERT INTO users (username, password_hash) VALUES (?, ?)")
                .bind("admin")
                .bind(hashed_pass)
                .execute(&pool)
                .await?;
            leptos::logging::log!("Seeded default admin user with username: 'admin', password: 'admin123'");
        }

        Ok(pool)
    }
}
