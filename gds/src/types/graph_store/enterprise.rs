use std::sync::Arc;

/// Lifecycle stage for an operational event emitted by a graph store.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GraphStoreOperationalStage {
    ViewRequested,
    ViewMaterialized,
    ProcedureDispatched,
    ProcedureCompleted,
    MutationApplied,
}

/// A lightweight event payload for enterprise graph-store observability.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GraphStoreOperationalEvent {
    pub stage: GraphStoreOperationalStage,
    pub detail: String,
}

/// Receives operational events emitted by graph-store-backed execution.
pub trait GraphStoreInstrumentation: Send + Sync {
    fn observe(&self, event: &GraphStoreOperationalEvent);
}

/// Hooks that can pre- or post-process operational events before GraphFrame
/// continues into the next execution stage.
pub trait GraphStoreOperationalHooks: Send + Sync {
    fn before(&self, _event: &GraphStoreOperationalEvent) -> bool {
        true
    }

    fn after(&self, _event: &GraphStoreOperationalEvent) {}
}

impl<T> GraphStoreInstrumentation for Arc<T>
where
    T: GraphStoreInstrumentation + ?Sized,
{
    fn observe(&self, event: &GraphStoreOperationalEvent) {
        (**self).observe(event);
    }
}

impl<T> GraphStoreOperationalHooks for Arc<T>
where
    T: GraphStoreOperationalHooks + ?Sized,
{
    fn before(&self, event: &GraphStoreOperationalEvent) -> bool {
        (**self).before(event)
    }

    fn after(&self, event: &GraphStoreOperationalEvent) {
        (**self).after(event);
    }
}
