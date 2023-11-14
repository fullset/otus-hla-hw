mod cli;
mod config;
mod db;
mod error;
mod models;
mod state;
mod web;


use slog::{Drain, slog_o};
use anyhow::Context;
use futures::future;
use structopt::StructOpt;
use tokio_util::sync::CancellationToken;

use crate::{
    cli::Opt,
    config::Config,
    db::DbClient,
    state::{AppState, HealthChecker},
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let plain = slog_term::PlainSyncDecorator::new(std::io::stdout());
    let log = slog::Logger::root(
        slog_term::FullFormat::new(plain)
        .build().fuse(), slog_o!()
    );

    let _guard = slog_scope::set_global_logger(log);

    let cmd = Opt::from_args();
    let cfg = Config::new(cmd.config.as_deref())?;
    let cancel_token = CancellationToken::new();

    let app_state = init_app_state(cfg).await?;

    let result = if cmd.check {
        slog_scope::info!(
            "starting {} v{} check",
            env!("CARGO_PKG_NAME"),
            env!("CARGO_PKG_VERSION")
        );

        let healtcheck_result = app_state.healthcheck().await;
        if let Err(err) = &healtcheck_result {
            slog_scope::error!(
                "{} v{} check failed with error {}",
                env!("CARGO_PKG_NAME"),
                env!("CARGO_PKG_VERSION"),
                err
            );
        }

        healtcheck_result
    } else {
        slog_scope::info!(
            "starting {} v{}",
            env!("CARGO_PKG_NAME"),
            env!("CARGO_PKG_VERSION")
        );

        slog_scope::debug!("command line arguments: {:#?}", cmd);
        slog_scope::debug!("config: {:#?}", app_state.cfg());

        run(app_state, cancel_token).await
    };

    result
}

async fn run(app_state: AppState, cancel_token: CancellationToken) -> anyhow::Result<()> {
    let mut task_handles = Vec::with_capacity(2);

    // Start web (http)
    let web_handle = tokio::task::spawn({
        web::run(app_state.clone())
    });
    task_handles.push(web_handle);

    let (result, _idx, _other) = future::select_all(task_handles).await;
    match &result {
        Ok(Ok(_)) => slog_scope::crit!("unrecoverable error"),
        Ok(Err(err)) => slog_scope::crit!("unrecoverable error: {:#}", err),
        Err(err) => slog_scope::crit!("unrecoverable error: {}", err),
    }

    cancel_token.cancel();
    result??;

    Ok(())
}

async fn init_app_state(cfg: Config) -> anyhow::Result<AppState> {
    let db_client = DbClient::connect(&cfg.postgres)
        .await
        .context("making postgres client")?;

    Ok(AppState::new(
        cfg,
        db_client,
    ))
}
