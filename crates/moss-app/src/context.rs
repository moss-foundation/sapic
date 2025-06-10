use rustc_hash::FxHashMap;
use std::{
    any::{Any, TypeId},
    sync::Arc,
};

pub trait Global: 'static {
    fn global(ctx: Context) -> Arc<Self>;
}

pub trait GlobalProvider {
    fn global(ctx: &Context) -> &Self;
}

impl<T: Global> GlobalProvider for T {
    fn global(ctx: &Context) -> &Self {
        ctx.global_unchecked::<Self>()
    }
}

pub struct Context {
    globals_by_type: FxHashMap<TypeId, Arc<dyn Any>>,
}

impl Context {
    pub fn new() -> Self {
        Self {
            globals_by_type: FxHashMap::default(),
        }
    }

    pub fn global<T: Global>(&self) -> Option<&T> {
        self.globals_by_type
            .get(&TypeId::of::<T>())
            .and_then(|arc| arc.downcast_ref::<T>())
    }

    pub fn global_unchecked<T: Global>(&self) -> &T {
        self.globals_by_type
            .get(&TypeId::of::<T>())
            .and_then(|arc| arc.downcast_ref::<T>())
            .expect(&format!(
                "Global resource {} expected to be registered",
                std::any::type_name::<T>()
            ))
    }
}
