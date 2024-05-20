use std::fmt::Display;

#[cfg(feature = "serde")]
use serde::ser::SerializeStruct;
#[cfg(feature = "serde")]
use serde::Serialize;

use crate::error::Error::RootNodeAlreadyPresent;
use crate::prelude::Node;

/// The strategy to use when removing a node from the tree.
///
/// This enum represents the strategy to use when removing a node from the tree. The `RetainChildren`
/// strategy retains the children of the node when the node is removed. The `RemoveNodeAndChildren`
/// strategy removes the node and its children when the node is removed.
#[derive(Clone, Debug)]
pub enum NodeRemovalStrategy {
	/// Retain the children of the node. This means that the children of the node are attached to the
	/// parent of the node when the node is removed. So the children of the node become children of the
	/// parent of the node.
	RetainChildren,
	/// Remove the node and all subsequent children. This means that the node and its children are
	/// removed from the tree when the node is removed. All the subsequent grand children of the node are
	/// removed from the tree.
	RemoveNodeAndChildren,
}

pub type SubTree<Q, T> = Tree<Q, T>;

/// A tree data structure.
///
/// This struct represents a tree data structure. A tree is a data structure that consists of nodes
/// connected by edges. Each node has a parent node and zero or more child nodes. The tree has a root
/// node that is the topmost node in the tree. The tree can be used to represent hierarchical data
/// structures such as file systems, organization charts, and family trees.
///
/// # Example
///
/// ```rust
/// # use tree_ds::prelude::Tree;
///
/// let tree: Tree<i32, i32> = Tree::new();
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Tree<Q, T> where Q: PartialEq + Eq + Clone, T: PartialEq + Eq + Clone {
	nodes: Vec<Node<Q, T>>,
}

impl<Q, T> Tree<Q, T> where Q: PartialEq + Eq + Clone + Display, T: PartialEq + Eq + Clone {
	/// Create a new tree.
	///
	/// This method creates a new tree with no nodes.
	///
	/// # Returns
	///
	/// A new tree with no nodes.
	///
	/// # Example
	///
	/// ```rust
	/// # use tree_ds::prelude::Tree;
	///
	/// let tree: Tree<i32, i32> = Tree::new();
	/// ```
	pub fn new() -> Self {
		Tree::default()
	}

	/// Add a node to the tree.
	///
	/// This method adds a node to the tree. The node is added as a child of the parent node with the
	/// given parent id. If the parent id is `None`, the node is added as a root node. The node id is
	/// used to identify the node and the value is the value of the node. The value can be used to store
	/// any data that you want to associate with the node.
	///
	/// # Arguments
	///
	/// * `node_id` - The id of the node.
	/// * `value` - The value of the node.
	/// * `parent_id` - The id of the parent node. If `None`, the node is added as a root node.
	///
	/// # Returns
	///
	/// The id of the node that was added to the tree. However, if no parent id is provided and the tree already
	/// has a root node, an error is returned.
	///
	/// # Example
	///
	/// ```rust
	/// # use tree_ds::prelude::{Tree, Node};
	///
	/// let mut tree: Tree<i32, i32> = Tree::new();
	/// let node_id = tree.add_node(Node::new(1, Some(2)), None).unwrap();
	/// ```
	pub fn add_node(&mut self, node: Node<Q, T>, parent_id: Option<Q>) -> crate::prelude::Result<Q> {
		if let Some(parent_id) = parent_id {
			if let Some(parent) = self.nodes
									  .iter_mut()
									  .find(|n| n.get_node_id() == parent_id) {
				parent.add_child(node.clone());
			}
		} else if self.get_root_node().is_some() {
			return Err(RootNodeAlreadyPresent);
		}
		self.nodes.push(node.clone());
		Ok(node.get_node_id())
	}

