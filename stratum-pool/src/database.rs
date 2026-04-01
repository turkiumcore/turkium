use anyhow::Result;
use sqlx::sqlite::{SqlitePool, SqlitePoolOptions};
use sqlx::Row;

pub struct Database {
    pool: SqlitePool,
}

impl Database {
    pub async fn new(path: &str) -> Result<Self> {
        // Create database file if not exists
        if !std::path::Path::new(path).exists() {
            std::fs::File::create(path)?;
        }

        let database_url = format!("sqlite://{}", path);
        
        let pool = SqlitePoolOptions::new()
            .max_connections(20)
            .connect(&database_url)
            .await?;

        Ok(Self { pool })
    }

    pub async fn migrate(&self) -> Result<()> {
        // Create tables
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS miners (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                address TEXT NOT NULL UNIQUE,
                username TEXT NOT NULL,
                difficulty REAL NOT NULL DEFAULT 1.0,
                shares_found INTEGER NOT NULL DEFAULT 0,
                total_difficulty REAL NOT NULL DEFAULT 0.0,
                last_share_time INTEGER NOT NULL,
                created_at INTEGER NOT NULL
            )
            "#,
        )
        .execute(&self.pool)
        .await?;

        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS shares (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                miner_id INTEGER NOT NULL,
                job_id TEXT NOT NULL,
                difficulty REAL NOT NULL,
                is_block INTEGER NOT NULL DEFAULT 0,
                timestamp INTEGER NOT NULL,
                created_at INTEGER NOT NULL,
                FOREIGN KEY(miner_id) REFERENCES miners(id)
            )
            "#,
        )
        .execute(&self.pool)
        .await?;

        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS blocks (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                miner_id INTEGER NOT NULL,
                block_hash TEXT NOT NULL UNIQUE,
                block_height INTEGER NOT NULL,
                reward INTEGER NOT NULL,
                timestamp INTEGER NOT NULL,
                created_at INTEGER NOT NULL,
                FOREIGN KEY(miner_id) REFERENCES miners(id)
            )
            "#,
        )
        .execute(&self.pool)
        .await?;

        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS rewards (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                miner_id INTEGER NOT NULL,
                amount INTEGER NOT NULL,
                block_id INTEGER NOT NULL,
                paid INTEGER NOT NULL DEFAULT 0,
                paid_at INTEGER,
                created_at INTEGER NOT NULL,
                FOREIGN KEY(miner_id) REFERENCES miners(id),
                FOREIGN KEY(block_id) REFERENCES blocks(id)
            )
            "#,
        )
        .execute(&self.pool)
        .await?;

        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS block_templates (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                job_id TEXT NOT NULL UNIQUE,
                prev_hash TEXT NOT NULL,
                coinbase1 TEXT NOT NULL,
                coinbase2 TEXT NOT NULL,
                merkle_branch TEXT NOT NULL,
                version TEXT NOT NULL,
                nbits TEXT NOT NULL,
                ntime TEXT NOT NULL,
                height INTEGER NOT NULL,
                created_at INTEGER NOT NULL
            )
            "#,
        )
        .execute(&self.pool)
        .await?;

        // Create indexes
        sqlx::query("CREATE INDEX IF NOT EXISTS idx_shares_miner_id ON shares(miner_id)")
            .execute(&self.pool)
            .await?;

        sqlx::query("CREATE INDEX IF NOT EXISTS idx_shares_timestamp ON shares(timestamp)")
            .execute(&self.pool)
            .await?;

        sqlx::query("CREATE INDEX IF NOT EXISTS idx_blocks_miner_id ON blocks(miner_id)")
            .execute(&self.pool)
            .await?;

        sqlx::query("CREATE INDEX IF NOT EXISTS idx_rewards_miner_id ON rewards(miner_id)")
            .execute(&self.pool)
            .await?;

        sqlx::query("CREATE INDEX IF NOT EXISTS idx_rewards_paid ON rewards(paid)")
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    pub fn pool(&self) -> &SqlitePool {
        &self.pool
    }
}

