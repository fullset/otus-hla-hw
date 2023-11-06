use crate::db::PgConfig;

use std::{net::SocketAddr, path::Path};

use config as config_lib;

#[derive(Clone, Debug, serde::Deserialize)]
pub struct Config {
    pub http: SocketAddr,
    pub postgres: PgConfig,
    // TODO: logging config
    // pub logging: LoggingConfig,
}


impl Config {
    pub fn new<P: AsRef<Path>>(path: Option<P>) -> Result<Self, config_lib::ConfigError> {
        let mut cfg = config_lib::Config::builder();

        // Строим конфиг из файла
        if let Some(path) = path {
            let path = path.as_ref().to_str().ok_or_else(|| {
                config_lib::ConfigError::Message("invalid path utf-8 encoding".to_owned())
            })?;
            cfg = cfg.add_source(config_lib::File::with_name(path));
        }

        // Переопределяем при необходимости значениями из env_var
        cfg = cfg.add_source(
            config_lib::Environment::default()
                .prefix("OTUS")
                .separator("__")
                .ignore_empty(true),
        );

        let cfg = cfg.build()?;
        let cfg: Config = cfg.try_deserialize()?;

        Ok(cfg)
    }
}
