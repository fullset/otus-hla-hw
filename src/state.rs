use crate::db::sqlx::pool::PoolConnection;
use sqlx::Postgres;

use std::sync::Arc;

use async_trait::async_trait;

use crate::{config::Config, db::DbClient};

#[derive(Clone)]
pub struct AppState(Arc<AppStateInner>);

impl std::ops::Deref for AppState {
    type Target = AppStateInner;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[async_trait]
pub trait HealthChecker {
    async fn healthcheck(&self) -> anyhow::Result<()>;
}

#[async_trait]
impl HealthChecker for AppState {
    async fn healthcheck(&self) -> anyhow::Result<()> {
        self.0.healthcheck().await
    }
}

pub struct AppStateInner {
    //TODO: logger
    // logger: Logger,
    db_client: DbClient,
    cfg: Config,
}

impl AppState {
    pub fn new(
        cfg: Config,
        // logger: Logger,
        db_client: DbClient,
    ) -> Self {
        Self(Arc::new(AppStateInner {
            // logger,
            db_client,
            cfg,
        }))
    }

    pub fn cfg(&self) -> &Config {
        &self.0.cfg
    }

    //TODO: logger
    // pub fn logger(&self) -> &Logger {
    // &self.0.logger
    // }

    pub async fn acquire_db_connection(&self) -> Result<PoolConnection<Postgres>, sqlx::Error> {
        self.0.db_client.pool().acquire().await
    }
}

#[async_trait]
impl HealthChecker for AppStateInner {
    async fn healthcheck(&self) -> anyhow::Result<()> {
        slog_scope::info!("AppStateInner health check");
        // server::reexport::futures::try_join!(
        // self.db_client.healthcheck(),
        // )?;
        Ok(())
    }
}