	/// Get a node in the tree.
	///
	/// This method gets the node with the given node id in the tree.
	///
	/// # Arguments
	///
	/// * `node_id` - The id of the node.
	///
	/// # Returns
	///
	/// The node with the given node id in the tree or `None` if the node is not found.
	///
	/// # Example
	///
	/// ```rust
	/// # use tree_ds::prelude::{Node, Tree};
	///
	/// let mut tree: Tree<i32, i32> = Tree::new();
	///
	/// let node = Node::new(1, Some(2));
	/// tree.add_node(node.clone(), None).unwrap();
	///
	/// assert_eq!(tree.get_node(1), Some(node));
	/// ```
	pub fn get_node(&self, node_id: Q) -> Option<Node<Q, T>> {
		self.nodes
			.iter()
			.find(|n| n.get_node_id() == node_id).cloned()
	}

	/// Get the root node of the tree.
	///
	/// This method gets the root node of the tree. The root node is the topmost node in the tree. The
	/// root node has no parent node.
	///
	/// # Returns
	///
	/// The root node of the tree or `None` if the tree has no root node.
	///
	/// # Example
	///
	/// ```rust
	/// # use tree_ds::prelude::{Node, Tree};
	///
	/// let mut tree: Tree<i32, i32> = Tree::new();
	///
	/// let node = Node::new(1, Some(2));
	/// tree.add_node(node.clone(), None).unwrap();
	///
	/// assert_eq!(tree.get_root_node(), Some(node));
	/// ```
	pub fn get_root_node(&self) -> Option<Node<Q, T>> {
		self.nodes.iter().find(|n| n.get_parent().is_none()).cloned()
	}

	/// Get the nodes in the tree.
	///
	/// This method gets the nodes in the tree.
	///
	/// # Returns
	///
	/// The nodes in the tree.
	///
	/// # Example
	///
	/// ```rust
	/// # use tree_ds::prelude::{Node, Tree};
	///
	/// let mut tree: Tree<i32, i32> = Tree::new();
	///
	/// let node = Node::new(1, Some(2));
	/// tree.add_node(node.clone(), None).unwrap();
	///
	/// assert_eq!(tree.get_nodes().len(), 1);
	/// ```
	pub fn get_nodes(&self) -> Vec<Node<Q, T>> {
		self.nodes.clone()
	}

	/// Remove a node from the tree.
	///
	/// This method removes a node from the tree. The node is removed using the given removal strategy.
	/// The removal strategy determines how the node and its children are removed from the tree. The
	/// `RetainChildren` strategy retains the children of the node when the node is removed. The
	/// `RemoveNodeAndChildren` strategy removes the node and its children when the node is removed.
	///
	/// # Arguments
	///
	/// * `node_id` - The id of the node to remove.
	/// * `strategy` - The strategy to use when removing the node.
	///
	/// # Example
	///
	/// ```rust
	/// # use tree_ds::prelude::{Node, Tree, NodeRemovalStrategy};
	///
	/// let mut tree: Tree<i32, i32> = Tree::new();
	///
	/// let node = Node::new(1, Some(2));
	/// tree.add_node(node.clone(), None).unwrap();
	/// let node_2 = Node::new(2, Some(3));
	/// tree.add_node(node_2.clone(), Some(1)).unwrap();
	/// let node_3 = Node::new(3, Some(6));
	/// tree.add_node(node_3.clone(), Some(2)).unwrap();
	///
	/// tree.remove_node(2, NodeRemovalStrategy::RetainChildren);
	/// assert_eq!(tree.get_nodes().len(), 2);
	pub fn remove_node(&mut self, node_id: Q, strategy: NodeRemovalStrategy) {
		match strategy {
			NodeRemovalStrategy::RetainChildren => {
				let node = self.get_node(node_id.clone()).unwrap();
				let parent_node = node.get_parent().unwrap();
				parent_node.remove_child(node.clone());
				let children = node.get_children();
				for child in children {
					parent_node.add_child(child.clone());
				}
				self.nodes.retain(|n| n.get_node_id() != node_id);
			}
			NodeRemovalStrategy::RemoveNodeAndChildren => {
				let node = self.get_node(node_id.clone()).unwrap();
				let children = node.get_children();
				if let Some(parent) = node.get_parent() {
					parent.remove_child(node.clone());
				}
				self.nodes.retain(|n| n.get_node_id() != node_id);
				for child in children {
					node.remove_child(child.clone());
					self.remove_node(child.get_node_id(), strategy.clone());
				}
			}
		}
	}

