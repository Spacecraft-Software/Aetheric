// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2026 Mohamed Hammad & Spacecraft Software

use crate::Stratum;
use dashmap::DashMap;
use std::sync::Arc;

/// Registry of active snapshots. Each snapshot is an `Arc<Stratum>` clone.
#[derive(Debug)]
pub struct SnapshotRegistry {
    inner: DashMap<u32, Arc<Stratum>>,
    next_id: std::sync::atomic::AtomicU32,
}

impl SnapshotRegistry {
    /// Create a new empty registry.
    #[must_use]
    pub fn new() -> Self {
        Self {
            inner: DashMap::new(),
            next_id: std::sync::atomic::AtomicU32::new(1),
        }
    }

    /// Create a new snapshot handle from a `Stratum`. Returns the handle id.
    pub fn create(&self, stratum: Arc<Stratum>) -> u32 {
        let id = self
            .next_id
            .fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        self.inner.insert(id, stratum);
        id
    }

    /// Release a snapshot handle. Returns `true` if the id existed.
    pub fn release(&self, id: u32) -> bool {
        self.inner.remove(&id).is_some()
    }

    /// Get a snapshot by id.
    #[must_use]
    pub fn get(&self, id: u32) -> Option<Arc<Stratum>> {
        self.inner.get(&id).map(|entry| Arc::clone(&*entry))
    }
}

impl Default for SnapshotRegistry {
    fn default() -> Self {
        Self::new()
    }
}
