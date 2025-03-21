use std::sync::{Arc, Mutex};

use crate::{MiwaConfig, MiwaResult};

use super::container::Container;

#[derive(Clone)]
pub struct MiwaContext {
    component_id: String,
    services: Arc<Mutex<Container>>,
    config: MiwaConfig,
}

impl MiwaContext {
    pub(crate) fn new(component_id: String, config: MiwaConfig) -> MiwaContext {
        let ctx = MiwaContext {
            component_id,
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

    pub fn config(&self) -> &MiwaConfig {
        &self.config
    }

    pub fn component_id(&self) -> &str {
        &self.component_id
    }
}

pub trait FromMiwaContext<'a>
where
    Self: Sized,
{
    fn from_context(context: &'a MiwaContext) -> MiwaResult<Self>;
}

impl<'a> FromMiwaContext<'a> for &'a MiwaContext {
    fn from_context(context: &'a MiwaContext) -> MiwaResult<Self> {
        Ok(context)
    }
}