	/// Get a subsection of the tree.
	///
	/// This method gets a subsection of the tree starting from the node with the given node id. The
	/// subsection is a list of nodes that are descendants of the node with the given node id upto the
	/// given number of descendants. If the number of descendants is `None`, all the descendants of the
	/// node are included in the subsection.
	///
	/// # Arguments
	///
	/// * `node_id` - The id of the node to get the subsection from.
	/// * `generations` - The number of descendants to include in the subsection. If `None`, all the
	/// descendants of the node are included in the subsection.
	///
	/// # Returns
	///
	/// The subsection of the tree starting from the node with the given node id.
	///
	/// # Example
	///
	/// ```rust
	/// # use tree_ds::prelude::{Node, Tree};
	///
	/// # let mut tree: Tree<i32, i32> = Tree::new();
	///
	/// let node = Node::new(1, Some(2));
	/// tree.add_node(node.clone(), None).unwrap();
	/// let node_2 = Node::new(2, Some(3));
	/// tree.add_node(node_2.clone(), Some(1)).unwrap();
	/// let node_3 = Node::new(3, Some(6));
	/// tree.add_node(node_3.clone(), Some(2)).unwrap();
	///
	/// let subsection = tree.get_subtree(2, None);
	/// assert_eq!(subsection.get_nodes().len(), 2);
	/// ```
	pub fn get_subtree(&self, node_id: Q, generations: Option<i32>) -> SubTree<Q, T> {
		let mut subsection = Vec::new();
		if let Some(node) = self.get_node(node_id) {
			subsection.push(node.clone());
			// Get the subsequent children of the node recursively for the number of generations and add them to the subsection.
			if let Some(generations) = generations {
				let children = node.get_children();
				for current_generation in 0..generations {
					for child in children.clone() {
						subsection.append(&mut self.get_subtree(child.get_node_id(), Some(current_generation)).get_nodes());
					}
				}
			} else {
				let children = node.get_children();
				for child in children {
					subsection.append(&mut self.get_subtree(child.get_node_id(), None).get_nodes());
				}
			}
		}

		SubTree {
			nodes: subsection
		}
	}

	/// Add a subsection to the tree.
	///
	/// This method adds a subsection to the tree. The subsection is a list of nodes that are descendants
	/// of the node with the given node id. The subsection is added as children of the node with the
	/// given node id.
	///
	/// # Arguments
	///
	/// * `node_id` - The id of the node to add the subsection to.
	/// * `subtree` - The subsection to add to the tree.
	///
	/// # Example
	///
	/// ```rust
	/// # use tree_ds::prelude::{Node, Tree, SubTree};
	///
	/// let mut tree: Tree<i32, i32> = Tree::new();
	/// let node_id = tree.add_node(Node::new(1, Some(2)), None).unwrap();
	/// let mut subtree = SubTree::new();
	/// subtree.add_node(Node::new(2, Some(3)), None).unwrap();
	/// subtree.add_node(Node::new(3, Some(6)), Some(2)).unwrap();
	/// tree.add_subtree(node_id, subtree);
	/// assert_eq!(tree.get_nodes().len(), 3);
	/// ```
	pub fn add_subtree(&mut self, node_id: Q, subtree: SubTree<Q, T>) {
		let node = self.get_node(node_id).unwrap();
		// Get the root node in the subsection and add it as a child of the node.
		let subtree_nodes = subtree.get_nodes();
		let root_node = subtree.get_root_node().unwrap();
		node.add_child(root_node.clone());
		self.nodes.append(&mut subtree_nodes.clone());
	}

