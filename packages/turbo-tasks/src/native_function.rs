use crate::{self as turbo_tasks, task::NativeTaskFn, NodeRef};
use anyhow::Result;
use std::hash::Hash;

#[turbo_tasks::value]
pub struct NativeFunction {
    pub name: String,
    #[trace_ignore]
    pub bind_fn: Box<dyn (Fn(Vec<NodeRef>) -> Result<NativeTaskFn>) + Send + Sync + 'static>,
}

#[turbo_tasks::value_impl]
impl NativeFunction {
    #[turbo_tasks::constructor]
    pub fn new(
        name: String,
        bind_fn: impl (Fn(Vec<NodeRef>) -> Result<NativeTaskFn>) + Send + Sync + 'static,
    ) -> Self {
        Self {
            name,
            bind_fn: Box::new(bind_fn),
        }
    }

    pub fn bind(&self, inputs: Vec<NodeRef>) -> Result<NativeTaskFn> {
        (self.bind_fn)(inputs)
    }
}

impl PartialEq for &'static NativeFunction {
    fn eq(&self, other: &Self) -> bool {
        (*self as *const NativeFunction) == (*other as *const NativeFunction)
    }
}

impl Hash for &'static NativeFunction {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        Hash::hash(&(*self as *const NativeFunction), state);
    }
}
