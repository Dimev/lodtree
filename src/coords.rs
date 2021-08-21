//! Contains coordinate structs, QuadVec for quadtrees, and OctVec for octrees, as well as their LodVec implementation

use crate::traits::LodVec;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Default, Debug)]
pub struct QuadVec {
    /// x position in the quadtree
    pub x: u64,

    /// y position in the quadtree
    pub y: u64,

    /// lod depth in the quadtree
    /// this is limited, hence we use u8
    pub depth: u8,
}

impl QuadVec {
    #[inline]
    pub fn new(x: u64, y: u64, depth: u8) -> Self {
        Self { x, y, depth }
    }
}

impl LodVec for QuadVec {
    #[inline]
    fn num_children() -> usize {
        4
    }

    #[inline]
    fn root() -> Self {
        Self {
            x: 0,
            y: 0,
            depth: 0,
        }
    }

    #[inline]
    fn get_child(self, index: usize) -> Self {
        match index {
            0 => QuadVec::new(self.x << 1, self.y << 1, self.depth + 1),
            1 => QuadVec::new(self.x << 1, (self.y << 1) + 1, self.depth + 1),
            2 => QuadVec::new((self.x << 1) + 1, self.y << 1, self.depth + 1),
            _ => QuadVec::new((self.x << 1) + 1, (self.y << 1) + 1, self.depth + 1),
        }
    }

    #[inline]
    fn can_subdivide(self, node: Self, detail: u64) -> bool {
        // return early if the level of this chunk is too high
        if node.depth >= self.depth {
            return false;
        }

        // difference in lod level between the target and the node
        let level_difference = self.depth - node.depth;

        // minimum corner of the bounding box
        let min = (
            (node.x << (level_difference + 1)).saturating_sub(
                ((detail + 1) << level_difference) - (1 << level_difference),
            ),
            (node.y << (level_difference + 1)).saturating_sub(
                ((detail + 1) << level_difference) - (1 << level_difference),
            ),
        );

        // max as well
        let max = (
            (node.x << (level_difference + 1)).saturating_add(
                ((detail + 1) << level_difference) + (1 << level_difference),
            ),
            (node.y << (level_difference + 1)).saturating_add(
                ((detail + 1) << level_difference) + (1 << level_difference),
            ),
        );

        // local position of the target, which is one lod level higher to allow more detail
        let local = (self.x << 1, self.y << 1);

        // check if the target is inside of the bounding box
        local.0 >= min.0 && local.0 < max.0 && local.1 >= min.1 && local.1 < max.1
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Default, Debug)]
pub struct OctVec {
    /// x position in the octree
    pub x: u64,

    /// y position in the octree
    pub y: u64,

    /// z position in the octree
    pub z: u64,

    /// lod depth in the octree
    /// this is limited, hence we use u8
    pub depth: u8,
}

impl OctVec {
    #[inline]
    pub fn new(x: u64, y: u64, z: u64, depth: u8) -> Self {
        Self { x, y, z, depth }
    }
}

impl LodVec for OctVec {
    #[inline]
    fn num_children() -> usize {
        8
    }

    #[inline]
    fn root() -> Self {
        Self {
            x: 0,
            y: 0,
            z: 0,
            depth: 0,
        }
    }

    #[inline]
    fn get_child(self, index: usize) -> Self {
        match index {
            0 => Self::new(self.x << 1, self.y << 1, self.z << 1, self.depth + 1),
            1 => Self::new(self.x << 1, self.y << 1, (self.z << 1) + 1, self.depth + 1),
            2 => Self::new(self.x << 1, (self.y << 1) + 1, self.z << 1, self.depth + 1),
            3 => Self::new(
                self.x << 1,
                (self.y << 1) + 1,
                (self.z << 1) + 1,
                self.depth + 1,
            ),
            4 => Self::new((self.x << 1) + 1, self.y << 1, self.z << 1, self.depth + 1),
            5 => Self::new(
                (self.x << 1) + 1,
                self.y << 1,
                (self.z << 1) + 1,
                self.depth + 1,
            ),
            6 => Self::new(
                (self.x << 1) + 1,
                (self.y << 1) + 1,
                self.z << 1,
                self.depth + 1,
            ),
            _ => Self::new(
                (self.x << 1) + 1,
                (self.y << 1) + 1,
                (self.z << 1) + 1,
                self.depth + 1,
            ),
        }
    }

    #[inline]
    fn can_subdivide(self, node: Self, detail: u64) -> bool {
        // return early if the level of this chunk is too high
        if node.depth >= self.depth {
            return false;
        }

        // difference in lod level between the target and the node
        let level_difference = self.depth - node.depth;

        // minimum corner of the bounding box
        let min = (
            (node.x << (level_difference + 1)).saturating_sub(
                ((detail + 1) << level_difference) - (1 << level_difference),
            ),
            (node.y << (level_difference + 1)).saturating_sub(
                ((detail + 1) << level_difference) - (1 << level_difference),
            ),
            (node.z << (level_difference + 1)).saturating_sub(
                ((detail + 1) << level_difference) - (1 << level_difference),
            ),
        );

        // max as well
        let max = (
            (node.x << (level_difference + 1)).saturating_add(
                ((detail + 1) << level_difference) + (1 << level_difference),
            ),
            (node.y << (level_difference + 1)).saturating_add(
                ((detail + 1) << level_difference) + (1 << level_difference),
            ),
            (node.z << (level_difference + 1)).saturating_add(
                ((detail + 1) << level_difference) + (1 << level_difference),
            ),
        );

        // local position of the target
        let local = (self.x << 1, self.y << 1, self.z << 1);

        // check if the target is inside of the bounding box
        local.0 >= min.0
            && local.0 < max.0
            && local.1 >= min.1
            && local.1 < max.1
            && local.2 >= min.2
            && local.2 < max.2
    }
}
