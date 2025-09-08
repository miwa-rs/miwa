use std::{
    any::{Any, TypeId},
    collections::HashMap,
    sync::Arc,
};

pub struct Container {
    services: HashMap<TypeId, DynSingleton>,
}

impl Container {
    pub(crate) fn new() -> Container {
        Container {
            services: HashMap::new(),
        }
    }
    pub fn resolve<T: Clone + 'static>(&self) -> Option<T> {
        self.services.get(&TypeId::of::<T>())?.as_owned()
    }

    pub(crate) fn register<T: Send + 'static>(&mut self, resource: T) {
        self.services
            .insert(TypeId::of::<T>(), DynSingleton::new(resource));
    }

    pub(crate) fn register_trait<T: ?Sized + Send + Sync + 'static>(&mut self, resource: Arc<T>) {
        self.services
            .insert(TypeId::of::<Arc<T>>(), DynSingleton::new(resource));
    }
}

pub struct DynSingleton(Box<dyn Any + Send>);

impl DynSingleton {
    pub fn new<T: Send + 'static>(resource: T) -> DynSingleton {
        DynSingleton(Box::new(resource))
    }
    pub fn as_owned<T: Clone + 'static>(&self) -> Option<T> {
        self.0.downcast_ref::<T>().cloned()
    }
}
