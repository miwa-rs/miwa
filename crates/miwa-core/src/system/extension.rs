use std::any::TypeId;

use super::{error::SystemResult, SystemContext, SystemGroup};

#[async_trait::async_trait]
pub trait Extension {
    async fn start(&self) -> SystemResult<()>;
    async fn shutdown(&self) -> SystemResult<()>;
}

#[async_trait::async_trait]
pub trait ExtensionFactory: Send + Sync
where
    Self: 'static,
{
    fn name(&self) -> &str;
    async fn build(&self, context: &SystemContext) -> SystemResult<Box<dyn Extension>>;

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
pub trait ErasedExtensionFactory {
    fn name(&self) -> &str;
    async fn build(&self, context: &SystemContext) -> SystemResult<Box<dyn Extension>>;

    fn exposes(&self) -> Vec<TypeId> {
        vec![]
    }

    fn requires(&self) -> Vec<TypeId> {
        vec![]
    }
}

pub struct IntoErasedExtensionFactory<F> {
    factory: F,
}

impl<F> IntoErasedExtensionFactory<F> {
    pub fn from_extension_factory(factory: F) -> Self {
        IntoErasedExtensionFactory { factory }
    }
}

#[async_trait::async_trait]
impl<F> ErasedExtensionFactory for IntoErasedExtensionFactory<F>
where
    F: ExtensionFactory,
{
    fn name(&self) -> &str {
        self.factory.name()
    }
    async fn build(&self, context: &SystemContext) -> SystemResult<Box<dyn Extension>> {
        self.factory.build(context).await
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
