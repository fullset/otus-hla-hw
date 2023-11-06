use crate::db::PgConnection;
use std::sync::Arc;

use async_trait::async_trait;

use crate::{config::Config, db::DbClient};

#[derive(Clone)]
pub struct AppState {
    inner: Arc<AppStateInner>,
}

#[async_trait]
pub trait HealthChecker {
    async fn healthcheck(&self) -> anyhow::Result<()>;
}

#[async_trait]
impl HealthChecker for AppState {
    async fn healthcheck(&self) -> anyhow::Result<()> {
        self.inner.healthcheck().await
    }
}

struct AppStateInner {
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
    ) -> Self
    {
        Self {
            inner: Arc::new(AppStateInner {
                // logger,
                db_client,
                cfg,
            }),
        }
    }

    pub fn cfg(&self) -> &Config {
        &self.inner.cfg
    }

    //TODO: logger
    // pub fn logger(&self) -> &Logger {
        // &self.inner.logger
    // }

    pub fn db(&self) -> &DbClient {
        &self.inner.db_client
    }

    pub async fn acquire_db_connection(&self) -> Result<PgConnection, sqlx::Error> {
        self.inner
            .db_client
            .pool()
            .acquire()
            .await
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