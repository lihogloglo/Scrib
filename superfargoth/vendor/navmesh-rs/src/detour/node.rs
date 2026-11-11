//! Search nodes, the node pool and the open list (`DetourNode`), with upstream's
//! hash, allocation order and heap tie-breaks.

use alloc::vec::Vec;

use super::PolyRef;
use crate::Error;

pub(crate) const NODE_OPEN: u8 = 0x01;
pub(crate) const NODE_CLOSED: u8 = 0x02;
pub(crate) const NODE_PARENT_DETACHED: u8 = 0x04;
const NULL_IDX: u16 = 0xffff;

/// Whether a new path total fails to improve a node's total (upstream's
/// `total >= node.total`). Written as "not strictly less" so a NaN total (from
/// coordinates whose distances overflow) never counts as an improvement: with
/// upstream's comparison a NaN node reopens forever. Identical for non-NaN values.
#[inline]
#[allow(clippy::neg_cmp_op_on_partial_ord)]
pub(crate) fn no_better(total: f32, old: f32) -> bool {
    !(total < old)
}

#[derive(Clone, Copy, Debug, Default)]
pub(crate) struct Node {
    pub pos: [f32; 3],
    pub cost: f32,
    pub total: f32,
    /// Parent node index + 1 (0: none).
    pub pidx: u32,
    pub state: u8,
    pub flags: u8,
    pub id: PolyRef,
}

#[inline]
fn hash_ref(a: PolyRef) -> u32 {
    #[cfg(feature = "polyref64")]
    {
        let mut a = a;
        a = (!a).wrapping_add(a << 18);
        a ^= a >> 31;
        a = a.wrapping_mul(21);
        a ^= a >> 11;
        a = a.wrapping_add(a << 6);
        a ^= a >> 22;
        a as u32
    }
    #[cfg(not(feature = "polyref64"))]
    {
        let mut a = a;
        a = a.wrapping_add(!(a << 15));
        a ^= a >> 10;
        a = a.wrapping_add(a << 3);
        a ^= a >> 6;
        a = a.wrapping_add(!(a << 11));
        a ^= a >> 16;
        a
    }
}

pub(crate) struct NodePool {
    pub nodes: Vec<Node>,
    first: Vec<u16>,
    next: Vec<u16>,
    max_nodes: usize,
    count: usize,
}

fn vec_of<T: Clone>(n: usize, v: T) -> Result<Vec<T>, Error> {
    let mut out = Vec::new();
    out.try_reserve_exact(n)
        .map_err(|_| Error::AllocationFailed)?;
    out.resize(n, v);
    Ok(out)
}

impl NodePool {
    pub fn new(max_nodes: usize, hash_size: usize) -> Result<NodePool, Error> {
        Ok(NodePool {
            nodes: vec_of(max_nodes, Node::default())?,
            first: vec_of(hash_size, NULL_IDX)?,
            next: vec_of(max_nodes, NULL_IDX)?,
            max_nodes,
            count: 0,
        })
    }

    pub fn clear(&mut self) {
        self.first.fill(NULL_IDX);
        self.count = 0;
    }

    /// A new empty pool with this pool's node and hash sizes.
    /// The nodes in use.
    pub fn used(&self) -> &[Node] {
        &self.nodes[..self.count.min(self.nodes.len())]
    }

    pub fn same_shape(&self) -> Result<NodePool, Error> {
        NodePool::new(self.max_nodes, self.first.len())
    }

    /// The node capacity.
    pub fn max_nodes(&self) -> usize {
        self.max_nodes
    }

    #[inline]
    fn bucket(&self, id: PolyRef) -> usize {
        (hash_ref(id) as usize) & (self.first.len() - 1)
    }

    /// Every node for `id` (any state), most recent first, at most `max`.
    pub fn find_nodes(&self, id: PolyRef, out: &mut Vec<usize>, max: usize) {
        out.clear();
        let mut i = self.first[self.bucket(id)];
        while i != NULL_IDX {
            if self.nodes[i as usize].id == id {
                if out.len() >= max {
                    return;
                }
                out.push(i as usize);
            }
            i = self.next[i as usize];
        }
    }

