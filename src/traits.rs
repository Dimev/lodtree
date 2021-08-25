//! Contains LodVec trait, which is needed for the coordinate system to be used in a tree
//! Sample implementations for this are in coords.rs

/// trait for defining a Level of Detail vector
/// such a vector contains the current position in the octree (3d coords), as well as the lod level it's at, in integer coords
pub trait LodVec: Sized + Copy + Clone + Send + Sync + Default {
    /// gets one of the child node position of this node, defined by it's index
    fn get_child(self, index: usize) -> Self;

    /// get the number of child nodes a node can have in the tree
    fn num_children() -> usize;

    /// returns the lod vector as if it's at the root of the tree
    fn root() -> Self;

    /// wether the node can subdivide, compared to another node and the required detail.
    ///
    /// Assumes self is the target position for a lod.
    ///
    /// The depth determines the max lod level allowed, detail determines the amount of chunks around the target.
    ///
    /// The implementation used in the QuadVec implementation is as follows:
    /// ```rust
    /// # struct Chunk { x: u64, y: u64, depth: u8 }
    /// # impl Chunk {
    /// fn can_subdivide(self, node: Self, detail: u64) -> bool {
    ///    // return early if the level of this chunk is too high
    ///    if node.depth >= self.depth {
    ///        return false;
    ///    }
    ///
    ///    // difference in lod level between the target and the node
    ///    let level_difference = self.depth - node.depth;
    ///
    ///    // minimum corner of the bounding box
    ///    let min = (
    ///        (node.x << (level_difference + 1))
    ///            .saturating_sub(((detail + 1) << level_difference) - (1 << level_difference)),
    ///        (node.y << (level_difference + 1))
    ///            .saturating_sub(((detail + 1) << level_difference) - (1 << level_difference)),
    ///    );
    ///
    ///    // max as well
    ///    let max = (
    ///        (node.x << (level_difference + 1))
    ///            .saturating_add(((detail + 1) << level_difference) + (1 << level_difference)),
    ///        (node.y << (level_difference + 1))
    ///            .saturating_add(((detail + 1) << level_difference) + (1 << level_difference)),
    ///    );
    ///
    ///    // local position of the target, which is one lod level higher to allow more detail
    ///    let local = (self.x << 1, self.y << 1);
    ///
    ///    // check if the target is inside of the bounding box
    ///    local.0 >= min.0 && local.0 < max.0 && local.1 >= min.1 && local.1 < max.1
    /// }
    /// # }
    /// ```
    fn can_subdivide(self, node: Self, detail: u64) -> bool;
}