// Miner operations - used by server
pub async fn get_or_create_miner(
    pool: &SqlitePool,
    address: &str,
    username: &str,
) -> Result<i64> {
    let now = chrono::Utc::now().timestamp();

    let result = sqlx::query_scalar::<_, i64>(
        "SELECT id FROM miners WHERE address = ?1"
    )
    .bind(address)
    .fetch_optional(pool)
    .await?;

    if let Some(id) = result {
        return Ok(id);
    }

    let id = sqlx::query_scalar::<_, i64>(
        r#"
        INSERT INTO miners (address, username, last_share_time, created_at)
        VALUES (?1, ?2, ?3, ?4)
        RETURNING id
        "#
    )
    .bind(address)
    .bind(username)
    .bind(now)
    .bind(now)
    .fetch_one(pool)
    .await?;

    Ok(id)
}

// Used by share processor
pub async fn update_miner_difficulty(
    pool: &SqlitePool,
    miner_id: i64,
    difficulty: f64,
) -> Result<()> {
    sqlx::query("UPDATE miners SET difficulty = ?1 WHERE id = ?2")
        .bind(difficulty)
        .bind(miner_id)
        .execute(pool)
        .await?;

    Ok(())
}

// Used by share processor
pub async fn save_share(
    pool: &SqlitePool,
    miner_id: i64,
    job_id: &str,
    difficulty: f64,
    is_block: bool,
) -> Result<()> {
    let now = chrono::Utc::now().timestamp();

    sqlx::query(
        r#"
        INSERT INTO shares (miner_id, job_id, difficulty, is_block, timestamp, created_at)
        VALUES (?1, ?2, ?3, ?4, ?5, ?6)
        "#
    )
    .bind(miner_id)
    .bind(job_id)
    .bind(difficulty)
    .bind(is_block as i32)
    .bind(now)
    .bind(now)
    .execute(pool)
    .await?;

    // Update miner stats
    sqlx::query(
        r#"
        UPDATE miners 
        SET shares_found = shares_found + 1,
            total_difficulty = total_difficulty + ?1,
            last_share_time = ?2
        WHERE id = ?3
        "#
    )
    .bind(difficulty)
    .bind(now)
    .bind(miner_id)
    .execute(pool)
    .await?;

    Ok(())
}

// Used by share processor
pub async fn get_miner_stats(
    pool: &SqlitePool,
    miner_id: i64,
) -> Result<(i64, f64)> {
    let row = sqlx::query(
        "SELECT shares_found, total_difficulty FROM miners WHERE id = ?1"
    )
    .bind(miner_id)
    .fetch_one(pool)
    .await?;

    let shares: i64 = row.get(0);
    let difficulty: f64 = row.get(1);

    Ok((shares, difficulty))
}

// Used by reward distributor
pub async fn get_unpaid_rewards(
    pool: &SqlitePool,
) -> Result<Vec<(i64, i64, String)>> {
    let rows = sqlx::query(
        r#"
        SELECT r.id, r.amount, m.address
        FROM rewards r
        JOIN miners m ON r.miner_id = m.id
        WHERE r.paid = 0
        "#
    )
    .fetch_all(pool)
    .await?;

    let mut result = Vec::new();
    for row in rows {
        let reward_id: i64 = row.get(0);
        let amount: i64 = row.get(1);
        let address: String = row.get(2);
        result.push((reward_id, amount, address));
    }

    Ok(result)
}

// Used by reward distributor
pub async fn mark_reward_paid(
    pool: &SqlitePool,
    reward_id: i64,
) -> Result<()> {
    let now = chrono::Utc::now().timestamp();

    sqlx::query("UPDATE rewards SET paid = 1, paid_at = ?1 WHERE id = ?2")
        .bind(now)
        .bind(reward_id)
        .execute(pool)
        .await?;

    Ok(())
}

// Used by template updater
pub async fn save_block_template(
    pool: &SqlitePool,
    job_id: &str,
    prev_hash: &str,
    coinbase1: &str,
    coinbase2: &str,
    merkle_branch: &str,
    version: &str,
    nbits: &str,
    ntime: &str,
    height: i64,
) -> Result<()> {
    let now = chrono::Utc::now().timestamp();

    sqlx::query(
        r#"
        INSERT OR REPLACE INTO block_templates 
        (job_id, prev_hash, coinbase1, coinbase2, merkle_branch, version, nbits, ntime, height, created_at)
        VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)
        "#
    )
    .bind(job_id)
    .bind(prev_hash)
    .bind(coinbase1)
    .bind(coinbase2)
    .bind(merkle_branch)
    .bind(version)
    .bind(nbits)
    .bind(ntime)
    .bind(height)
    .bind(now)
    .execute(pool)
    .await?;

    Ok(())
}
