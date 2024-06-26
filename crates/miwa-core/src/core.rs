mod config;
mod container;
mod context;
mod error;
mod extension;
mod phase;

use std::{
    any::TypeId,
    collections::{HashMap, HashSet},
};

pub use self::config::{Configurable, ExtensionConfig, MiwaConfig};
use ::config::{Config, Environment, File};
pub use context::{FromMiwaContext, MiwaContext};
pub use error::{MiwaError, MiwaResult};
pub use extension::{
    ErasedExtensionFactory, Extension, ExtensionFactory, ExtensionGroup, IntoErasedExtensionFactory,
};
use petgraph::{algo::toposort, graph::NodeIndex, visit::NodeRef, Graph};
use tracing::{info, warn};

pub struct Miwa<P>(P);

pub struct Prepare {
    env: Option<Environment>,
    file: Option<String>,
}

impl Miwa<Prepare> {
    pub fn prepare() -> Self {
        Miwa(Prepare {
            env: None,
            file: None,
        })
    }

    pub fn with_env(mut self, prefix: &str) -> Self {
        self.0.env = Some(Environment::default().prefix(prefix).separator("_"));
        self
    }

    pub fn with_file(mut self, path: &str) -> Self {
        self.0.file = Some(path.to_owned());
        self
    }

    pub fn build(self) -> MiwaResult<Miwa<Build>> {
        let mut cfg = Config::builder();

        if let Some(env) = self.0.env {
            cfg = cfg.add_source(env);
        }

        if let Some(file) = self.0.file {
            cfg = cfg.add_source(File::with_name(file.as_str()));
        }

        let config = cfg.build()?;

        Ok(Miwa(Build {
            extensions: vec![],
            ctx: MiwaContext::new(MiwaConfig::with_config(config)?),
            registered: HashSet::new(),
        }))
    }
}

pub struct Build {
    extensions: Vec<Box<dyn ErasedExtensionFactory>>,
    ctx: MiwaContext,
    registered: HashSet<TypeId>,
}

impl Miwa<Build> {
    pub fn add_extension(mut self, extension: impl ExtensionFactory + 'static) -> Self {
        self.add_extension_internal(extension);
        self
    }
    fn add_extension_internal(&mut self, extension: impl ExtensionFactory + 'static) {
        let id = extension.id();
        if !self.0.registered.contains(&id) {
            self.0.registered.insert(id);
            let erased = IntoErasedExtensionFactory::from_extension_factory(extension);
            self.0.extensions.push(Box::new(erased));
        } else {
            warn!(
                "Skipping extension {}: which is already registered.",
                extension.name()
            )
        }
    }

    pub fn add_extension_group(&mut self, group: impl ExtensionGroup + 'static) -> &mut Self {
        group.register(&mut SystemGroup(self));
        self
    }

    pub async fn start(&mut self) -> MiwaResult<()> {
        let mut extensions = vec![];

        let sorted = self.build_graph();

        for idx in &sorted {
            let extension = &self.0.extensions[*idx];
            info!("initializing extension {}", extension.name());
            let ext = extension.build(&self.0.ctx).await?;
            extensions.push(ext);
        }
        for idx in &sorted {
            let extension = &self.0.extensions[*idx];
            let ext = &extensions[*idx];
            info!("Starting extension {}", extension.name());
            ext.start().await?;
        }

        let _ = tokio::signal::ctrl_c().await;

        for idx in sorted {
            let extension = &self.0.extensions[idx];
            let ext = &extensions[idx];
            info!("Stopping extension {}", extension.name());
            ext.shutdown().await?;
        }
        Ok(())
    }

    fn build_graph(&mut self) -> Vec<usize> {
        let mut graph = Graph::new();
        let mut nodes = HashMap::new();
        let mut providing = HashMap::new();
        let mut requiring = HashMap::new();

        for extension in &self.0.extensions {
            let node = graph.add_node(extension.name().to_string());
            nodes.insert(node.id(), node);
        }

        for (idx, extension) in self.0.extensions.iter().enumerate() {
            for ty in extension.exposes() {
                let value = providing.entry(idx).or_insert_with(Vec::new);
                value.push(ty);
            }
            for ty in extension.requires() {
                let value = requiring.entry(ty).or_insert_with(Vec::new);
                value.push(idx);
            }
        }

        for (id, node) in &nodes {
            let provided = providing.get(&id.index()).cloned().unwrap_or_default();

            for p in provided {
                let dependants = requiring.get(&p).cloned().unwrap_or_default();

                for d in dependants {
                    let index = NodeIndex::new(d);
                    let n = nodes.get(&index).unwrap();

                    graph.add_edge(node.id(), n.id(), ());
                }
            }
        }
        let sorted = toposort(&graph, None).unwrap();

        sorted.into_iter().map(|id| id.index()).collect()
    }
}

pub struct SystemGroup<'a>(&'a mut Miwa<Build>);

impl<'a> SystemGroup<'a> {
    pub fn add_extension(&mut self, extension: impl ExtensionFactory + 'static) -> &mut Self {
        self.0.add_extension_internal(extension);
        self
    }
}
