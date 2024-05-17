//! # Tree-DS
//! A simple tree data structure implementation in Rust.
//!
//! ## Features
//! - **default**: By default the library is synchronous.
//! - **async**: Enables support for async operations on the tree.
//!
//! ## Usage
//!
//! ```rust
//! use tree_ds::prelude::*;
//!
//!
//! let mut tree: Tree<i32, i32> = Tree::new();
//! let root = tree.add_node(Node::new(1, Some(2)), None);
//! let child_1 = tree.add_node(Node::new(2, Some(3)), Some(root));
//! let child_2 = tree.add_node(Node::new(3, Some(4)), Some(child_1));
//! let child_3 = tree.add_node(Node::new(4, Some(5)), Some(child_2));
//! let sub_tree = tree.get_sub_tree(child_2);
//!
//! ```

mod tree;
mod node;

pub mod prelude {
	pub use crate::{
		node::Node,
		tree::{NodeRemovalStrategy, SubTree, Tree},
	};
}
