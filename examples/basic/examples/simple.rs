use std::sync::Arc;

use miwa::core::{Dep, Extension, Miwa, MiwaContext, MiwaResult};
use miwa::derive::extension;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let mut handle = Miwa::prepare()
        .build()?
        .add_extension(second_extension)
        .add_extension(first_extension)
        .start()
        .await?;

    handle.wait().await?;

    Ok(())
}

struct FirstExtension;

#[async_trait::async_trait]
impl Extension for FirstExtension {
    async fn start(&self) -> MiwaResult<()> {
        Ok(())
    }

    async fn shutdown(&self) -> MiwaResult<()> {
        Ok(())
    }
}

pub trait Service: Send + Sync {
    fn hello(&self);
}

pub type ServiceRef = Arc<dyn Service>;

#[derive(Clone, Debug)]
pub struct ServiceImpl;

impl Service for ServiceImpl {
    fn hello(&self) {
        println!("Hello Service")
    }
}

#[extension(provides(ServiceRef))]
async fn first_extension(context: &MiwaContext) -> MiwaResult<FirstExtension> {
    context.register_trait::<dyn Service>(Arc::new(ServiceImpl));
    Ok(FirstExtension)
}

#[extension]
async fn second_extension(Dep(service): Dep<ServiceRef>) -> MiwaResult<SecondExtension> {
    Ok(SecondExtension(service))
}

struct SecondExtension(ServiceRef);

#[async_trait::async_trait]
impl Extension for SecondExtension {
    async fn start(&self) -> MiwaResult<()> {
        self.0.hello();
        Ok(())
    }

    async fn shutdown(&self) -> MiwaResult<()> {
        Ok(())
    }
}
