pub mod coords;
pub mod traits;

pub use coords::*;
pub use traits::*;

use std::collections::VecDeque;
use std::num::NonZeroUsize;

// TODO: docs, examples
// Also, iterator that borrows the Tree and loops over all chunks

// struct for keeping track of chunks
// keeps track of the parent and child indices
#[derive(Copy, Clone, Debug, Default)]
struct TreeNode {
    // children, these can't be the root (index 0), so we can use Some and Nonzero for slightly more compact memory
    // children are also contiguous, so we can assume that this to this + num children - 1 are all the children of this node
    children: Option<NonZeroUsize>,

    // where the chunk for this node is stored
    chunk: usize,
}

// utility struct for holding actual chunks and the node that owns them
#[derive(Debug)]
pub struct ChunkContainer<C>
where
    C: Sized,
{
    pub chunk: C,
    index: usize,
}

// Tree holding all chunks
// partially based on: https://stackoverflow.com/questions/41946007/efficient-and-well-explained-implementation-of-a-quadtree-for-2d-collision-det
// assumption here is that because of the fact that we need to keep inactive chunks in memory for later use, we can keep them together with the actual nodes
#[derive(Debug)]
pub struct Tree<C, L>
where
    C: Sized,
    L: LodVec,
{
    /// All chunks in the tree
    chunks: Vec<ChunkContainer<C>>,

    /// nodes in the Tree
    nodes: Vec<TreeNode>,

    /// list of free nodes in the Tree, to allocate new nodes into
    free_list: VecDeque<usize>,

    /// parent chunk indices of the chunks to be added
    /// tuple of the parent index, the position, and the chunk
    chunks_to_add: Vec<(usize, L, C)>,

    /// chunk indices to be removed, tuple of index, parent index
    chunks_to_remove: Vec<(usize, usize)>,

    /// indices of the chunks that need to be activated
    chunks_to_activate: Vec<usize>,

    /// indices of the chunks that need to be deactivated
    chunks_to_deactivate: Vec<usize>,

    /// internal queue for processing, that way we won't need to reallocate it
    processing_queue: Vec<(L, usize)>,
    // TODO: add a special array for chunks that are in bounds, to help doing editing
}

