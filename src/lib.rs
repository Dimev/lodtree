//! LodTree, a simple tree data structure for doing chunk-based level of detail
//! The aim of this crate is to provide a generic, easy to use tree data structure that can be used to make Lod Quadtrees, Octrees and more
//! 
//! Internally, the tree tries to keep as much memory allocated, to avoid the cost of heap allocation, and stores the actual chunks data seperate from the tree data
//! 
//! This does come at a cost, mainly, only the chunks that are going to be added and their locations can be retreived as a slice, although for most (procedural) terrain implementation
//! making new chunks and editing them will be the highest cost to do, so that shouldn't be the biggest issue

// TODO: example with drawing

pub mod coords;
pub mod traits;
pub mod tree;

pub use crate::traits::*;
pub use crate::tree::*;

