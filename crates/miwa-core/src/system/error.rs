use config::ConfigError;

#[derive(thiserror::Error, Debug)]
pub enum SystemError {
    #[error("Component not found {0}")]
    ComponentMissing(String),
    #[error(transparent)]
    Other(#[from] anyhow::Error),
    #[error(transparent)]
    Config(#[from] ConfigError),
}

impl SystemError {
    pub fn component_missing(component: &str) -> SystemError {
        SystemError::ComponentMissing(component.to_owned())
    }
}

pub type SystemResult<T> = Result<T, SystemError>;
