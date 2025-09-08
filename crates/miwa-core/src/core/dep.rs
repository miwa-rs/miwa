use crate::MiwaError;

use super::{FromMiwaContext, MiwaContext, MiwaId, MiwaResult};

pub struct Dep<T>(pub T);

impl<T: Clone + 'static> FromMiwaContext<'_> for Dep<T> {
    fn from_context(context: &MiwaContext) -> MiwaResult<Self> {
        let dep = context.resolve::<T>().ok_or_else(|| {
            MiwaError::ComponentMissing(format!(
                "Failed to resolve dependency: {}",
                std::any::type_name::<T>()
            ))
        })?;
        Ok(Dep(dep))
    }
}

impl<T: 'static> MiwaId for Dep<T> {
    fn component_id() -> std::any::TypeId {
        std::any::TypeId::of::<T>()
    }
}
