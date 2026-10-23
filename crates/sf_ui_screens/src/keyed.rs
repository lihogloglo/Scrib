//! Keyed child lists: a view-model's ordered keys mirrored as child entities of one parent,
//! with the fewest structural changes (the React/Unity `ListView` keyed reconciliation).
//!
//! A key that stays keeps its entity (and everything under it: focus, hover, scroll); a new
//! key spawns one child, a gone key despawns its child, and the parent's child order is
//! rewritten only when the order changed. What to redraw inside a kept child is the caller's
//! call (row versions say when).

use std::collections::HashMap;
use std::hash::Hash;

use bevy_ecs::prelude::*;

/// What one [`KeyedChildren::sync`] did.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct KeyedSync {
    pub spawned: usize,
    pub despawned: usize,
    /// The child order was rewritten.
    pub reordered: bool,
}

impl KeyedSync {
    pub fn is_noop(&self) -> bool {
        *self == Self::default()
    }
}

/// The keyed children of `parent`. The parent holds no other children.
#[derive(Clone, Debug)]
pub struct KeyedChildren<K> {
    parent: Entity,
    order: Vec<(K, Entity)>,
}

impl<K: Clone + Eq + Hash> KeyedChildren<K> {
    pub fn new(parent: Entity) -> Self {
        Self {
            parent,
            order: Vec::new(),
        }
    }

    pub fn parent(&self) -> Entity {
        self.parent
    }

    /// The child showing `key`.
    pub fn get(&self, key: &K) -> Option<Entity> {
        self.order.iter().find(|(k, _)| k == key).map(|(_, e)| *e)
    }

    /// The key a child shows.
    pub fn key_of(&self, entity: Entity) -> Option<&K> {
        self.order
            .iter()
            .find(|(_, e)| *e == entity)
            .map(|(k, _)| k)
    }

    /// Keys and children in display order.
    pub fn iter(&self) -> impl Iterator<Item = (&K, Entity)> {
        self.order.iter().map(|(k, e)| (k, *e))
    }

    pub fn len(&self) -> usize {
        self.order.len()
    }

    pub fn is_empty(&self) -> bool {
        self.order.is_empty()
    }

    /// Brings the children in line with `keys` (display order; duplicates after the first are
    /// ignored). `spawn` makes the entity of a new key; this parents it.
    pub fn sync(
        &mut self,
        commands: &mut Commands,
        keys: impl IntoIterator<Item = K>,
        mut spawn: impl FnMut(&mut Commands, &K) -> Entity,
    ) -> KeyedSync {
        let previous = std::mem::take(&mut self.order);
        let mut old: HashMap<K, Entity> = previous.iter().cloned().collect();
        let mut out = KeyedSync::default();
        let mut order: Vec<(K, Entity)> = Vec::new();
        let mut spawned = Vec::new();
        for key in keys {
            if order.iter().any(|(k, _)| *k == key) {
                continue;
            }
            let entity = if let Some(e) = old.remove(&key) {
                e
            } else {
                let e = spawn(commands, &key);
                commands.entity(e).insert(ChildOf(self.parent));
                spawned.push(e);
                e
            };
            order.push((key, entity));
        }
        out.spawned = spawned.len();
        for entity in old.values() {
            out.despawned += 1;
            commands.entity(*entity).despawn();
        }
        // Where the children are now: kept ones in their previous order, new ones appended.
        let now = previous
            .iter()
            .map(|(_, e)| *e)
            .filter(|e| !old.values().any(|gone| gone == e))
            .chain(spawned);
        let wanted: Vec<Entity> = order.iter().map(|(_, e)| *e).collect();
        if !now.eq(wanted.iter().copied()) {
            out.reordered = true;
            commands.entity(self.parent).replace_children(&wanted);
        }
        self.order = order;
        out
    }

    /// Despawns every child.
    pub fn clear(&mut self, commands: &mut Commands) {
        for (_, entity) in self.order.drain(..) {
            commands.entity(entity).despawn();
        }
    }
}
