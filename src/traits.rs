/// trait for defining a Level of Detail vector
/// such a vector contains the current position in the octree (3d coords), as well as the lod level it's at, in integer coords
pub trait LodVec: Sized + Copy + Clone + Send + Sync + Default {
    /// gets one of the child position of this node, defined by it's index
    fn get_child(self, index: usize) -> Self;

    /// get the number of child nodes this lod vector can have
    fn num_children() -> usize;

    /// returns the lod vector as if it's at the root
    fn root() -> Self;

    /// wether the node can subdivide, compared to another node and the required detail, and maximum number of lod levels
    /// assumes self is the target position for a lod
    fn can_subdivide(self, node: Self, detail: u64, max_lod_levels: u64) -> bool;
}

/// struct representing a chunk
pub trait Chunk: Sized {
    type Lod: LodVec;

    /// sets the current chunk to be active, which is visible, or inactive, which is invisible
    fn set_active(&mut self, active: bool);
}
