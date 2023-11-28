use std::sync::{Arc, Mutex};

use crate::{SystemConfig, SystemResult};

use super::container::Container;

#[derive(Clone)]
pub struct SystemContext {
    services: Arc<Mutex<Container>>,
    config: SystemConfig,
}

impl SystemContext {
    pub(crate) fn new(config: SystemConfig) -> SystemContext {
        let ctx = SystemContext {
            services: Arc::new(Mutex::new(Container::new())),
            config,
        };

        ctx.register(ctx.clone());

        ctx
    }
    pub fn resolve<T: Clone + 'static>(&self) -> Option<T> {
        let services = self.services.lock().unwrap();
        services.resolve()
    }

    pub fn register<T>(&self, resource: T)
    where
        T: Send + 'static,
    {
        let mut services = self.services.lock().unwrap();
        services.register(resource);
    }

    pub fn config(&self) -> &SystemConfig {
        &self.config
    }
}

pub trait FromSystemContext<'a>
where
    Self: Sized,
{
    fn from_context(context: &'a SystemContext) -> SystemResult<Self>;
}

impl<'a> FromSystemContext<'a> for &'a SystemContext {
    fn from_context(context: &'a SystemContext) -> SystemResult<Self> {
        Ok(context)
    }
}
