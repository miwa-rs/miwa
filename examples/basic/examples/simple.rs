use miwa::core::{Extension, Miwa, MiwaContext, MiwaResult};
use miwa::derive::{extension, interface, Injectable};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    Miwa::prepare()
        .build()?
        .add_extension(second_extension)
        .add_extension(first_extension)
        .start()
        .await?;

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

#[interface]
pub trait Service {}

#[derive(Clone, Debug, Injectable)]
pub struct ServiceImpl;

impl Service for ServiceImpl {}

#[extension(provides(ServiceRef))]
async fn first_extension(context: &MiwaContext) -> MiwaResult<FirstExtension> {
    context.register(ServiceRef::wrap(ServiceImpl));
    Ok(FirstExtension)
}

#[extension]
async fn second_extension(service: ServiceRef) -> MiwaResult<SecondExtension> {
    Ok(SecondExtension(service))
}

struct SecondExtension(ServiceRef);

#[async_trait::async_trait]
impl Extension for SecondExtension {
    async fn start(&self) -> MiwaResult<()> {
        Ok(())
    }

    async fn shutdown(&self) -> MiwaResult<()> {
        Ok(())
    }
}
