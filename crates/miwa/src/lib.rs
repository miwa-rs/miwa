pub mod derive {
    pub use miwa_macros::{extension, ExtensionConfig, Injectable};
}

pub mod core {
    pub use miwa_core::{
        Extension, ExtensionConfig, ExtensionFactory, ExtensionGroup, FromSystemContext, System,
        SystemConfig, SystemContext, SystemError, SystemResult,
    };
}
