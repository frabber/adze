//! Stable, generational element identity.
//!
//! Raw indices are never used as references between elements (D5): an index is
//! reused the moment a slot is freed, which is exactly how the topological
//! naming problem gets in. Every ID carries the generation of the slot it was
//! minted for, so a stale ID compares unequal to whatever occupies the slot now
//! and resolves to `None`.
//!
//! IDs are `Ord` so they can key ordered containers (D7).

/// Shared surface of the generational IDs, so [`crate::arena::Arena`] can mint
/// them without knowing which element type it holds.
pub trait ElementId: Copy + Eq + Ord + core::fmt::Debug {
    /// Human-readable element kind, used in [`crate::Violation`] messages.
    const KIND: &'static str;

    fn from_raw(index: u32, generation: u32) -> Self;
    fn index(self) -> u32;
    fn generation(self) -> u32;
}

macro_rules! define_id {
    ($name:ident, $kind:literal, $doc:literal) => {
        #[doc = $doc]
        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub struct $name {
            index: u32,
            generation: u32,
        }

        impl ElementId for $name {
            const KIND: &'static str = $kind;

            fn from_raw(index: u32, generation: u32) -> Self {
                Self { index, generation }
            }

            fn index(self) -> u32 {
                self.index
            }

            fn generation(self) -> u32 {
                self.generation
            }
        }

        impl core::fmt::Debug for $name {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                write!(f, "{}{}v{}", $kind, self.index, self.generation)
            }
        }
    };
}

define_id!(
    VertexId,
    "v",
    "Identity of a vertex, stable across topology change."
);
define_id!(
    EdgeId,
    "e",
    "Identity of an edge, stable across topology change."
);
define_id!(
    FaceId,
    "f",
    "Identity of a face, stable across topology change."
);
