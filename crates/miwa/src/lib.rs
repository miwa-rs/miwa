pub mod derive {
    pub use miwa_macros::{extension, ExtensionConfig};
}

pub mod core {
    pub use miwa_core::{
        Configurable, Dep, Extension, ExtensionConfig, ExtensionFactory, ExtensionGroup,
        FromMiwaContext, Miwa, MiwaConfig, MiwaContext, MiwaError, MiwaHandle, MiwaId, MiwaResult,
    };
}
