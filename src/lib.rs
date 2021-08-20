pub mod coords;
pub mod traits;

pub use coords::*;
pub use traits::*;

use std::collections::VecDeque;
use std::num::NonZeroUsize;

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
pub struct ChunkContainer<C, L>
where
    C: Chunk<Lod = L>,
    L: LodVec,
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
    C: Chunk<Lod = L>,
    L: LodVec,
{
    // all chunks
    // chunks are swap removed to keep memory compact
    pub chunks: Vec<ChunkContainer<C, L>>,

    // nodes in the Tree
    nodes: Vec<TreeNode>,

    // list of free nodes in the Tree, to allocate new nodes into
    free_list: VecDeque<usize>,
}

impl<C, L> Tree<C, L>
where
    C: Chunk<Lod = L> + Sized,
    L: LodVec,
{
    pub fn new() -> Self {
        // make a new Tree
        // also allocate some room for nodes
        Self {
            chunks: Vec::with_capacity(512),
            nodes: Vec::with_capacity(512),
            free_list: VecDeque::with_capacity(512),
        }
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
    pub fn get_pending_changes(
        &self,
        targets: &[L],
        detail: u64,
        max_levels: u64,
    ) -> TreeUpdate<C, L> {
        // if we don't have a root, make one pending for creation
        if self.nodes.is_empty() {
            return TreeUpdate::no_root();
        }

        // the update state we'll build here
        let mut pending_update = TreeUpdate::empty();

        // processing queue, this stores the current position, as well as the current node index
        let mut processing_queue = Vec::<(L, usize)>::with_capacity(self.chunks.len());

        // add the root node (always at 0, if there is no root we would have returned earlier)
        processing_queue.push((L::root(), 0));

        // then, traverse the tree, as long as something is inside the queue
        while let Some((current_position, current_node_index)) = processing_queue.pop() {
            // fetch the current node
            let current_node = self.nodes[current_node_index];

            // wether we can subdivide
            let can_subdivide = targets
                .iter()
                .any(|x| x.can_subdivide(current_position, detail, max_levels));

            // if we can subdivide, and the current node does not have children, subdivide the current node
            if can_subdivide && current_node.children == None {
                // add children to be added
                for i in 0..L::num_children() {
                    // add the position. No need to do this in reverse, as we'll use a VecDeque to go over free indices in order
                    pending_update.to_add.push(current_position.get_child(i));

                    // and add the current node index, so we can set it's children later on
                    pending_update
                        .to_add_parent_indices
                        .push(current_node_index);

                    // and queue ourselves for deactivation
                    pending_update
                        .to_deactivate_indices
                        .push(current_node_index);
                }
            } else if let Some(index) = current_node.children {
                // otherwise, if we cant subdivide and don't have a root as children, remove our children
                if !can_subdivide
                    && !(0..L::num_children())
                        .into_iter()
                        .any(|i| self.nodes[i + index.get()].children != None)
                {
                    // first, queue ourselves for activation
                    pending_update.to_activate_indices.push(current_node_index);

                    for i in 0..L::num_children() {
                        // no need to do this in reverse, that way the last node removed will be added to the free list, which is also the first thing used by the adding logic
                        pending_update.to_remove_indices.push(index.get() + i);
                        pending_update
                            .to_remove_parent_indices
                            .push(current_node_index);
                    }
                } else {
                    // queue child nodes for processing if we didn't subdivide or cleaned up our children
                    for i in 0..L::num_children() {
                        processing_queue.push((current_position.get_child(i), index.get() + i));
                    }
                }
            }
        }

        pending_update
    }

    // actually performs the update
    // returns wether this update changed anything
    pub fn execute_changes(&mut self, update: TreeUpdate<C, L>) {
        // first, activate any chunks needed
        for index in update.to_activate_indices {
            self.chunks[self.nodes[index].chunk].chunk.set_active(true);
        }

        // and deactivate
        for index in update.to_deactivate_indices {
            self.chunks[self.nodes[index].chunk].chunk.set_active(false);
        }

        // then, remove old chunks
        for (index, parent_index) in update
            .to_remove_indices
            .into_iter()
            .zip(update.to_remove_parent_indices)
        {
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
        for (parent_index, chunk) in update
            .to_add_parent_indices
            .into_iter()
            .zip(update.chunks_to_add)
        {
            // add the node
            let new_node_index = match self.free_list.pop_front() {
                Some(x) => {
                    // reuse a free node
                    self.nodes[x] = TreeNode {
                        children: None,
                        chunk: self.chunks.len(),
                    };
                    self.chunks.push(ChunkContainer::<C, L> { index: x, chunk });
                    x
                }
                None => {
                    // otherwise, use a new index
                    self.nodes.push(TreeNode {
                        children: None,
                        chunk: self.chunks.len(),
                    });
                    self.chunks.push(ChunkContainer::<C, L> {
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

    // checks wether the Tree has the specified leaf node and if so, returns the index it's at

    // remove everything
    pub fn clear(&mut self) {
        self.chunks.clear();
        self.nodes.clear();
        self.free_list.clear();
    }
}

impl<C, L> Default for Tree<C, L>
where
    C: Chunk<Lod = L>,
    L: LodVec,
{
    fn default() -> Self {
        Self::new()
    }
}

// Tree update state
// holds state of what needs to be generated for the update, and things needed to perform the update with minimal work
// also takes in a list of all chunks that will be added during the update, which is generated elsewhere
#[derive(Clone, Debug)]
pub struct TreeUpdate<C, L>
where
    C: Chunk<Lod = L>,
    L: LodVec,
{
    to_add_parent_indices: Vec<usize>, // and the chunks that are the parent of the chunk that's going to be added
    to_remove_indices: Vec<usize>, // what chunks are going to be removed (indices of that chunk)
    to_remove_parent_indices: Vec<usize>, // indices of the parent nodes when removing
    to_activate_indices: Vec<usize>, // what chunks are going to be activated
    to_deactivate_indices: Vec<usize>, // and deactivated
    pub to_add: Vec<L>,            // positions of the chunks being added
    pub chunks_to_add: Vec<C>, // chunks that are added. These will be used in the Tree for rendering
                               // chunks that need to be updated
                               // indices of the chunks that need to be updated
}

impl<C, L> TreeUpdate<C, L>
where
    C: Chunk<Lod = L>,
    L: LodVec,
{
    fn empty() -> Self {
        Self {
            to_add: Vec::new(),
            to_add_parent_indices: Vec::new(),
            to_remove_indices: Vec::new(),
            to_remove_parent_indices: Vec::new(),
            to_activate_indices: Vec::new(),
            to_deactivate_indices: Vec::new(),
            chunks_to_add: Vec::new(),
        }
    }

    fn no_root() -> Self {
        Self {
            to_add: vec![L::root()],
            to_add_parent_indices: vec![0],
            to_remove_indices: Vec::new(),
            to_remove_parent_indices: Vec::new(),
            to_activate_indices: Vec::new(),
            to_deactivate_indices: Vec::new(),
            chunks_to_add: Vec::new(),
        }
    }

    pub fn did_anything(&self) -> bool {
        !self.to_add.is_empty() || !self.to_remove_indices.is_empty()
    }
}

impl<C, L> Default for TreeUpdate<C, L>
where
    C: Chunk<Lod = L>,
    L: LodVec,
{
    fn default() -> Self {
        Self::empty()
    }
}
