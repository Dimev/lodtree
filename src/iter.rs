//! Iterators over chunks
use crate::traits::*;
use crate::tree::*;

// implements all iterators for the given functions
// this allows quickly and easily set them up for all chunks
macro_rules! impl_all_iterators {
    (
		$name:ident,
		$name_mut:ident,
		$name_pos:ident,
		$name_chunk_and_pos:ident,
		$name_chunk_and_pos_mut:ident,
		$len:ident,
		$get:ident,
		$get_mut:ident,
		$get_pos:ident,
		$(#[$doc:meta])*
		$func_name:ident,
		$(#[$doc_mut:meta])*
		$func_name_mut:ident,
		$(#[$doc_pos:meta])*
		$func_name_pos:ident,
		$(#[$doc_chunk_and_pos:meta])*
		$func_name_chunk_and_pos:ident,
		$(#[$doc_chunk_and_pos_mut:meta])*
		$func_name_chunk_and_pos_mut:ident,
	) => {
        // define the struct
        #[doc=concat!("Iterator for chunks, see ", stringify!($func_name), "() under Tree for documentation")]
		pub struct $name<'a, C: Sized, L: LodVec> {
            tree: &'a Tree<C, L>,
            index: usize,
        }

		#[doc=concat!("Iterator for mutable chunks, see ", stringify!($func_name_mut), "() under Tree for documentation")]
        pub struct $name_mut<'a, C: Sized, L: LodVec> {
            tree: &'a mut Tree<C, L>,
            index: usize,
        }

        #[doc=concat!("Iterator for chunk positions, see ", stringify!($func_name_pos), "() under Tree for documentation")]
        pub struct $name_pos<'a, C: Sized, L: LodVec> {
            tree: &'a Tree<C, L>,
            index: usize,
        }

        #[doc=concat!("Iterator for chunks and positions, see ", stringify!($func_name_chunk_and_pos), "() under Tree for documentation")]
        pub struct $name_chunk_and_pos<'a, C: Sized, L: LodVec> {
            tree: &'a Tree<C, L>,
            index: usize,
        }

        #[doc=concat!("Iterator for mutable chunks and positions, see ", stringify!($func_name_chunk_and_pos_mut), "() under Tree for documentation")]
		pub struct $name_chunk_and_pos_mut<'a, C: Sized, L: LodVec> {
            tree: &'a mut Tree<C, L>,
            index: usize,
        }

        // and implement iterator for it
        impl<'a, C: Sized, L: LodVec> Iterator for $name<'a, C, L> {
            type Item = &'a C;

			#[inline]
            fn next(&mut self) -> Option<Self::Item> {
                // if the item is too big, stop
                if self.index >= self.tree.$len() {
                    None
                } else {
                    // otherwise, get the item
                    let item = self.tree.$get(self.index);

                    // increment the index
                    self.index += 1;

                    Some(item)
                }
            }
        }

        impl<'a, C: Sized, L: LodVec> Iterator for $name_mut<'a, C, L> {
            type Item = &'a mut C;

			#[inline]
            fn next(&mut self) -> Option<Self::Item> {
                // if the item is too big, stop
                if self.index >= self.tree.$len() {
                    None
                } else {
                    // otherwise, get the item
                    let item = unsafe { self.tree.$get_mut(self.index).as_mut()? };

                    // increment the index
                    self.index += 1;

                    Some(item)
                }
            }
        }

        impl<'a, C: Sized, L: LodVec> Iterator for $name_pos<'a, C, L> {
            type Item = L;

			#[inline]
            fn next(&mut self) -> Option<Self::Item> {
                // if the item is too big, stop
                if self.index >= self.tree.$len() {
                    None
                } else {
                    // otherwise, get the item
                    let item = self.tree.$get_pos(self.index);

                    // increment the index
                    self.index += 1;

                    Some(item)
                }
            }
        }

        impl<'a, C: Sized, L: LodVec> Iterator for $name_chunk_and_pos<'a, C, L> {
            type Item = (&'a C, L);

			#[inline]
            fn next(&mut self) -> Option<Self::Item> {
                // if the item is too big, stop
                if self.index >= self.tree.$len() {
                    None
                } else {
                    // otherwise, get the item
                    let item = (self.tree.$get(self.index), self.tree.$get_pos(self.index));

                    // increment the index
                    self.index += 1;

                    Some(item)
                }
            }
        }

        impl<'a, C: Sized, L: LodVec> Iterator for $name_chunk_and_pos_mut<'a, C, L> {
            type Item = (&'a mut C, L);

			#[inline]
            fn next(&mut self) -> Option<Self::Item> {
                // if the item is too big, stop
                if self.index >= self.tree.$len() {
                    None
                } else {
                    // otherwise, get the item
                    let item = (
                        unsafe { self.tree.$get_mut(self.index).as_mut()? },
                        self.tree.$get_pos(self.index),
                    );

                    // increment the index
                    self.index += 1;

                    Some(item)
                }
            }
        }

        // exact size as well
        impl<'a, C: Sized, L: LodVec> ExactSizeIterator for $name<'a, C, L> {
			#[inline]
            fn len(&self) -> usize {
                self.tree.$len()
            }
        }

        impl<'a, C: Sized, L: LodVec> ExactSizeIterator for $name_mut<'a, C, L> {
			#[inline]
            fn len(&self) -> usize {
                self.tree.$len()
            }
        }

        impl<'a, C: Sized, L: LodVec> ExactSizeIterator for $name_pos<'a, C, L> {
			#[inline]
            fn len(&self) -> usize {
                self.tree.$len()
            }
        }

        impl<'a, C: Sized, L: LodVec> ExactSizeIterator for $name_chunk_and_pos<'a, C, L> {
			#[inline]
            fn len(&self) -> usize {
                self.tree.$len()
            }
        }

        impl<'a, C: Sized, L: LodVec> ExactSizeIterator for $name_chunk_and_pos_mut<'a, C, L> {
			#[inline]
            fn len(&self) -> usize {
                self.tree.$len()
            }
        }

        // fused, because it will always return none when done
        impl<'a, C: Sized, L: LodVec> std::iter::FusedIterator for $name<'a, C, L> {}
        impl<'a, C: Sized, L: LodVec> std::iter::FusedIterator for $name_mut<'a, C, L> {}
        impl<'a, C: Sized, L: LodVec> std::iter::FusedIterator for $name_pos<'a, C, L> {}
        impl<'a, C: Sized, L: LodVec> std::iter::FusedIterator for $name_chunk_and_pos<'a, C, L> {}
        impl<'a, C: Sized, L: LodVec> std::iter::FusedIterator
            for $name_chunk_and_pos_mut<'a, C, L>
        {
        }

        // and implement all of them for the tree
        impl<'a, C, L> Tree<C, L>
        where
            C: Sized,
            L: LodVec,
            Self: 'a,
        {
			#[inline]
			$(#[$doc])*
			pub fn $func_name(&mut self) -> $name<C, L> {
				$name {
					tree: self,
					index: 0,
				}
			}

			#[inline]
			$(#[$doc_mut])*
			pub fn $func_name_mut(&mut self) -> $name_mut<C, L> {
				$name_mut {
					tree: self,
					index: 0,
				}
			}

			#[inline]
			$(#[$doc_pos])*
			pub fn $func_name_pos(&mut self) -> $name_pos<C, L> {
				$name_pos {
					tree: self,
					index: 0,
				}
			}

			#[inline]
			$(#[$doc_chunk_and_pos])*
			pub fn $func_name_chunk_and_pos(&mut self) -> $name_chunk_and_pos<C, L> {
				$name_chunk_and_pos {
					tree: self,
					index: 0,
				}
			}

			#[inline]
			$(#[$doc_chunk_and_pos_mut])*
			pub fn $func_name_chunk_and_pos_mut(&mut self) -> $name_chunk_and_pos_mut<C, L> {
				$name_chunk_and_pos_mut {
					tree: self,
					index: 0,
				}
			}
        }
    };
}

// chunks
impl_all_iterators!(
    ChunkIter,
    ChunkIterMut,
    PositionIter,
    ChunkAndPositionIter,
    ChunkAndPositionIterMut,
    get_num_chunks,
    get_chunk,
    get_chunk_pointer_mut,
    get_chunk_position,
    /// returns an iterator over all chunks
    iter_chunks,
    /// returns an iterator over all chunks, mutable
    iter_chunks_mut,
    /// returns an iterator over all positions of all chunks
    iter_chunk_positions,
    /// returns an iterator over all chunks and their positions
    iter_chunks_and_positions,
    /// returns an iterator over all chunks as mutable and their positions
    iter_chunks_and_positions_mut,
);

// to activate
impl_all_iterators!(
    ChunkToActivateIter,
    ChunkToActivateIterMut,
    PositionToActivateIter,
    ChunkAndPositionToActivateIter,
    ChunkAndPositionIterToActivateMut,
    get_num_chunks_to_activate,
    get_chunk_to_activate,
    get_chunk_to_activate_pointer_mut,
    get_position_of_chunk_to_activate,
    /// returns an iterator over all chunks to activate
    iter_chunks_to_activate,
    /// returns an iterator over all chunks to activate, mutable
    iter_chunks_to_activate_mut,
    /// returns an iterator over all positions of all chunks to activate
    iter_chunks_to_activate_positions,
    /// returns an iterator over all chunks to activate and their positions
    iter_chunks_to_activate_and_positions,
    /// returns an iterator over all chunks to activate as mutable and their positions
    iter_chunks_to_activate_and_positions_mut,
);

// to deactivate
impl_all_iterators!(
    ChunkToDeactivateIter,
    ChunkToDeactivateIterMut,
    PositionToDeactivateIter,
    ChunkAndPositionToDeactivateIter,
    ChunkAndPositionIterToDeactivateMut,
    get_num_chunks_to_deactivate,
    get_chunk_to_deactivate,
    get_chunk_to_deactivate_pointer_mut,
    get_position_of_chunk_to_deactivate,
    /// returns an iterator over all chunks to deactivate
    iter_chunks_to_deactivate,
    /// returns an iterator over all chunks to deactivate, mutable
    iter_chunks_to_deactivate_mut,
    /// returns an iterator over all positions of all chunks to deactivate
    iter_chunks_to_deactivate_positions,
    /// returns an iterator over all chunks to deactivate and their positions
    iter_chunks_to_deactivate_and_positions,
    /// returns an iterator over all chunks to deactivate as mutable and their positions
    iter_chunks_to_deactivate_and_positions_mut,
);

// to add
impl_all_iterators!(
    ChunkToAddIter,
    ChunkToAddIterMut,
    PositionToAddIter,
    ChunkAndPositionToAddIter,
    ChunkAndPositionIterToAddMut,
    get_num_chunks_to_add,
    get_chunk_to_add,
    get_chunk_to_add_pointer_mut,
    get_position_of_chunk_to_add,
    /// returns an iterator over all chunks to add
    iter_chunks_to_add,
    /// returns an iterator over all chunks to add, mutable
    iter_chunks_to_add_mut,
    /// returns an iterator over all positions of all chunks to add
    iter_chunks_to_add_positions,
    /// returns an iterator over all chunks to add and their positions
    iter_chunks_to_add_and_positions,
    /// returns an iterator over all chunks to add as mutable and their positions
    iter_chunks_to_add_and_positions_mut,
);

// to remove
impl_all_iterators!(
    ChunkToRemoveIter,
    ChunkTorRmoveIterMut,
    PositionToRemoveIter,
    ChunkAndPositionToRemoveIter,
    ChunkAndPositionIterToRemoveMut,
    get_num_chunks_to_remove,
    get_chunk_to_remove,
    get_chunk_to_remove_pointer_mut,
    get_position_of_chunk_to_remove,
    /// returns an iterator over all chunks to remove
    iter_chunks_to_remove,
    /// returns an iterator over all chunks to remove, mutable
    iter_chunks_to_remove_mut,
    /// returns an iterator over all positions of all chunks to remove
    iter_chunks_to_remove_positions,
    /// returns an iterator over all chunks to remove and their positions
    iter_chunks_to_remove_and_positions,
    /// returns an iterator over all chunks to remove as mutable and their positions
    iter_chunks_to_remove_and_positions_mut,
);

// to delete
impl_all_iterators!(
    ChunkToDeleteIter,
    ChunkToDeleteIterMut,
    PositionToDeleteIter,
    ChunkAndPositionToDeleteIter,
    ChunkAndPositionIterToDeleteMut,
    get_num_chunks_to_delete,
    get_chunk_to_delete,
    get_chunk_to_delete_pointer_mut,
    get_position_of_chunk_to_delete,
    /// returns an iterator over all chunks to delete
    iter_chunks_to_delete,
    /// returns an iterator over all chunks to delete, mutable
    iter_chunks_to_delete_mut,
    /// returns an iterator over all positions of all chunks to delete
    iter_chunks_to_delete_positions,
    /// returns an iterator over all chunks to delete and their positions
    iter_chunks_to_delete_and_positions,
    /// returns an iterator over all chunks to delete as mutable and their positions
    iter_chunks_to_delete_and_positions_mut,
);

// iterator for all chunks that are inside given bounds
pub struct ChunksInBoundIter<L: LodVec> {
    // internal stack for which chunks are next
    stack: Vec<L>,

    // and maximum depth to go to
    max_depth: u8,

    // and the min of the bound
    bound_min: L,

    // and max of the bound
    bound_max: L,
}

impl<L: LodVec> Iterator for ChunksInBoundIter<L> {
    type Item = L;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let current = self.stack.pop()?;

        // go over all child nodes
        for i in 0..L::num_children() {
            let position = current.get_child(i);

            // if they are in bounds, and the correct depth, add them to the stack
            if position.is_inside_bounds(self.bound_min, self.bound_max, self.max_depth) {
                self.stack.push(position);
            }
        }
        // and return this item from the stack
        Some(current)
    }
}

pub struct ChunksInBoundAndMaybeTreeIter<'a, C: Sized, L: LodVec> {
    // the tree
    tree: &'a Tree<C, L>,

    // internal stack for which chunks are next
    stack: Vec<(L, Option<TreeNode>)>,

    // and maximum depth to go to
    max_depth: u8,

    // and the min of the bound
    bound_min: L,

    // and max of the bound
    bound_max: L,
}

impl<'a, C: Sized, L: LodVec> Iterator for ChunksInBoundAndMaybeTreeIter<'a, C, L> {
    type Item = (L, Option<&'a C>);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let (current_position, current_node) = self.stack.pop()?;

        // go over all child nodes
        for i in 0..L::num_children() {
            let position = current_position.get_child(i);

            // if they are in bounds, and the correct depth, add them to the stack
            if position.is_inside_bounds(self.bound_min, self.bound_max, self.max_depth) {
                // also, check if the node has children
                if let Some(node) = current_node {
                    // and if it has children
                    if let Some(children) = node.children {
                        // children, so node
                        self.stack
                            .push((position, Some(self.tree.nodes[children.get() + i])));
                    } else {
                        // no node, so no chunk
                        self.stack.push((position, None));
                    }
                } else {
                    // no node, so no chunk
                    self.stack.push((position, None));
                }
            }
        }
        // and return this item from the stack
        if let Some(node) = current_node {
            // there is a node, so get the chunk it has
            let chunk = &self.tree.chunks[node.chunk].chunk;

            // and return it
            Some((current_position, Some(chunk)))
        } else {
            // no chunk, so return that as None
            Some((current_position, None))
        }
    }
}

pub struct ChunksInBoundAndTreeIter<'a, C: Sized, L: LodVec> {
    // the tree
    tree: &'a Tree<C, L>,

    // internal stack for which chunks are next
    stack: Vec<(L, TreeNode)>,

    // and maximum depth to go to
    max_depth: u8,

    // and the min of the bound
    bound_min: L,

    // and max of the bound
    bound_max: L,
}

impl<'a, C: Sized, L: LodVec> Iterator for ChunksInBoundAndTreeIter<'a, C, L> {
    type Item = (L, &'a C);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let (current_position, current_node) = self.stack.pop()?;

        // go over all child nodes
        for i in 0..L::num_children() {
            let position = current_position.get_child(i);

            // if the node has children
            if let Some(children) = current_node.children {
                // if they are in bounds, and the correct depth, add them to the stack
                if position.is_inside_bounds(self.bound_min, self.bound_max, self.max_depth) {
                    // and push to the stack
                    self.stack
                        .push((position, self.tree.nodes[children.get() + i]));
                }
            }
        }

        // and return the position and node
        Some((
            current_position,
            &self.tree.chunks[current_node.chunk].chunk,
        ))
    }
}

pub struct ChunksInBoundAndMaybeTreeIterMut<'a, C: Sized, L: LodVec> {
    // the tree
    tree: &'a mut Tree<C, L>,

    // internal stack for which chunks are next
    stack: Vec<(L, Option<TreeNode>)>,

    // and maximum depth to go to
    max_depth: u8,

    // and the min of the bound
    bound_min: L,

    // and max of the bound
    bound_max: L,
}

impl<'a, C: Sized, L: LodVec> Iterator for ChunksInBoundAndMaybeTreeIterMut<'a, C, L> {
    type Item = (L, Option<&'a mut C>);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let (current_position, current_node) = self.stack.pop()?;

        // go over all child nodes
        for i in 0..L::num_children() {
            let position = current_position.get_child(i);

            // if they are in bounds, and the correct depth, add them to the stack
            if position.is_inside_bounds(self.bound_min, self.bound_max, self.max_depth) {
                // also, check if the node has children
                if let Some(node) = current_node {
                    // and if it has children
                    if let Some(children) = node.children {
                        // children, so node
                        self.stack
                            .push((position, Some(self.tree.nodes[children.get() + i])));
                    } else {
                        // no node, so no chunk
                        self.stack.push((position, None));
                    }
                } else {
                    // no node, so no chunk
                    self.stack.push((position, None));
                }
            }
        }
        // and return this item from the stack
        if let Some(node) = current_node {
            // there is a node, so get the chunk it has
            let chunk = &mut self.tree.chunks[node.chunk].chunk as *mut C;

            // and return it
            // Safety: The iterator lives at least as long as the tree, and no changes can be made to the tree while it's borrowed by the iterator
            Some((current_position, Some(unsafe { chunk.as_mut()? })))
        } else {
            // no chunk, so return that as None
            Some((current_position, None))
        }
    }
}

pub struct ChunksInBoundAndTreeIterMut<'a, C: Sized, L: LodVec> {
    // the tree
    tree: &'a mut Tree<C, L>,

    // internal stack for which chunks are next
    stack: Vec<(L, TreeNode)>,

    // and maximum depth to go to
    max_depth: u8,

    // and the min of the bound
    bound_min: L,

    // and max of the bound
    bound_max: L,
}

impl<'a, C: Sized, L: LodVec> Iterator for ChunksInBoundAndTreeIterMut<'a, C, L> {
    type Item = (L, &'a mut C);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let (current_position, current_node) = self.stack.pop()?;

        // go over all child nodes
        for i in 0..L::num_children() {
            let position = current_position.get_child(i);

            // if the node has children
            if let Some(children) = current_node.children {
                // if they are in bounds, and the correct depth, add them to the stack
                if position.is_inside_bounds(self.bound_min, self.bound_max, self.max_depth) {
                    // and push to the stack
                    self.stack
                        .push((position, self.tree.nodes[children.get() + i]));
                }
            }
        }

        // and return the position and node
        // Safety: The iterator lives at least as long as the tree, and no changes can be made to the tree while it's borrowed by the iterator
        Some((current_position, unsafe {
            (&mut self.tree.chunks[current_node.chunk].chunk as *mut C).as_mut()?
        }))
    }
}

// TODO: iterator that also goes over chunks in the tree
// as in: chunks in tree and bounds, immutable and mutable
// all chunks in the bounds, and ones in the tree, if any

impl<'a, C, L> Tree<C, L>
where
    C: Sized,
    L: LodVec,
    Self: 'a,
{
    /// iterate over all chunks that would be affected by an edit inside a certain bound
    #[inline]
    pub fn iter_all_chunks_in_bounds(
        bound_min: L,
        bound_max: L,
        max_depth: u8,
    ) -> ChunksInBoundIter<L> {
        debug_assert!(
            bound_min < bound_max,
            "Bounds must select a non-empty area/volume"
        );
        ChunksInBoundIter {
            stack: vec![L::root()],
            max_depth,
            bound_min,
            bound_max,
        }
    }

    /// iterate over all chunks that would be affected by an edit, including the chunk if it's in the tree
    #[inline]
    pub fn iter_all_chunks_in_bounds_and_maybe_tree(
        &'a self,
        bound_min: L,
        bound_max: L,
        max_depth: u8,
    ) -> ChunksInBoundAndMaybeTreeIter<C, L> {
        ChunksInBoundAndMaybeTreeIter {
            stack: vec![(L::root(), self.nodes.first().copied())],
            tree: self,
            max_depth,
            bound_min,
            bound_max,
        }
    }

    /// iterate over all chunks that would be affected by an edit, and the chunk that's in the tree.
    /// Skips any chunks that are not in the tree
    #[inline]
    pub fn iter_all_chunks_in_bounds_and_tree(
        &'a self,
        bound_min: L,
        bound_max: L,
        max_depth: u8,
    ) -> ChunksInBoundAndTreeIter<C, L> {
        // get the stack, empty if we can't get the first node
        let stack = if let Some(node) = self.nodes.first() {
            vec![(L::root(), *node)]
        } else {
            vec![]
        };

        ChunksInBoundAndTreeIter {
            stack,
            tree: self,
            max_depth,
            bound_min,
            bound_max,
        }
    }

    /// iterate over all chunks that would be affected by an edit, including the mutable chunk if it's in the tree
    #[inline]
    pub fn iter_all_chunks_in_bounds_and_maybe_tree_mut(
        &'a mut self,
        bound_min: L,
        bound_max: L,
        max_depth: u8,
    ) -> ChunksInBoundAndMaybeTreeIterMut<C, L> {
        ChunksInBoundAndMaybeTreeIterMut {
            stack: vec![(L::root(), self.nodes.first().copied())],
            tree: self,
            max_depth,
            bound_min,
            bound_max,
        }
    }

    /// iterate over all chunks that would be affected by an edit, and the chunk that's in the tree as mutable.
    /// Skips any chunks that are not in the tree
    #[inline]
    pub fn iter_all_chunks_in_bounds_and_tree_mut(
        &'a mut self,
        bound_min: L,
        bound_max: L,
        max_depth: u8,
    ) -> ChunksInBoundAndTreeIterMut<C, L> {
        // get the stack, empty if we can't get the first node
        let stack = if let Some(node) = self.nodes.first() {
            vec![(L::root(), *node)]
        } else {
            vec![]
        };
        ChunksInBoundAndTreeIterMut {
            stack,
            tree: self,
            max_depth,
            bound_min,
            bound_max,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::coords::*;
    use std::ops::Range;
    use rand::distributions::uniform::SampleUniform;
    use rand::Rng;
    use std::cmp::Ordering;

    pub trait SafeRngRange {
        fn safe_uniform<T>(&mut self, range: Range<T>) -> T
        where
            T: SampleUniform + PartialOrd;
    }

    impl SafeRngRange for rand::rngs::ThreadRng {
        //#[no_panic]
        fn safe_uniform<T>(&mut self, range: Range<T>) -> T
        where
            T: SampleUniform + PartialOrd,
        {
            if range.is_empty() {
                range.start
            } else {
                self.gen_range(range)
            }
        }
    }
    // TODO: also test the other iters
    fn get_chunk_count_at_max_depth(a: QuadVec, b: QuadVec) -> u64 {
        assert_eq!(a.depth, b.depth);
        ((b.x - a.x) + 1) * ((b.y - a.y) + 1)
    }

    #[test]
    fn test_bounds_quadtree() {
        const D: u8 = 4;

        let mut rng = rand::thread_rng();

        for i in 1..100 {
            let cmax = 1 << D;
            let min = QuadVec::new(rng.safe_uniform(0..cmax), rng.safe_uniform(0..cmax), D);
            let max = QuadVec::new(rng.safe_uniform(0..cmax), rng.safe_uniform(0..cmax), D);
            //println!("Generated min  {:?}", min);
            //println!("Generated max {:?}", max);
            let cmp = min.partial_cmp(&max);
            if cmp.is_none() {
                // println!("Can not compare {min:?} and {max:?}");
                continue;
            }
            let (min, max) = match cmp.unwrap() {
                Ordering::Greater => (max, min),
                Ordering::Less => (min, max),
                Ordering::Equal => {
                    continue;
                }
            };
            struct C;
            let mut count = 0;
            for pos in Tree::<C, QuadVec>::iter_all_chunks_in_bounds(min, max, D) {
                // println!("{:?}", pos);

                if pos.depth == 4 {
                    count += 1;
                }
            }
            assert_eq!(count, get_chunk_count_at_max_depth(min, max));
        }
    }

    fn get_chunk_count_at_max_depth_oct(a: OctVec, b: OctVec) -> u64 {
        assert_eq!(a.depth, b.depth);
        ((b.x - a.x) + 1) * ((b.y - a.y) + 1) * ((b.z - a.z) + 1)
    }
    ///The same unit test as test_bounds juts for OctVec:
    /// //todo check it once again
    #[test]
    fn test_bounds_octree() {
        const D: u8 = 4;

        let mut rng = rand::thread_rng();

        for i in 1..100 {
            let cmax = 1 << D;
            let min = OctVec::new(
                rng.safe_uniform(0..cmax),
                rng.safe_uniform(0..cmax),
                rng.safe_uniform(0..cmax),
                D,
            );
            let max = OctVec::new(
                rng.safe_uniform(0..cmax),
                rng.safe_uniform(0..cmax),
                rng.safe_uniform(0..cmax),
                D,
            );

            println!("Generated min  {:?}", min);
            println!("Generated max {:?}", max);
            let cmp = min.partial_cmp(&max);
            if cmp.is_none() {
                println!("Can not compare {min:?} and {max:?}");
                continue;
            }
            let (min, max) = match cmp.unwrap() {
                Ordering::Greater => (max, min),
                Ordering::Less => (min, max),
                Ordering::Equal => {
                    continue;
                }
            };
            struct C;
            let mut count = 0;
            for pos in Tree::<C, OctVec>::iter_all_chunks_in_bounds(min, max, D) {
                // println!("{:?}", pos);

                if pos.depth == 4 {
                    count += 1;
                }
            }
            assert_eq!(count, get_chunk_count_at_max_depth_oct(min, max));
        }
    }

    #[test]
    fn test_iter_all_chunks_in_bounds_and_tree_mut() {
        struct Chunk {
            visible: bool,
            cache_state: i32,
            // 0 is new, 1 is merged, 2 is cached, 3 is both
            selected: bool,
            in_bounds: bool,
        }

        fn chunk_creator(position: QuadVec) -> Chunk {
            let r = 6;

            let visible = match position.depth {
                4 => (position.x as i32 - r).pow(2) + (position.y as i32 - r).pow(2) < r,
                _ => false,
            };
            // dbg!(position);
            //  dbg!(visible);
            Chunk {
                visible: true,
                cache_state: visible as i32,
                selected: false,
                in_bounds: false,
            }
        }
        let mut tree = Tree::new(65);
        let qv = QuadVec::new(6, 6, 4);
        if tree.prepare_update(&[qv], 6, chunk_creator) {
            // position should already have been set, so we can just change the visibility
            for chunk in tree.iter_chunks_to_activate_mut() {
                chunk.visible = true;
                //     chunk.cache_state |= 1;
            }

            for chunk in tree.iter_chunks_to_deactivate_mut() {
                chunk.visible = false;
            }

            // and make chunks that are cached visible
            for chunk in tree.iter_chunks_to_remove_mut() {
                chunk.cache_state = 2;
            }

            // do the update
            tree.do_update();

            // and clean
            tree.complete_update();
        }

        let min = QuadVec::new(0, 0, 4);
        let max = QuadVec::new(8, 8, 4);
        let mut count = 0;

        for i in tree.iter_all_chunks_in_bounds_and_tree_mut(min, max, 4) {
            if i.0
                .contains_child_node(QuadVec::new(i.0.x << 1, i.0.y << 1, 4))
            {
                i.1.visible = true;
                println!("cords:{:?}", i.0.get_float_coords())
            }
        }
    }
}
