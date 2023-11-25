use std::{
    ops::{Deref, DerefMut},
    sync::atomic::AtomicU64,
};

#[derive(Hash, Eq, PartialEq, Clone, Copy)]
pub struct Id {
    inner: u64,
}

impl Id {
    pub fn new(global: &AtomicU64) -> Self {
        Self {
            inner: global.fetch_add(1, std::sync::atomic::Ordering::SeqCst),
        }
    }
}

impl Deref for Id {
    type Target = u64;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for Id {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
