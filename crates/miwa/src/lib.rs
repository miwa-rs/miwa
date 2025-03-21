pub mod derive {
    pub use miwa_macros::{extension, interface, ExtensionConfig, Injectable};
}

pub mod core {
    pub use miwa_core::{
        Configurable, Extension, ExtensionConfig, ExtensionFactory, ExtensionGroup,
        FromMiwaContext, Miwa, MiwaConfig, MiwaContext, MiwaError, MiwaHandle, MiwaResult,
    };
}
