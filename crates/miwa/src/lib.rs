pub mod derive {
    pub use miwa_macros::{extension, ExtensionConfig, Injectable};
}

pub mod core {
    pub use miwa_core::{
        Configurable, Extension, ExtensionConfig, ExtensionFactory, ExtensionGroup,
        FromMiwaContext, Miwa, MiwaConfig, MiwaContext, MiwaError, MiwaResult,
    };
}