	/// Print the tree.
	///
	/// This method prints the tree to the standard output.
	fn print_tree(f: &mut std::fmt::Formatter<'_>, node: &Node<Q, T>, level: usize, mut is_within: (bool, usize), is_last_child: bool) -> std::fmt::Result where Q: PartialEq + Eq + Clone + Display, T: PartialEq + Eq + Clone + Display + Default {
		for x in 1..level {
			if is_within.0 && x == is_within.1 {
				write!(f, "│   ")?;
			} else {
				write!(f, "    ")?;
			}
		}
		if level > 0 {
			if is_last_child {
				writeln!(f, "└── {}", node)?;
			} else {
				writeln!(f, "├── {}", node)?;
			}
		} else {
			writeln!(f, "{}", node)?;
		}
		let children = node.get_children();
		let children_count = children.len();
		for (index, child) in children.iter().enumerate() {
			let last_item = index == children_count - 1;
			// Check if parent was last child
			let is_parent_last_item = if let Some(parent) = node.get_parent() {
				parent.get_children()
					  .last().unwrap()
					  .get_node_id() == node.get_node_id()
			} else {
				true
			};
			if !is_within.0 {
				is_within.0 = !is_parent_last_item;
				is_within.1 = level;
			} else {
				is_within.1 = if level > 0 { level - 1 } else { level };
			}
			Tree::print_tree(f, child, level + 1, (is_within.0, is_within.1), last_item)?;
		}
		Ok(())
	}
}

impl<Q, T> Default for Tree<Q, T> where Q: PartialEq + Eq + Clone, T: PartialEq + Eq + Clone {
	fn default() -> Self {
		Tree {
			nodes: Vec::new(),
		}
	}
}

impl<Q, T> Display for Tree<Q, T> where Q: PartialEq + Eq + Clone + Display, T: PartialEq + Eq + Clone + Display + Default {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		if let Some(node) = self.get_root_node() {
			Tree::print_tree(f, &node, 0, (false, 0), true)?;
		} else {
			let root = self.nodes.first().unwrap();
			Tree::print_tree(f, root, 0, (false, 0), true)?;
		}
		Ok(())
	}
}

#[cfg(feature = "serde")]
impl<Q, T> Serialize for Tree<Q, T> where Q: PartialEq + Eq + Clone + Serialize, T: PartialEq + Eq + Clone + Serialize {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
		let mut serialized_struct = serializer.serialize_struct("Tree", 1)?;
		serialized_struct.serialize_field("nodes", &self.nodes)?;
		serialized_struct.end()
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_tree_new() {
		let tree = Tree::<u32, u32>::new();
		assert_eq!(tree.nodes.len(), 0);
	}

	#[test]
	fn test_tree_add_node() {
		let mut tree = Tree::new();
		let node_id = tree.add_node(Node::new(1, Some(2)), None).unwrap();
		assert_eq!(tree.nodes.len(), 1);
		assert_eq!(node_id, 1);
		let node_id_2 = tree.add_node(Node::new(2, Some(3)), Some(1)).unwrap();
		assert_eq!(tree.nodes.len(), 2);
		assert_eq!(node_id_2, 2);
		let node_2 = tree.get_node(2).unwrap();
		assert_eq!(node_2.get_parent().unwrap().get_node_id(), 1);
	}

	#[test]
	fn test_tree_get_node() {
		let mut tree = Tree::new();
		let node = Node::new(1, Some(2));
		tree.add_node(node.clone(), None).unwrap();
		assert_eq!(tree.get_node(1), Some(node));
		assert_eq!(tree.get_node(2), None);
	}

	#[test]
	fn test_tree_get_nodes() {
		let mut tree = Tree::new();
		let node = Node::new(1, Some(2));
		tree.add_node(node.clone(), None).unwrap();
		assert_eq!(tree.get_nodes().len(), 1);
	}

	#[test]
	fn test_tree_remove_node() {
		let mut tree = Tree::new();
		let node = Node::new(1, Some(2));
		tree.add_node(node.clone(), None).unwrap();
		let node_2 = Node::new(2, Some(3));
		tree.add_node(node_2.clone(), Some(1)).unwrap();
		let node_3 = Node::new(3, Some(6));
		tree.add_node(node_3.clone(), Some(2)).unwrap();
		tree.remove_node(2, NodeRemovalStrategy::RetainChildren);
		assert_eq!(tree.get_nodes().len(), 2);
		let node_4 = Node::new(4, Some(5));
		let node_5 = Node::new(5, Some(12));
		tree.add_node(node_4.clone(), Some(3)).unwrap();
		tree.add_node(node_5.clone(), Some(3)).unwrap();
		tree.remove_node(3, NodeRemovalStrategy::RemoveNodeAndChildren);
		assert_eq!(tree.get_nodes().len(), 1);
	}

