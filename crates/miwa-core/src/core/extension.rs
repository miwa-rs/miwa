use std::any::TypeId;

use super::{error::MiwaResult, MiwaContext, SystemGroup};

#[async_trait::async_trait]
pub trait Extension : Send {
    async fn start(&self) -> MiwaResult<()>;
    async fn shutdown(&self) -> MiwaResult<()>;
}

#[async_trait::async_trait]
pub trait ExtensionFactory: Send + Sync
where
    Self: 'static,
{
    fn name(&self) -> &str;
    async fn init(&self, context: &MiwaContext) -> MiwaResult<Box<dyn Extension>>;

    fn exposes(&self) -> Vec<TypeId> {
        vec![]
    }

    fn requires(&self) -> Vec<TypeId> {
        vec![]
    }

    fn id(&self) -> TypeId {
        TypeId::of::<Self>()
    }
}

#[async_trait::async_trait]
pub(crate) trait InternalExtensionFactory : Send {
    fn name(&self) -> &str;
    async fn init(&self, context: &MiwaContext) -> MiwaResult<Box<dyn Extension>>;

    fn exposes(&self) -> Vec<TypeId> {
        vec![]
    }

    fn requires(&self) -> Vec<TypeId> {
        vec![]
    }
}

pub(crate) struct IntoInternalExtensionFactory<F> {
    factory: F,
}

impl<F> IntoInternalExtensionFactory<F> {
    pub fn from_extension_factory(factory: F) -> Self {
        IntoInternalExtensionFactory { factory }
    }
}

#[async_trait::async_trait]
impl<F> InternalExtensionFactory for IntoInternalExtensionFactory<F>
where
    F: ExtensionFactory,
{
    fn name(&self) -> &str {
        self.factory.name()
    }
    async fn init(&self, context: &MiwaContext) -> MiwaResult<Box<dyn Extension>> {
        self.factory.init(context).await
    }
    fn exposes(&self) -> Vec<TypeId> {
        self.factory.exposes()
    }

    fn requires(&self) -> Vec<TypeId> {
        self.factory.requires()
    }
}

pub trait ExtensionGroup {
    fn register(&self, system: &mut SystemGroup);
}
