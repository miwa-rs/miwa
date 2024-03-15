use config::ConfigError;

#[derive(thiserror::Error, Debug)]
pub enum MiwaError {
    #[error("Component not found {0}")]
    ComponentMissing(String),
    #[error(transparent)]
    Other(#[from] anyhow::Error),
    #[error(transparent)]
    Config(#[from] ConfigError),
}

impl MiwaError {
    pub fn component_missing(component: &str) -> MiwaError {
        MiwaError::ComponentMissing(component.to_owned())
    }
}

pub type MiwaResult<T> = Result<T, MiwaError>;
