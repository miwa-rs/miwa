mod config;
mod container;
mod context;
mod error;
mod extension;

use std::{
    any::TypeId,
    collections::{HashMap, HashSet},
};

pub use config::{Configurable, ExtensionConfig, SystemConfig};
pub use context::{FromSystemContext, SystemContext};
pub use error::{SystemError, SystemResult};
pub use extension::{
    ErasedExtensionFactory, Extension, ExtensionFactory, ExtensionGroup, IntoErasedExtensionFactory,
};
use petgraph::{algo::toposort, graph::NodeIndex, visit::NodeRef, Graph};
use tracing::{info, warn};

pub struct System {
    extensions: Vec<Box<dyn ErasedExtensionFactory>>,
    ctx: SystemContext,
    registered: HashSet<TypeId>,
}

impl System {
    pub fn prepare() -> SystemResult<Self> {
        let config = SystemConfig::default_cfg()?;

        Ok(System {
            extensions: vec![],
            ctx: SystemContext::new(config.clone()),
            registered: HashSet::new(),
        })
    }

    pub fn add_extension(&mut self, extension: impl ExtensionFactory + 'static) -> &mut Self {
        let id = extension.id();
        if !self.registered.contains(&id) {
            self.registered.insert(id);
            let erased = IntoErasedExtensionFactory::from_extension_factory(extension);
            self.extensions.push(Box::new(erased));
        } else {
            warn!(
                "Skipping extension {}: which is already registered.",
                extension.name()
            )
        }

        self
    }

    pub fn add_extension_group(&mut self, group: impl ExtensionGroup + 'static) -> &mut Self {
        group.register(&mut SystemGroup(self));
        self
    }

    pub async fn start(&mut self) -> SystemResult<()> {
        let mut extensions = vec![];

        let sorted = self.build_graph();

        for idx in &sorted {
            let extension = &self.extensions[*idx];
            info!("initializing extension {}", extension.name());
            let ext = extension.build(&self.ctx).await?;
            extensions.push(ext);
        }
        for idx in &sorted {
            let extension = &self.extensions[*idx];
            let ext = &extensions[*idx];
            info!("Starting extension {}", extension.name());
            ext.start().await?;
        }

        let _ = tokio::signal::ctrl_c().await;

        for idx in sorted {
            let extension = &self.extensions[idx];
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

        for extension in &self.extensions {
            let node = graph.add_node(extension.name().to_string());
            nodes.insert(node.id(), node);
        }

        for (idx, extension) in self.extensions.iter().enumerate() {
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

pub struct SystemGroup<'a>(&'a mut System);

impl<'a> SystemGroup<'a> {
    pub fn add_extension(&mut self, extension: impl ExtensionFactory + 'static) -> &mut Self {
        self.0.add_extension(extension);
        self
    }
}
