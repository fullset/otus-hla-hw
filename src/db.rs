pub type Pool = sqlx::PgPool;
pub type Connection = sqlx::PgConnection;

#[derive(Debug, Clone)]
pub struct DbClient {
    pool: Pool,
}

impl DbClient {
    pub async fn connect(cfg: &PgConfig) -> Result<Self, sqlx::Error> {
        Ok(Self {
            pool: cfg.builder().finish().await?,
        })
    }

    pub fn pool(&self) -> &Pool {
        &self.pool
    }
}

use std::{
    time::Duration,
};

use  slog::{self, Level};

pub use sqlx::{self, PgPool};

use serde::Deserialize;
use sqlx::{
    pool::{PoolConnection, PoolOptions},
    postgres::PgConnectOptions, Postgres,
};

pub type PgConnection = PoolConnection<Postgres>;

// From https://docs.rs/sqlx-core/0.6.2/src/sqlx_core/pool/options.rs.html#128
fn default_max_lifetime() -> Option<Duration> {
    Some(Duration::from_secs(30 * 60))
}

// From https://docs.rs/sqlx-core/0.6.2/src/sqlx_core/pool/options.rs.html#127
fn default_idle_timeout() -> Option<Duration> {
    Some(Duration::from_secs(10 * 60))
}

// From https://docs.rs/sqlx-core/0.6.2/src/sqlx_core/connection.rs.html#146
fn default_statement_log_slow_duration() -> Duration {
    Duration::from_secs(1)
}

fn default_statement_log_slow_level() -> Level {
    Level::Warning
}

/// Конфигурация пула подключений к БД.
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct PgConfig {
    // Postgres options
    pub host: String,
    pub port: u16,
    pub database: String,
    pub user: String,
    pub password: String,

    // SQLx pool options
    pub max_connections: u32,
    pub min_connections: Option<u32>,
}

impl PgConfig {
    /// Создание билдера из текущего конфига.
    pub fn builder(&self) -> PgPoolBuilder {
        PgPoolBuilder::new(self)
    }
}

/// Билдер пула соединений к PostgreSQL.
pub struct PgPoolBuilder {
    conn_config: PgConnectOptions,
    pool_config: PoolOptions<Postgres>,
}

impl PgPoolBuilder {
    /// Создание нового билдера из конфига.
    pub fn new(config: &PgConfig) -> Self {
        let mut conn_config = PgConnectOptions::new()
            .host(&config.host)
            .port(config.port)
            .username(&config.user)
            .database(&config.database)
            .password(&config.password);

        let mut pool_config = PoolOptions::new()
            .max_connections(config.max_connections);
        if let Some(x) = config.min_connections {
            pool_config = pool_config.min_connections(x);
        }

        Self {
            conn_config,
            pool_config,
        }
    }

    /// Создание пула из текущего билдера.
    pub async fn finish(self) -> Result<PgPool, sqlx::Error> {
        self.pool_config.connect_with(self.conn_config).await
    }

    /// Изменение опций для пула.
    pub fn set_options<F>(self, options_changer: F) -> Self
    where
        F: FnOnce(PoolOptions<Postgres>) -> PoolOptions<Postgres>,
    {
        let Self {
            conn_config,
            mut pool_config,
        } = self;
        pool_config = options_changer(pool_config);
        Self {
            conn_config,
            pool_config,
        }
    }
}
