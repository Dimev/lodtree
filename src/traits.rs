/// trait for defining a Level of Detail vector
/// such a vector contains the current position in the octree (3d coords), as well as the lod level it's at, in integer coords
pub trait LodVec: Sized + Copy + Clone + Send + Sync + Default {
    /// gets one of the child position of this node, defined by it's index
    fn get_child(self, index: usize) -> Self;

    /// get the number of child nodes this lod vector can have
    fn num_children() -> usize;

    /// returns the lod vector as if it's at the root
    fn root() -> Self;

    /// wether the node can subdivide, compared to another node and the required detail
    /// assumes self is the target position for a lod
    /// the depth determines the max lod level allowed, detail determines the amount of chunks around the target
    fn can_subdivide(self, node: Self, detail: u64) -> bool;
}