    pub fn find_node(&self, id: PolyRef, state: u8) -> Option<usize> {
        let mut i = self.first[self.bucket(id)];
        while i != NULL_IDX {
            let n = &self.nodes[i as usize];
            if n.id == id && n.state == state {
                return Some(i as usize);
            }
            i = self.next[i as usize];
        }
        None
    }

    pub fn get_node(&mut self, id: PolyRef, state: u8) -> Option<usize> {
        let b = self.bucket(id);
        if let Some(i) = self.find_node(id, state) {
            return Some(i);
        }
        if self.count >= self.max_nodes {
            return None;
        }
        let i = self.count;
        self.count += 1;
        let n = &mut self.nodes[i];
        n.pidx = 0;
        n.cost = 0.0;
        n.total = 0.0;
        n.id = id;
        n.state = state;
        n.flags = 0;
        self.next[i] = self.first[b];
        self.first[b] = i as u16;
        Some(i)
    }

    /// Node index + 1 (0 for none), as stored in `pidx`.
    #[inline]
    pub fn idx(i: usize) -> u32 {
        i as u32 + 1
    }

    #[inline]
    pub fn at_idx(idx: u32) -> Option<usize> {
        if idx == 0 {
            None
        } else {
            Some(idx as usize - 1)
        }
    }
}

/// One open-list slot: the node index with the node's total as it was when the
/// slot was written. Only `modify` changes a queued node's total, and it rewrites
/// the slot first, so the cached key always equals `nodes[node].total`; the heap
/// operations then never touch the node array.
#[derive(Clone, Copy)]
struct Entry {
    total: f32,
    node: u32,
}

pub(crate) struct NodeQueue {
    heap: Vec<Entry>,
}

impl NodeQueue {
    pub fn new(n: usize) -> Result<NodeQueue, Error> {
        let mut heap = Vec::new();
        heap.try_reserve_exact(n + 1)
            .map_err(|_| Error::AllocationFailed)?;
        Ok(NodeQueue { heap })
    }
    pub fn clear(&mut self) {
        self.heap.clear();
    }
    pub fn empty(&self) -> bool {
        self.heap.is_empty()
    }
    /// Removes and returns the node with the smallest total (`dtNodeQueue::pop`).
    pub fn pop(&mut self, _nodes: &[Node]) -> usize {
        let result = self.heap[0].node as usize;
        let Some(last) = self.heap.pop() else {
            return result;
        };
        if !self.heap.is_empty() {
            self.trickle_down(0, last);
        }
        result
    }
    pub fn push(&mut self, node: usize, nodes: &[Node]) {
        let entry = Entry {
            total: nodes[node].total,
            node: node as u32,
        };
        self.heap.push(entry);
        let i = self.heap.len() - 1;
        self.bubble_up(i, entry);
    }
    pub fn modify(&mut self, node: usize, nodes: &[Node]) {
        if let Some(i) = self.heap.iter().position(|e| e.node as usize == node) {
            self.bubble_up(
                i,
                Entry {
                    total: nodes[node].total,
                    node: node as u32,
                },
            );
        }
    }
    fn bubble_up(&mut self, mut i: usize, entry: Entry) {
        while i > 0 {
            let parent = (i - 1) / 2;
            let p = self.heap[parent];
            if p.total > entry.total {
                self.heap[i] = p;
                i = parent;
            } else {
                break;
            }
        }
        self.heap[i] = entry;
    }
    fn trickle_down(&mut self, mut i: usize, entry: Entry) {
        let size = self.heap.len();
        let mut child = i * 2 + 1;
        while child < size {
            if child + 1 < size && self.heap[child].total > self.heap[child + 1].total {
                child += 1;
            }
            self.heap[i] = self.heap[child];
            i = child;
            child = i * 2 + 1;
        }
        self.bubble_up(i, entry);
    }
}
