use std::sync::atomic::AtomicU64;

static mut COMPONENT_ID: AtomicU64 = AtomicU64::new(0);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ComponentId {
    id: u64,
}

impl Default for ComponentId {
    fn default() -> Self {
        let id = unsafe { COMPONENT_ID.fetch_add(1, std::sync::atomic::Ordering::SeqCst) };
        Self { id }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_component_id() {
        let id1 = ComponentId::default();
        let id2 = ComponentId::default();
        let id3 = ComponentId::default();
        assert_ne!(id1.id, id2.id);
        assert_ne!(id2.id, id3.id);
        assert_ne!(id3.id, id1.id);
    }

    #[tokio::test]
    async fn test_component_id_thread() {
        // make 3 ids in futures
        // that all run in parallel
        // and assert that they are all different

        let id1 = tokio::spawn(async {
            let id = ComponentId::default();
            id.id
        });

        let id2 = tokio::spawn(async {
            let id = ComponentId::default();
            id.id
        });

        let id3 = tokio::spawn(async {
            let id = ComponentId::default();
            id.id
        });

        let (id1, id2, id3) = tokio::join!(id1, id2, id3);
        let (id1, id2, id3) = (id1.unwrap(), id2.unwrap(), id3.unwrap());

        assert_ne!(id1, id2);
        assert_ne!(id2, id3);
        assert_ne!(id3, id1);
    }
}
