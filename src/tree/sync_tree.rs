use crate::error::Error::{InvalidOperation, NodeNotFound, RootNodeAlreadyPresent};
use crate::lib::*;
use crate::node::{Node, Nodes};
use crate::prelude::{NodeRemovalStrategy, SubTree, TraversalStrategy};
#[cfg(feature = "serde")]
use ::serde::{ser::SerializeStruct, Deserialize, Serialize};

/// A tree data structure.
///
/// This struct represents a tree data structure. A tree is a data structure that consists of nodes
/// connected by edges. Each node has a parent node and zero or more child nodes. The tree has a root
/// node that is the topmost node in the tree. The tree can be used to represent hierarchical data
/// structures such as file systems, organization charts, and family trees. A tree can have any number
/// of nodes and each node can have any number of children. The tree can be traversed in different
/// orders such as pre-order, post-order, and in-order. The tree can be named for easy identification
/// when working with multiple trees or subtrees.
///
/// # Type Parameters
///
/// * `Q` - The type of the node id.
/// * `T` - The type of the node value.
///
/// # Example
///
/// ```rust
/// # use tree_ds::prelude::Tree;
///
/// let tree: Tree<i32, i32> = Tree::new(Some("Sample Tree"));
/// ```
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Tree<Q, T>
where
    Q: PartialEq + Eq + Clone,
    T: PartialEq + Eq + Clone,
{
    name: Option<String>,
    nodes: Nodes<Q, T>,
}

