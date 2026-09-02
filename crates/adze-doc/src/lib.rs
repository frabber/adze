//! The document: an ordered op log plus periodic snapshots (D2).
//!
//! Undo is replay from the nearest snapshot; a live op is re-evaluated when its
//! parameters change; post-op tweaks live in delta layers keyed by stable IDs.
//! Empty until M2.
