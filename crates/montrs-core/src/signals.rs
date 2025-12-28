use std::sync::Arc;
use parking_lot::RwLock;
use std::collections::HashSet;
use slotmap::{DefaultKey, SlotMap};
use once_cell::sync::Lazy;

/// A unique identifier for a reactive effect or subscriber.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SubscriberId(DefaultKey);

/// The reactive runtime that tracks dependencies.
pub struct ReactiveRuntime {
    subscribers: RwLock<SlotMap<DefaultKey, Arc<dyn Fn() + Send + Sync>>>,
    dependencies: RwLock<std::collections::HashMap<DefaultKey, HashSet<DefaultKey>>>,
}

impl ReactiveRuntime {
    pub fn new() -> Self {
        Self {
            subscribers: RwLock::new(SlotMap::new()),
            dependencies: RwLock::new(std::collections::HashMap::new()),
        }
    }
}

static RUNTIME: Lazy<ReactiveRuntime> = Lazy::new(ReactiveRuntime::new);

/// A reactive signal that holds a value of type T.
pub struct Signal<T> {
    value: Arc<RwLock<T>>,
    subscribers: Arc<RwLock<HashSet<DefaultKey>>>,
}

impl<T: Send + Sync + 'static> Signal<T> {
    pub fn new(val: T) -> Self {
        Self {
            value: Arc::new(RwLock::new(val)),
            subscribers: Arc::new(RwLock::new(HashSet::new())),
        }
    }

    pub fn get(&self) -> T 
    where T: Clone {
        // Register current effect as a subscriber (Simplified for v0.1)
        self.value.read().clone()
    }

    pub fn set(&self, val: T) {
        {
            let mut writer = self.value.write();
            *writer = val;
        }
        self.notify();
    }

    pub fn mutate<F: FnOnce(&mut T)>(&self, f: F) {
        {
            let mut writer = self.value.write();
            f(&mut *writer);
        }
        self.notify();
    }

    fn notify(&self) {
        let subs = self.subscribers.read();
        for &sub_key in subs.iter() {
            let runtime_subs = RUNTIME.subscribers.read();
            if let Some(effect) = runtime_subs.get(sub_key) {
                effect();
            }
        }
    }
}

impl<T: Clone> Clone for Signal<T> {
    fn clone(&self) -> Self {
        Self {
            value: Arc::clone(&self.value),
            subscribers: Arc::clone(&self.subscribers),
        }
    }
}
