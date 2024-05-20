//! # Tree-DS
//! A simple tree data structure implementation in Rust.
//!
//! ## Cargo Features
//! - `default`: By default the library is synchronous.
//! - `async`: Enables support for async operations on the tree.
//! - `serde`: Enables serialization and deserialization of the tree.
//!
//! ## Usage
//!
//! ```rust
//! use tree_ds::prelude::*;
//!
//!
//! let mut tree: Tree<i32, i32> = Tree::new();
//! let root = tree.add_node(Node::new(1, Some(2)), None).unwrap();
//! let child_1 = tree.add_node(Node::new(2, Some(3)), Some(root)).unwrap();
//! let child_2 = tree.add_node(Node::new(3, Some(4)), Some(child_1)).unwrap();
//! let child_3 = tree.add_node(Node::new(4, Some(5)), Some(child_2)).unwrap();
//! let sub_tree = tree.get_subtree(child_2, None);
//!
//! ```

mod tree;
mod node;
mod error;

pub mod prelude {
	//! A module to re-export the necessary types for the tree data structure.

	pub use crate::{
		node::Node,
		tree::{NodeRemovalStrategy, SubTree, Tree},
	};

	pub type Result<T> = std::result::Result<T, crate::error::Error>;
}