	#[test]
	fn test_tree_get_subsection() {
		let mut tree = Tree::new();
		let node = Node::new(1, Some(2));
		tree.add_node(node.clone(), None).unwrap();
		let node_2 = Node::new(2, Some(3));
		tree.add_node(node_2.clone(), Some(1)).unwrap();
		let node_3 = Node::new(3, Some(6));
		tree.add_node(node_3.clone(), Some(2)).unwrap();
		let node_4 = Node::new(4, Some(5));
		tree.add_node(node_4.clone(), Some(2)).unwrap();
		let node_5 = Node::new(5, Some(6));
		tree.add_node(node_5.clone(), Some(3)).unwrap();
		let subsection = tree.get_subtree(2, None);
		assert_eq!(subsection.get_nodes().len(), 4);
		let subsection = tree.get_subtree(2, Some(0));
		assert_eq!(subsection.get_nodes().len(), 1);
		let subsection = tree.get_subtree(2, Some(1));
		assert_eq!(subsection.get_nodes().len(), 3);
	}

	#[test]
	fn test_tree_add_subsection() {
		let mut tree = Tree::new();
		let node_id = tree.add_node(Node::new(1, Some(2)), None).unwrap();
		let mut subtree = SubTree::new();
		subtree.add_node(Node::new(2, Some(3)), None).unwrap();
		subtree.add_node(Node::new(3, Some(6)), Some(2)).unwrap();
		tree.add_subtree(node_id, subtree);
		assert_eq!(tree.get_nodes().len(), 3);
	}

	#[test]
	fn test_tree_display() {
		let mut tree = Tree::new();
		let node = Node::new(1, Some(2));
		tree.add_node(node.clone(), None).unwrap();
		let node_2 = Node::new(2, Some(3));
		tree.add_node(node_2.clone(), Some(1)).unwrap();
		let node_3 = Node::new(3, Some(6));
		tree.add_node(node_3.clone(), Some(2)).unwrap();
		let node_4 = Node::new(4, Some(5));
		tree.add_node(node_4.clone(), Some(2)).unwrap();
		let node_5 = Node::new(5, Some(6));
		tree.add_node(node_5.clone(), Some(3)).unwrap();
		let expected_str = "1: 2\n└── 2: 3\n    ├── 3: 6\n    │   └── 5: 6\n    └── 4: 5\n";
		assert_eq!(tree.to_string(), expected_str);
	}

	#[test]
	fn compare_tree() {
		let mut tree = Tree::new();
		let node = Node::new(1, Some(2));
		tree.add_node(node.clone(), None).unwrap();
		let node_2 = Node::new(2, Some(3));
		tree.add_node(node_2.clone(), Some(1)).unwrap();
		let node_3 = Node::new(3, Some(6));
		tree.add_node(node_3.clone(), Some(2)).unwrap();
		let node_4 = Node::new(4, Some(5));
		tree.add_node(node_4.clone(), Some(2)).unwrap();
		let node_5 = Node::new(5, Some(6));
		tree.add_node(node_5.clone(), Some(3)).unwrap();
		let mut tree_2 = Tree::new();
		let node = Node::new(1, Some(2));
		tree_2.add_node(node.clone(), None).unwrap();
		let node_2 = Node::new(2, Some(3));
		tree_2.add_node(node_2.clone(), Some(1)).unwrap();
		let node_3 = Node::new(3, Some(6));
		tree_2.add_node(node_3.clone(), Some(2)).unwrap();
		let node_4 = Node::new(4, Some(5));
		tree_2.add_node(node_4.clone(), Some(2)).unwrap();
		let node_5 = Node::new(5, Some(6));
		tree_2.add_node(node_5.clone(), Some(3)).unwrap();
		assert_eq!(tree, tree_2);
	}
}