impl<'a, C, L> Tree<C, L>
where
    C: Sized,
    L: LodVec,
{
    pub fn new() -> Self {
        // make a new Tree
        // also allocate some room for nodes
        Self {
            chunks_to_add: Vec::with_capacity(512),
            chunks_to_remove: Vec::with_capacity(512),
            chunks_to_activate: Vec::with_capacity(512),
            chunks_to_deactivate: Vec::with_capacity(512),
            chunks: Vec::with_capacity(512),
            nodes: Vec::with_capacity(512),
            free_list: VecDeque::with_capacity(512),
            processing_queue: Vec::with_capacity(512),
        }
    }

    // get the number of chunks
    pub fn get_num_chunks(&self) -> usize {
        self.chunks.len()
    }

    // traverse the tree and figure out what needs to happen to which chunks
    // creates a "state" that holds everything needed to perform the update, and what chunks will be affected and what data needs to be generated for those chunks

    // how it works:
    // each node contains a pointer to it's chunk data and first child
    // start from the root node, which is at 0
    // check if we can't subdivide, and if all children are leafs
    // if so, queue children for removal, and self for activation (child indices, chunk pointer)
    // if we can subdivide, and have no children, queue children for addition, and self for removal (child positions, chunk pointer)
    // if none of the above and have children, queue children for processing
    // processing queue is only the node positon and node index

    // when removing nodes, do so in groups of num children, and use the free list
    // clear the free list once we only have one chunk (the root) active
    // swap remove chunks, and update the node that references them (nodes won't move due to free list)
    /// prepares the tree for an update
    /// returns wether any update is needed
    pub fn prepare_update(
        &mut self,
        targets: &[L],
        detail: u64,
        chunk_creator: fn(L) -> C,
    ) -> bool {
        // first, clear the previous arrays
        self.chunks_to_add.clear();
        self.chunks_to_remove.clear();
        self.chunks_to_activate.clear();
        self.chunks_to_deactivate.clear();

        // if we don't have a root, make one pending for creation
        if self.nodes.is_empty() {
            // we need to add the root as pending
            self.chunks_to_add
                .push((0, L::root(), chunk_creator(L::root())));

            // and an update is needed
            return true;
        }

        // clear the processing queue from any previous updates
        self.processing_queue.clear();

        // add the root node (always at 0, if there is no root we would have returned earlier) to the processing queue
        self.processing_queue.push((L::root(), 0));

        // then, traverse the tree, as long as something is inside the queue
        while let Some((current_position, current_node_index)) = self.processing_queue.pop() {
            // fetch the current node
            let current_node = self.nodes[current_node_index];

            // wether we can subdivide
            let can_subdivide = targets
                .iter()
                .any(|x| x.can_subdivide(current_position, detail));

            // if we can subdivide, and the current node does not have children, subdivide the current node
            if can_subdivide && current_node.children == None {
                // add children to be added
                for i in 0..L::num_children() {
                    // add the new chunk to be added
                    self.chunks_to_add.push((
                        current_node_index,
                        current_position.get_child(i),
                        chunk_creator(current_position.get_child(i)),
                    ));

                    // and add ourselves for deactivation
                    self.chunks_to_deactivate.push(current_node_index);
                }
            } else if let Some(index) = current_node.children {
                // otherwise, if we cant subdivide and don't have a root as children, remove our children
                if !can_subdivide
                    && !(0..L::num_children())
                        .into_iter()
                        .any(|i| self.nodes[i + index.get()].children != None)
                {
                    // first, queue ourselves for activation
                    self.chunks_to_activate.push(current_node_index);

                    for i in 0..L::num_children() {
                        // no need to do this in reverse, that way the last node removed will be added to the free list, which is also the first thing used by the adding logic
                        self.chunks_to_remove
                            .push((index.get() + i, current_node_index));
                        //pending_update
                        //    .to_remove_parent_indices
                        //    .push(current_node_index);
                    }
                } else {
                    // queue child nodes for processing if we didn't subdivide or cleaned up our children
                    for i in 0..L::num_children() {
                        self.processing_queue
                            .push((current_position.get_child(i), index.get() + i));
                    }
                }
            }
        }

        // and return wether an update needs to be done
        !self.chunks_to_add.is_empty() || !self.chunks_to_remove.is_empty()
    }

    // actually performs the update
    // returns wether this update changed anything
    pub fn do_update(&mut self) {
        // no need to do anything with chunks that needed to be (de)activated, as we assume that has been handled beforehand

        // then, remove old chunks
        // we'll drain the vector, as we don't need it anymore afterward
        for (index, parent_index) in self.chunks_to_remove.drain(..) {
            // remove the node from the tree
            self.nodes[parent_index].children = None;
            self.free_list.push_back(index);

            // and swap remove the chunk
            let chunk_index = self.nodes[index].chunk;

            self.chunks.swap_remove(chunk_index);

            // and properly set the chunk pointer of the node of the chunk we just moved, if any
            if chunk_index < self.chunks.len() {
                self.nodes[self.chunks[chunk_index].index].chunk = chunk_index;
            }
        }

        // add new chunks
        // we'll drain the vector here as well, as we won't need it anymore afterward
        for (parent_index, _, chunk) in self.chunks_to_add.drain(..) {
            // add the node
            let new_node_index = match self.free_list.pop_front() {
                Some(x) => {
                    // reuse a free node
                    self.nodes[x] = TreeNode {
                        children: None,
                        chunk: self.chunks.len(),
                    };
                    self.chunks.push(ChunkContainer { index: x, chunk });
                    x
                }
                None => {
                    // otherwise, use a new index
                    self.nodes.push(TreeNode {
                        children: None,
                        chunk: self.chunks.len(),
                    });
                    self.chunks.push(ChunkContainer {
                        index: self.nodes.len() - 1,
                        chunk,
                    });
                    self.nodes.len() - 1
                }
            };

            // correctly set the children of the parent node
            if new_node_index >= L::num_children() {
                // because we loop in order, and our nodes are contiguous, the first node of the children got added on index i - (num children - 1)
                // so we need to adjust for that
                self.nodes[parent_index].children =
                    NonZeroUsize::new(new_node_index - (L::num_children() - 1));
            }
        }

        // if there's only chunk left, we know it's the root, so we can get rid of all free nodes and unused nodes
        if self.chunks.len() == 1 {
            self.free_list.clear();
            self.nodes.resize(
                1,
                TreeNode {
                    children: None,
                    chunk: 0,
                },
            );
        }
    }

    // remove everything
    pub fn clear(&mut self) {
        self.chunks.clear();
        self.nodes.clear();
        self.free_list.clear();
        self.chunks_to_add.clear();
        self.chunks_to_remove.clear();
        self.chunks_to_activate.clear();
        self.chunks_to_deactivate.clear();
        self.processing_queue.clear();
    }
}

impl<C, L> Default for Tree<C, L>
where
    C: Sized,
    L: LodVec,
{
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    struct TestChunk;

    #[test]
    fn new_tree() {
        // make a tree
        let mut tree = Tree::<TestChunk, QuadVec>::new();

        // as long as we need to update, do so
        while tree.prepare_update(&[QuadVec::new(128, 128, 32)], 8, |_| TestChunk {}) {
            // and actually update
            tree.do_update();
        }

        // and make the tree have no items
        while tree.prepare_update(&[], 8, |_| TestChunk {}) {
            // and actually update
            tree.do_update();
        }

        // and do the same for an octree
        let mut tree = Tree::<TestChunk, OctVec>::new();

        // as long as we need to update, do so
        while tree.prepare_update(&[OctVec::new(128, 128, 128, 32)], 8, |_| TestChunk {}) {
            // and actually update
            tree.do_update();
        }

        // and make the tree have no items
        while tree.prepare_update(&[], 8, |_| TestChunk {}) {
            // and actually update
            tree.do_update();
        }
    }
}
