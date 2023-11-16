pub type Pool = sqlx::PgPool;

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

pub use sqlx::{self, PgPool};

use serde::Deserialize;
use sqlx::{pool::PoolOptions, postgres::PgConnectOptions, Postgres};

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
        let conn_config = PgConnectOptions::new()
            .host(&config.host)
            .port(config.port)
            .username(&config.user)
            .database(&config.database)
            .password(&config.password);

        let mut pool_config = PoolOptions::new().max_connections(config.max_connections);
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
}