impl<Q, T> Tree<Q, T>
where
    Q: PartialEq + Eq + Clone + Display + Hash + Ord,
    T: PartialEq + Eq + Clone,
{
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
    /// let tree: Tree<i32, i32> = Tree::new(Some("Sample Tree"));
    /// ```
    pub fn new(tree_name: Option<&str>) -> Self {
        Self {
            name: tree_name.map(|x| x.to_string()),
            nodes: Nodes::default(),
        }
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
    /// let mut tree: Tree<i32, i32> = Tree::new(Some("Sample Tree"));
    /// let node_id = tree.add_node(Node::new(1, Some(2)), None);
    ///
    /// assert!(node_id.is_ok());
    /// // This should return an error because the tree already has a root node.
    /// let another_node_id = tree.add_node(Node::new(2, Some(3)), None);
    /// assert!(another_node_id.is_err());
    /// ```
    pub fn add_node(
        &mut self,
        node: Node<Q, T>,
        parent_id: Option<&Q>,
    ) -> crate::prelude::Result<Q> {
        if let Some(parent_id) = parent_id {
            let parent = self
                .nodes
                .iter()
                .find(|n| &n.get_node_id().expect("Error: Failed to get the node Id.") == parent_id)
                .ok_or(NodeNotFound(parent_id.to_string()))?;
            parent.add_child(node.clone())?;
        } else if self.get_root_node().is_some() {
            return Err(RootNodeAlreadyPresent);
        }
        self.nodes.push(node.clone());
        node.get_node_id()
    }

    /// Get the name of the tree.
    ///
    /// This method gets the name of the tree.
    ///
    /// # Returns
    ///
    /// The name of the tree.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use tree_ds::prelude::Tree;
    ///
    /// let tree: Tree<i32, i32> = Tree::new(Some("Sample Tree"));
    ///
    /// assert_eq!(tree.get_name(), Some("Sample Tree"));
    /// ```
    pub fn get_name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    /// Set the name of the tree.
    ///
    /// This method sets the name of the tree.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the tree.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use tree_ds::prelude::Tree;
    ///
    /// let mut tree: Tree<i32, i32> = Tree::new(Some("Sample Tree"));
    /// tree.rename(Some("New Name"));
    /// assert_eq!(tree.get_name(), Some("New Name"));
    /// ```
    pub fn rename(&mut self, name: Option<&str>) {
        self.name = name.map(|x| x.to_string());
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
    /// let mut tree: Tree<i32, i32> = Tree::new(Some("Sample Tree"));
    ///
    /// let node = Node::new(1, Some(2));
    /// let node_id = tree.add_node(node.clone(), None).unwrap();
    ///
    /// assert_eq!(tree.get_node_by_id(&node_id), Some(node));
    /// ```
    pub fn get_node_by_id(&self, node_id: &Q) -> Option<Node<Q, T>> {
        self.nodes
            .iter()
            .find(|n| &n.get_node_id().expect("Error: Failed to get the node Id.") == node_id)
            .cloned()
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
    /// let mut tree: Tree<i32, i32> = Tree::new(Some("Sample Tree"));
    ///
    /// let node = Node::new(1, Some(2));
    /// tree.add_node(node.clone(), None).unwrap();
    ///
    /// assert_eq!(tree.get_root_node(), Some(node));
    /// ```
    pub fn get_root_node(&self) -> Option<Node<Q, T>> {
        self.nodes
            .iter()
            .find(|n| {
                n.get_parent_id()
                    .expect("Error: Failed to get the node Id of the parent.")
                    .is_none()
            })
            .cloned()
    }

    /// Get the height of the node.
    ///
    /// This method gets the height of the node. The height of the node is the number of edges present
    /// in the longest path connecting the node to a leaf node.
    ///
    /// # Returns
    ///
    /// The height of the node. If the node is a leaf node, the height is 0.  This method returns an
    /// error if the node is not found in the tree.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use tree_ds::prelude::{Node, Tree};
    ///
    /// let mut tree: Tree<i32, i32> = Tree::new(Some("Sample Tree"));
    ///
    /// let node_1 = tree.add_node(Node::new(1, Some(2)), None).unwrap();
    /// let node_2 = tree.add_node(Node::new(2, Some(3)), Some(&node_1)).unwrap();
    /// let node_3 = tree.add_node(Node::new(3, Some(6)), Some(&node_2)).unwrap();
    ///
    /// assert!(tree.get_node_height(&node_2).is_ok());
    /// assert_eq!(tree.get_node_height(&node_2).unwrap(), 1);
    /// ```
    pub fn get_node_height(&self, node_id: &Q) -> crate::prelude::Result<i32> {
        let node = self
            .get_node_by_id(node_id)
            .ok_or(NodeNotFound(node_id.to_string()))?;
        let children = node.get_children_ids()?;
        if children.is_empty() {
            return Ok(0);
        }
        let mut height = 0;
        for child in children {
            let child_height = self.get_node_height(&child)?;
            if child_height > height {
                height = child_height;
            }
        }
        Ok(height + 1)
    }

    /// Get the depth of a node in the tree.
    ///
    /// This method gets the depth of a node in the tree. The depth of a node is the length of the path
    /// from the root node to the node. The depth of the node is the number of edges on the path from the
    /// root node to the node.
    ///
    /// # Arguments
    ///
    /// * `node_id` - The id of the node.
    ///
    /// # Returns
    ///
    /// The depth of the node in the tree.  This method returns an error if the node is not found in the tree.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use tree_ds::prelude::{Node, Tree};
    ///
    /// let mut tree: Tree<i32, i32> = Tree::new(Some("Sample Tree"));
    ///
    /// let node_1 = tree.add_node(Node::new(1, Some(2)), None).unwrap();
    /// let node_2 = tree.add_node(Node::new(2, Some(3)), Some(&node_1)).unwrap();
    /// let node_3 = tree.add_node(Node::new(3, Some(6)), Some(&node_2)).unwrap();
    /// let depth_result = tree.get_node_depth(&node_3);
    /// assert!(depth_result.is_ok());
    /// assert_eq!(depth_result.unwrap(), 2);
    /// ```
    pub fn get_node_depth(&self, node_id: &Q) -> crate::prelude::Result<i32> {
        let node = self
            .get_node_by_id(node_id)
            .ok_or(NodeNotFound(node_id.to_string()))?;
        let mut depth = 0;
        let mut parent = node.get_parent_id()?;
        while let Some(parent_id) = parent {
            depth += 1;
            parent = self
                .get_node_by_id(&parent_id)
                .ok_or(NodeNotFound(parent_id.to_string()))?
                .get_parent_id()?;
        }
        Ok(depth)
    }

    /// Get the ancestors of a node in the tree.
    ///
    /// This method gets the ancestors of a node in the tree. The ancestors of a node are all the nodes
    /// that are on the path from the root node to the node, not including the node itself.
    ///
    /// # Arguments
    ///
    /// * `node_id` - The id of the node.
    ///
    /// # Returns
    ///
    /// The ancestors of the node from closest to furthest.  This method returns an error if the node is not found in the tree.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use tree_ds::prelude::{Node, Tree};
    ///
    /// let mut tree: Tree<i32, i32> = Tree::new(Some("Sample Tree"));
    ///
    /// let node_1 = tree.add_node(Node::new(1, Some(2)), None).unwrap();
    /// let node_2 = tree.add_node(Node::new(2, Some(3)), Some(&node_1)).unwrap();
    /// let node_3 = tree.add_node(Node::new(3, Some(6)), Some(&node_2)).unwrap();
    /// let depth_result = tree.get_ancestor_ids(&node_3);
    /// assert!(depth_result.is_ok());
    /// assert_eq!(depth_result.unwrap(), vec![2, 1]);
    /// ```
    pub fn get_ancestor_ids(&self, node_id: &Q) -> crate::prelude::Result<Vec<Q>> {
        let node = self
            .get_node_by_id(node_id)
            .ok_or(NodeNotFound(node_id.to_string()))?;
        let mut ancestors = vec![];
        let mut parent = node.get_parent_id()?;
        while let Some(parent_id) = parent {
            ancestors.push(parent_id.clone());
            parent = self
                .get_node_by_id(&parent_id)
                .ok_or(NodeNotFound(parent_id.to_string()))?
                .get_parent_id()?;
        }
        Ok(ancestors)
    }

    /// Get the height of the tree.
    ///
    /// This method gets the height of the tree. The height of the tree is the length of the longest path
    /// from the root node to a leaf node. The height of the tree is the number of edges on the longest
    /// path from the root node to a leaf node.
    ///
    /// # Returns
    ///
    /// The height of the tree. This method returns an error if the tree has no root node.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use tree_ds::prelude::{Node, Tree, Result};
    ///
    /// # fn main() -> Result<()> {
    /// let mut tree: Tree<i32, i32> = Tree::new(Some("Sample Tree"));
    ///
    /// let node_1 = tree.add_node(Node::new(1, Some(2)), None)?;
    /// let node_2 = tree.add_node(Node::new(2, Some(3)), Some(&node_1))?;
    /// let node_3 = tree.add_node(Node::new(3, Some(6)), Some(&node_2))?;
    /// let tree_height = tree.get_height();
    /// assert!(tree_height.is_ok());
    /// assert_eq!(tree_height?, 2);
    /// # Ok(())
    /// # }
    /// ```
    pub fn get_height(&self) -> crate::prelude::Result<i32> {
        let root = self
            .get_root_node()
            .ok_or(InvalidOperation(String::from("Tree has no root node")))?;
        self.get_node_height(&root.get_node_id()?)
    }

    /// Get the degree of a node in the tree.
    ///
    /// This method gets the degree of a node in the tree. The degree of a node is the number of children
    /// that the node has.
    ///
    /// # Arguments
    ///
    /// * `node_id` - The id of the node.
    ///
    /// # Returns
    ///
    /// The degree of the node in the tree. This method returns an error if the node is not found in the tree.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use tree_ds::prelude::{Result, Node, Tree};
    ///
    /// # fn main() -> Result<()> {
    /// let mut tree: Tree<i32, i32> = Tree::new(Some("Sample Tree"));
    ///
    /// let node_1 = tree.add_node(Node::new(1, Some(2)), None)?;
    /// let node_2 = tree.add_node(Node::new(2, Some(3)), Some(&node_1))?;
    /// let node_3 = tree.add_node(Node::new(3, Some(6)), Some(&node_1))?;
    ///
    /// assert_eq!(tree.get_node_degree(&node_1)?, 2);
    /// assert_eq!(tree.get_node_degree(&node_2)?, 0);
    /// assert_eq!(tree.get_node_degree(&node_3)?, 0);
    /// # Ok(())
    /// # }
    /// ```
    pub fn get_node_degree(&self, node_id: &Q) -> crate::prelude::Result<i32> {
        let node = self
            .get_node_by_id(node_id)
            .ok_or(NodeNotFound(node_id.to_string()))?;
        Ok(node.get_children_ids()?.len() as i32)
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
    /// let mut tree: Tree<i32, i32> = Tree::new(Some("Sample Tree"));
    ///
    /// let node = Node::new(1, Some(2));
    /// tree.add_node(node.clone(), None).unwrap();
    ///
    /// assert_eq!(tree.get_nodes().len(), 1);
    /// ```
    pub fn get_nodes(&self) -> &Nodes<Q, T> {
        self.nodes.as_ref()
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
    /// # Returns
    /// An error if the node is not found in the tree or if the node is the root node and the removal
    /// strategy is `RetainChildren`.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use tree_ds::prelude::{Node, Tree, NodeRemovalStrategy, Result};
    ///
    /// # fn main() -> Result<()> {
    /// let mut tree: Tree<i32, i32> = Tree::new(Some("Sample Tree"));
    ///
    /// let node_1 = tree.add_node(Node::new(1, Some(2)), None)?;
    /// let node_2 = tree.add_node(Node::new(2, Some(3)), Some(&node_1))?;
    /// tree.add_node(Node::new(3, Some(6)), Some(&node_2))?;
    ///
    /// tree.remove_node(&node_2, NodeRemovalStrategy::RetainChildren)?;
    /// assert_eq!(tree.get_nodes().len(), 2);
    /// # Ok(())
    /// # }
    /// ```
    pub fn remove_node(
        &mut self,
        node_id: &Q,
        strategy: NodeRemovalStrategy,
    ) -> crate::prelude::Result<()> {
        match strategy {
            NodeRemovalStrategy::RetainChildren => {
                let node = self
                    .get_node_by_id(node_id)
                    .ok_or(NodeNotFound(node_id.to_string()))?;
                let parent_node_id = &node.get_parent_id()?.ok_or(InvalidOperation(
                    String::from("Cannot remove root node with RetainChildren strategy"),
                ))?;
                let parent_node = self
                    .get_node_by_id(parent_node_id)
                    .ok_or(NodeNotFound(parent_node_id.to_string()))?;
                parent_node.remove_child(node.clone())?;
                let children = node.get_children_ids()?;
                for child in children {
                    if let Some(child) = self.get_node_by_id(&child) {
                        parent_node.add_child(child)?;
                    }
                }
                self.nodes.retain(|n| {
                    &n.get_node_id().expect("Error: Failed to get the node Id.") != node_id
                });
                Ok(())
            }
            NodeRemovalStrategy::RemoveNodeAndChildren => {
                let node = self
                    .get_node_by_id(node_id)
                    .ok_or(NodeNotFound(node_id.to_string()))?;
                let children = node.get_children_ids()?;
                if let Some(parent_id) = node.get_parent_id()? {
                    let parent = self
                        .get_node_by_id(&parent_id)
                        .ok_or(NodeNotFound(parent_id.to_string()))?;
                    parent.remove_child(node.clone())?;
                }
                self.nodes.retain(|n| {
                    &n.get_node_id().expect("Error: Failed to get the node Id.") != node_id
                });
                for child in children {
                    let child = self
                        .get_node_by_id(&child)
                        .ok_or(NodeNotFound(child.to_string()))?;
                    node.remove_child(child.clone())?;
                    self.remove_node(&child.get_node_id()?, strategy)?;
                }
                Ok(())
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
    /// * `generations` - The number of descendants to include in the subsection. If `None`, all the descendants of the node are included in the subsection.
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
    /// # fn main() -> tree_ds::prelude::Result<()> {
    /// # let mut tree: Tree<i32, i32> = Tree::new(Some("Sample Tree"));
    ///
    /// let node_1 = tree.add_node(Node::new(1, Some(2)), None)?;
    /// let node_2 = tree.add_node(Node::new(2, Some(3)), Some(&node_1))?;
    /// let node_3 = tree.add_node(Node::new(3, Some(6)), Some(&node_2))?;
    ///
    /// let subsection = tree.get_subtree(&node_2, None)?;
    /// assert_eq!(subsection.get_nodes().len(), 2);
    /// # Ok(())
    /// # }
    /// ```
    pub fn get_subtree(
        &self,
        node_id: &Q,
        generations: Option<i32>,
    ) -> crate::prelude::Result<SubTree<Q, T>> {
        let mut subsection = Nodes::default();
        let node = self
            .get_node_by_id(node_id)
            .ok_or(NodeNotFound(node_id.to_string()))?;
        subsection.push(node.clone());
        // Get the subsequent children of the node recursively for the number of generations and add them to the subsection.
        if let Some(generations) = generations {
            let children = node.get_children_ids()?;
            for current_generation in 0..generations {
                for child in children.clone() {
                    subsection.append(
                        &mut self
                            .get_subtree(&child, Some(current_generation))?
                            .get_nodes()
                            .clone(),
                    );
                }
            }
        } else {
            let children = node.get_children_ids()?;
            for child in children {
                subsection.append(&mut self.get_subtree(&child, None)?.get_nodes().clone());
            }
        }

        Ok(SubTree {
            name: Some(node_id.to_string()),
            nodes: subsection,
        })
    }

    /// Get the siblings of a node in the tree.
    ///
    /// This method gets the siblings of a node in the tree. The siblings of a node are the children
    /// that share the same parent as the node.
    ///
    /// # Arguments
    ///
    /// * `node_id` - The id of the node to get the siblings of.
    /// * `inclusive` - A flag that indicates whether to include the node in the siblings list.
    ///
    /// # Returns
    ///
    /// The siblings of the node in the tree.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use tree_ds::prelude::{Node, Tree};
    ///
    /// # fn main() -> tree_ds::prelude::Result<()> {
    /// let mut tree: Tree<i32, i32> = Tree::new(Some("Sample Tree"));
    /// let node_1 = tree.add_node(Node::new(1, Some(2)), None)?;
    /// let node_2 = tree.add_node(Node::new(2, Some(3)), Some(&node_1))?;
    /// tree.add_node(Node::new(3, Some(6)), Some(&node_1))?;
    /// tree.add_node(Node::new(4, Some(7)), Some(&node_1))?;
    ///
    /// let siblings = tree.get_sibling_ids(&node_2, false)?;
    /// assert_eq!(siblings.len(), 2);
    ///
    /// let siblings = tree.get_sibling_ids(&node_2, true)?;
    /// assert_eq!(siblings.len(), 3);
    /// # Ok(())
    /// # }
    /// ```
    pub fn get_sibling_ids(&self, node_id: &Q, inclusive: bool) -> crate::prelude::Result<Vec<Q>> {
        let node = self
            .get_node_by_id(node_id)
            .ok_or(NodeNotFound(node_id.to_string()))?;
        if let Some(parent_id) = node.get_parent_id()? {
            let parent = self
                .get_node_by_id(&parent_id)
                .ok_or(NodeNotFound(parent_id.to_string()))?;
            if inclusive {
                parent.get_children_ids()
            } else {
                Ok(parent
                    .get_children_ids()?
                    .iter()
                    .filter(|x| *x != node_id)
                    .cloned()
                    .collect())
            }
        } else if inclusive {
            // We need to clone this since Q does not implement Copy.
            Ok(vec![node_id.clone()])
        } else {
            Ok(vec![])
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
    /// # Returns
    /// This function return an error if:
    /// - The node is not found in the tree.
    /// - The subsection has no root node.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use tree_ds::prelude::{Node, Tree, SubTree};
    ///
    /// # fn main() -> tree_ds::prelude::Result<()> {
    /// let mut tree: Tree<i32, i32> = Tree::new(Some("Sample Tree"));
    /// let node_id = tree.add_node(Node::new(1, Some(2)), None)?;
    /// let mut subtree = SubTree::new(Some("Sample Tree"));
    /// let node_2 = subtree.add_node(Node::new(2, Some(3)), None)?;
    /// subtree.add_node(Node::new(3, Some(6)), Some(&node_2))?;
    /// tree.add_subtree(&node_id, subtree)?;
    /// assert_eq!(tree.get_nodes().len(), 3);
    /// # Ok(())
    /// # }
    /// ```
    pub fn add_subtree(
        &mut self,
        node_id: &Q,
        subtree: SubTree<Q, T>,
    ) -> crate::prelude::Result<()> {
        let node = self
            .get_node_by_id(node_id)
            .ok_or(NodeNotFound(node_id.to_string()))?;
        // Get the root node in the subsection and add it as a child of the node.
        let subtree_nodes = subtree.get_nodes();
        let root_node = subtree
            .get_root_node()
            .ok_or(InvalidOperation(String::from("Subtree has no root node.")))?;
        node.add_child(root_node.clone())?;
        self.nodes.append(&mut subtree_nodes.clone());
        Ok(())
    }

    /// Traverse the subtree from the given node.
    ///
    /// This method traverses the subtree from the given node in the given order.
    ///
    /// # Arguments
    ///
    /// * `order` - The order to traverse the tree.
    /// * `node_id` - The id of the node to start the traversal from.
    ///
    /// # Returns
    ///
    /// The nodes in the tree in the given order. This method returns an error if the node is not found
    /// in the tree.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use tree_ds::prelude::{Node, Tree, TraversalStrategy};
    ///
    /// # fn main() -> tree_ds::prelude::Result<()> {
    /// let mut tree: Tree<i32, i32> = Tree::new(Some("Sample Tree"));
    /// let node_1 = tree.add_node(Node::new(1, Some(2)), None)?;
    /// let node_2 = tree.add_node(Node::new(2, Some(3)), Some(&node_1))?;
    /// let node_3 = tree.add_node(Node::new(3, Some(6)), Some(&node_2))?;
    ///
    /// let ordered_nodes = tree.traverse(&node_1, TraversalStrategy::PreOrder)?;
    /// # let expected = vec![1, 2, 3];
    /// # assert_eq!(ordered_nodes, expected);
    /// # Ok(())
    /// # }
    /// ```
    pub fn traverse(
        &self,
        node_id: &Q,
        order: TraversalStrategy,
    ) -> crate::prelude::Result<Vec<Q>> {
        let mut nodes = vec![];
        let node = self
            .get_node_by_id(node_id)
            .ok_or(NodeNotFound(node_id.to_string()))?;
        match &order {
            TraversalStrategy::PreOrder => {
                nodes.push(node_id.clone());
                for child_id in node.get_children_ids()?.iter() {
                    nodes.append(&mut self.traverse(child_id, order)?);
                }
            }
            TraversalStrategy::PostOrder => {
                for child_id in node.get_children_ids()?.iter() {
                    nodes.append(&mut self.traverse(child_id, order)?);
                }
                nodes.push(node_id.clone());
            }
            TraversalStrategy::InOrder => {
                for (index, child_id) in node.get_children_ids()?.iter().enumerate() {
                    if index == 0 {
                        nodes.append(&mut self.traverse(child_id, order)?);
                        if !nodes.contains(child_id) {
                            nodes.push(child_id.clone());
                        }
                        if !nodes.contains(node_id) {
                            nodes.push(node_id.clone());
                        }
                    } else {
                        nodes.push(child_id.clone());
                        nodes.append(&mut self.traverse(child_id, order)?);
                    }
                }
            }
        }
        #[cfg(not(feature = "no_std"))]
        let mut seen = HashSet::new();
        #[cfg(feature = "no_std")]
        let mut seen = BTreeSet::new();
        nodes.retain(|x| seen.insert(x.clone()));
        Ok(nodes)
    }

    /// Print the tree.
    ///
    /// This method prints the tree to the standard output.
    #[doc(hidden)]
    fn print_tree(tree: &Tree<Q, T>, f: &mut Formatter<'_>) -> crate::prelude::Result<()>
    where
        Q: PartialEq + Eq + Clone + Display + Hash,
        T: PartialEq + Eq + Clone + Display + Default,
    {
        Tree::print_sub_tree(
            tree,
            f,
            &tree.get_root_node().ok_or(FmtError)?,
            String::new(),
            true,
        )?;
        Ok(())
    }

    fn print_sub_tree(
        tree: &Tree<Q, T>,
        f: &mut Formatter<'_>,
        root_node: &Node<Q, T>,
        mut parent_prefix: String,
        is_last_child: bool,
    ) -> crate::prelude::Result<()>
    where
        Q: PartialEq + Eq + Clone + Display + Hash,
        T: PartialEq + Eq + Clone + Display + Default,
    {
        write!(f, "{parent_prefix}")?;
        if is_last_child {
            if tree
                .get_root_node()
                .is_some_and(|x| x.get_node_id() == root_node.get_node_id())
            {
                writeln!(f, "{root_node}")?;
            } else {
                writeln!(f, "└── {root_node}")?;
                parent_prefix = format!("{parent_prefix}    ");
            }
        } else {
            writeln!(f, "├── {root_node}")?;
            parent_prefix = format!("{parent_prefix}│   ");
        }
        let children = root_node.get_children_ids()?;
        for (index, node_id) in children.iter().enumerate() {
            let node = tree
                .get_node_by_id(node_id)
                .ok_or(NodeNotFound(node_id.to_string()))?;

            Tree::print_sub_tree(
                tree,
                f,
                &node,
                parent_prefix.clone(),
                index == children.len() - 1,
            )?;
        }
        Ok(())
    }
}

impl<Q, T> Default for Tree<Q, T>
where
    Q: PartialEq + Eq + Clone,
    T: PartialEq + Eq + Clone,
{
    /// Create a new tree with no nodes.
    fn default() -> Self {
        Tree {
            name: None,
            nodes: Nodes::default(),
        }
    }
}

impl<Q, T> Display for Tree<Q, T>
where
    Q: PartialEq + Eq + Clone + Display + Hash + Ord,
    T: PartialEq + Eq + Clone + Display + Default,
{
    /// Print the tree.
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        if let Some(name) = &self.name {
            writeln!(f, "{name}")?;
            writeln!(
                f,
                "{}",
                name.clone().chars().map(|_| "*").collect::<String>()
            )?;
        }
        Tree::print_tree(self, f).map_err(|_| FmtError)?;
        Ok(())
    }
}

impl<Q, T> Drop for Tree<Q, T>
where
    Q: PartialEq + Eq + Clone,
    T: PartialEq + Eq + Clone,
{
    /// Drop the tree.
    #[doc(hidden)]
    fn drop(&mut self) {
        self.nodes.clear();
    }
}

#[cfg(feature = "serde")]
impl<Q, T> Serialize for Tree<Q, T>
where
    Q: PartialEq + Eq + Clone + Serialize,
    T: PartialEq + Eq + Clone + Serialize,
{
    /// Serialize the tree.
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        if self.name.is_none() {
            let mut state = serializer.serialize_struct("Tree", 1)?;
            state.serialize_field("nodes", &self.nodes)?;
            return state.end();
        }
        let mut state = serializer.serialize_struct("Tree", 2)?;
        state.serialize_field("name", &self.name)?;
        state.serialize_field("nodes", &self.nodes)?;
        state.end()
    }
}

#[cfg(feature = "serde")]
impl<'de, Q, T> Deserialize<'de> for Tree<Q, T>
where
    Q: PartialEq + Eq + Clone + Deserialize<'de>,
    T: PartialEq + Eq + Clone + Deserialize<'de>,
{
    /// Deserialize the tree.
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct TreeVisitor<Q, T>
        where
            Q: PartialEq + Eq + Clone,
            T: PartialEq + Eq + Clone,
        {
            name: Option<String>,
            nodes: Nodes<Q, T>,
        }

        let tree_visitor: TreeVisitor<Q, T> = Deserialize::deserialize(deserializer)?;
        let tree = Tree {
            name: tree_visitor.name,
            nodes: tree_visitor.nodes,
        };
        Ok(tree)
    }
}
