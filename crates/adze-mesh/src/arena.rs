//! Generational arena backing every element store.
//!
//! Iteration is slot order, which is the insertion order for a mesh that has
//! never had an element removed and a deterministic order in every other case
//! (D7). Freed slots are reused last-in-first-out from an explicit free list,
//! so two identical op sequences produce identical IDs on every platform.

use core::marker::PhantomData;

use crate::id::ElementId;

struct Slot<T> {
    generation: u32,
    value: Option<T>,
}

/// A store of elements of type `T` addressed by generational IDs of type `I`.
pub struct Arena<I, T> {
    slots: Vec<Slot<T>>,
    free: Vec<u32>,
    live: usize,
    _id: PhantomData<fn() -> I>,
}

impl<I, T> Default for Arena<I, T> {
    fn default() -> Self {
        Self {
            slots: Vec::new(),
            free: Vec::new(),
            live: 0,
            _id: PhantomData,
        }
    }
}

impl<I: ElementId, T> Arena<I, T> {
    pub fn new() -> Self {
        Self::default()
    }

    /// Number of live elements. Freed slots do not count.
    pub fn len(&self) -> usize {
        self.live
    }

    pub fn is_empty(&self) -> bool {
        self.live == 0
    }

    pub fn insert(&mut self, value: T) -> I {
        match self.free.pop() {
            Some(index) => {
                let slot = &mut self.slots[index as usize];
                debug_assert!(slot.value.is_none());
                slot.value = Some(value);
                self.live += 1;
                I::from_raw(index, slot.generation)
            }
            None => {
                let index = u32::try_from(self.slots.len()).expect("arena index overflow");
                self.slots.push(Slot {
                    generation: 1,
                    value: Some(value),
                });
                self.live += 1;
                I::from_raw(index, 1)
            }
        }
    }

    fn slot(&self, id: I) -> Option<&Slot<T>> {
        let slot = self.slots.get(id.index() as usize)?;
        (slot.generation == id.generation()).then_some(slot)
    }

    pub fn contains(&self, id: I) -> bool {
        self.get(id).is_some()
    }

    pub fn get(&self, id: I) -> Option<&T> {
        self.slot(id)?.value.as_ref()
    }

    pub fn get_mut(&mut self, id: I) -> Option<&mut T> {
        let slot = self.slots.get_mut(id.index() as usize)?;
        if slot.generation != id.generation() {
            return None;
        }
        slot.value.as_mut()
    }

    /// Frees the slot and bumps its generation, invalidating every outstanding
    /// copy of `id`.
    pub fn remove(&mut self, id: I) -> Option<T> {
        let slot = self.slots.get_mut(id.index() as usize)?;
        if slot.generation != id.generation() {
            return None;
        }
        let value = slot.value.take()?;
        slot.generation = slot.generation.wrapping_add(1).max(1);
        self.free.push(id.index());
        self.live -= 1;
        Some(value)
    }

    /// Live elements in slot order.
    pub fn iter(&self) -> impl Iterator<Item = (I, &T)> {
        self.slots.iter().enumerate().filter_map(|(index, slot)| {
            let value = slot.value.as_ref()?;
            let index = u32::try_from(index).expect("arena index overflow");
            Some((I::from_raw(index, slot.generation), value))
        })
    }

    /// IDs of live elements in slot order.
    pub fn ids(&self) -> impl Iterator<Item = I> + '_ {
        self.iter().map(|(id, _)| id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::id::VertexId;

    #[test]
    fn stale_ids_do_not_resolve_to_the_slot_that_replaced_them() {
        let mut arena: Arena<VertexId, u32> = Arena::new();
        let a = arena.insert(10);
        assert_eq!(arena.remove(a), Some(10));
        let b = arena.insert(20);

        assert_eq!(a.index(), b.index(), "the slot should have been reused");
        assert_ne!(a, b);
        assert_eq!(arena.get(a), None);
        assert_eq!(arena.get(b), Some(&20));
        assert_eq!(arena.len(), 1);
    }

    #[test]
    fn iteration_is_slot_order() {
        let mut arena: Arena<VertexId, u32> = Arena::new();
        let ids: Vec<_> = (0..5).map(|n| arena.insert(n)).collect();
        arena.remove(ids[1]);
        arena.remove(ids[3]);

        let seen: Vec<u32> = arena.iter().map(|(_, v)| *v).collect();
        assert_eq!(seen, vec![0, 2, 4]);
    }
}
