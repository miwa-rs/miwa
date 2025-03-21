use std::ops::Deref;

use config::{Config, ConfigError, Environment, FileFormat, Format, Source};
use serde::de::DeserializeOwned;
use serde_json::Value;

use crate::{FromMiwaContext, MiwaResult};

#[derive(Debug, Clone)]
pub struct MiwaConfig {
    cfg: Config,
}

impl MiwaConfig {
    pub fn default_cfg() -> MiwaResult<Self> {
        let prefix = "MIWA".to_string();
        Ok(MiwaConfig {
            cfg: Config::builder()
                .add_source(Environment::with_prefix(&prefix).separator("_"))
                .build()?,
        })
    }

    pub fn with_config(cfg: Config) -> MiwaResult<Self> {
        Ok(MiwaConfig { cfg })
    }

    pub fn get<T: DeserializeOwned>(&self, key: &str) -> MiwaResult<T> {
        self.cfg.get(key).map(Ok)?
    }
}

pub struct ExtensionConfig<T>(pub T);

impl<T> Deref for ExtensionConfig<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub trait Configurable {
    fn prefix() -> &'static str;
}

impl<'a, T: Configurable + DeserializeOwned> FromMiwaContext<'a> for ExtensionConfig<T> {
    fn from_context(context: &'a crate::MiwaContext) -> crate::MiwaResult<Self> {
        let cfg = context.config().get::<T>(T::prefix())?;
        Ok(ExtensionConfig(cfg))
    }
}

#[derive(Debug, Clone)]
pub struct JsonSource(pub Value);

impl Source for JsonSource {
    fn clone_into_box(&self) -> Box<dyn Source + Send + Sync> {
        Box::new((*self).clone())
    }

    fn collect(&self) -> Result<config::Map<String, config::Value>, ConfigError> {
        let json_str = serde_json::to_string(&self.0).unwrap();

        FileFormat::Json
            .parse(None, &json_str)
            .map_err(ConfigError::Foreign)
    }
}
