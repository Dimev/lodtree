//! LodTree, a simple tree data structure for doing chunk-based level of detail
//! The aim of this crate is to provide a generic, easy to use tree data structure that can be used to make Lod Quadtrees, Octrees and more
//! 
//! Internally, the tree tries to keep as much memory allocated, to avoid the cost of heap allocation, and stores the actual chunks data seperate from the tree data
//! 

pub mod coords;
pub mod traits;
pub mod tree;

pub use crate::traits::*;
pub use crate::tree::*;

