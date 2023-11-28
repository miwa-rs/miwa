use std::ops::Deref;

use config::{Config, Environment};
use serde::de::DeserializeOwned;

use crate::{FromSystemContext, SystemResult};

#[derive(Debug, Clone)]
pub struct SystemConfig {
    cfg: Config,
}

impl SystemConfig {
    pub fn default_cfg() -> SystemResult<Self> {
        let prefix = "MIWA".to_string();
        Ok(SystemConfig {
            cfg: Config::builder()
                .add_source(Environment::with_prefix(&prefix).separator("_"))
                .build()?,
        })
    }

    pub fn get<T: DeserializeOwned>(&self, key: &str) -> SystemResult<T> {
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

impl<'a, T: Configurable + DeserializeOwned> FromSystemContext<'a> for ExtensionConfig<T> {
    fn from_context(context: &'a crate::SystemContext) -> crate::SystemResult<Self> {
        let cfg = context.config().get::<T>(T::prefix())?;
        Ok(ExtensionConfig(cfg))
    }
}
