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
//! let child_1 = tree.add_node(Node::new(2, Some(3)), Some(&root)).unwrap();
//! let child_2 = tree.add_node(Node::new(3, Some(4)), Some(&child_1)).unwrap();
//! let child_3 = tree.add_node(Node::new(4, Some(5)), Some(&child_2)).unwrap();
//! let sub_tree = tree.get_subtree(&child_2, None);
//!
//! ```
//!
//! ## Traversal
//! The tree supports three traversal strategies:
//! - Pre-order
//! - Post-order
//! - In-order
//!
//! Consider the following tree:
//! ```text
//! Node 1: 2
//! └── Node 2: 3
//!    └── Node 3: 4
//!        └── Node 4: 5
//! ```
//!
//! You can modify nodes during traversal by using the iterator from the returned traversal data.
//!
//! ```rust
//! use tree_ds::prelude::*;
//!
//! let mut tree = Tree::new();
//! let root = tree.add_node(Node::new("Node 1", Some(2)), None).unwrap();
//! let child_1 = tree.add_node(Node::new("Node 2", Some(3)), Some(&root)).unwrap();
//! let child_2 = tree.add_node(Node::new("Node 3", Some(4)), Some(&child_1)).unwrap();
//! let child_3 = tree.add_node(Node::new("Node 4", Some(5)), Some(&child_2)).unwrap();
//!;
//! tree.traverse(TraversalStrategy::PreOrder, &root)
//!   .iter()
//!   .for_each(|node_id| {
//!     let node = tree.get_node(node_id).unwrap();
//!     let cur_value = node.get_value().unwrap();
//!     node.set_value(Some(cur_value + 1));
//! });
//! 
//! # assert_eq!("Node 1: 3\n└── Node 2: 4\n    └── Node 3: 5\n        └── Node 4: 6\n", tree.to_string());
//! # assert_eq!(tree.get_node(&root).unwrap().get_value().unwrap(), 3);
//! ```
//!
//! The newly modified tree will be:
//! ```text
//! Node 1: 3
//! └── Node 2: 4
//!    └── Node 3: 5
//!        └── Node 4: 6
//! ```

mod error;
mod node;
mod tree;

pub mod prelude {
	//! A module to re-export the necessary types for the tree data structure.

    pub use crate::{
        node::Node,
        tree::{NodeRemovalStrategy, SubTree, TraversalStrategy, Tree},
    };

    pub type Result<T> = std::result::Result<T, crate::error::Error>;
}
