use crate::lib::*;

#[cfg(feature = "async")]
pub use async_tree::Tree;
#[cfg(not(feature = "async"))]
pub use sync_tree::Tree;

#[cfg(feature = "async")]
mod async_tree;

#[cfg(not(feature = "async"))]
mod sync_tree;

/// The strategy to use when removing a node from the tree.
///
/// This enum represents the strategy to use when removing a node from the tree. The `RetainChildren`
/// strategy retains the children of the node when the node is removed. The `RemoveNodeAndChildren`
/// strategy removes the node and its children when the node is removed.
#[derive(Clone, Debug, Copy)]
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

/// The strategy to use when traversing the tree.
///
/// This enum represents the strategy to use when traversing the tree.
#[allow(clippy::enum_variant_names)]
#[derive(Clone, Debug, Copy)]
pub enum TraversalStrategy {
    /// Traverse the tree in pre-order. This means that the root node is visited first, then the left
    /// child, and then the right child.
    PreOrder,
    /// Traverse the tree in post-order. This means that the left child is visited first, then the right
    /// child, and then the root node.
    PostOrder,
    /// Traverse the tree in in-order. This means that the left child is visited first, then the root node,
    /// and then the right child.
    InOrder,
}

/// A subtree of a tree.
///
/// This struct represents a subtree of a tree. A subtree is a tree that is a part of a larger tree.
pub type SubTree<Q, T> = Tree<Q, T>;

#[cfg(test)]
mod tests {
    use crate::error::Error::{InvalidOperation, NodeNotFound, RootNodeAlreadyPresent};
    use crate::lib::*;
    #[allow(deprecated)]
    #[cfg(feature = "no_std")]
    use core::hash::SipHasher as DefaultHasher;
    #[cfg(not(feature = "no_std"))]
    use std::hash::DefaultHasher;

    use super::*;
    use crate::prelude::{Node, Result};

    #[test]
    fn test_tree_new() {
        let tree = Tree::<u32, u32>::new(Some("Sample Tree"));
        assert_eq!(tree.get_nodes().len(), 0);
    }

    #[test]
    fn test_tree_add_node() -> Result<()> {
        let mut tree = Tree::<u32, u32>::new(Some("Sample Tree"));
        let node_id = tree.add_node(Node::new(1, Some(2)), None)?;
        assert_eq!(tree.get_nodes().len(), 1);
        assert_eq!(node_id, 1);
        let node_id_2 = tree.add_node(Node::new(2, Some(3)), Some(&1))?;
        assert_eq!(tree.get_nodes().len(), 2);
        assert_eq!(node_id_2, 2);
        let node_2 = tree.get_node_by_id(&2).unwrap();
        assert_eq!(node_2.get_parent_id()?.unwrap(), 1);
        Ok(())
    }

    #[test]
    fn test_tree_add_more_than_one_root_node() {
        let mut tree = Tree::<u32, u32>::new(Some("Sample Tree"));
        let result = tree.add_node(Node::new(1, Some(2)), None);
        assert!(result.is_ok());
        let node_id = result.unwrap();
        assert_eq!(tree.get_nodes().len(), 1);
        assert_eq!(node_id, 1);
        let result = tree.add_node(Node::new(2, Some(3)), None);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), RootNodeAlreadyPresent);
    }

    #[test]
    fn test_tree_get_node() {
        let mut tree = Tree::<u32, u32>::new(Some("Sample Tree"));
        let node = Node::new(1, Some(2));
        tree.add_node(node.clone(), None).unwrap();
        assert_eq!(tree.get_node_by_id(&1), Some(node));
        assert_eq!(tree.get_node_by_id(&2), None);
    }

    #[test]
    fn test_tree_get_no_existent_node() {
        let tree = Tree::<u32, u32>::new(Some("Sample Tree"));
        assert_eq!(tree.get_node_by_id(&1), None);
    }

    #[test]
    fn test_tree_get_nodes() {
        let mut tree = Tree::<u32, u32>::new(Some("Sample Tree"));
        let node1 = Node::new(1, Some(2));
        let node2 = Node::new(2, Some(4));
        let node3 = Node::new(3, Some(7));
        let node1_id = tree.add_node(node1.clone(), None).unwrap();
        let node2_id = tree.add_node(node2.clone(), Some(&node1_id)).unwrap();
        let _ = tree.add_node(node3.clone(), Some(&node2_id)).unwrap();
        assert_eq!(tree.get_nodes().len(), 3);
    }

    #[test]
    fn test_tree_get_root_node() {
        let mut tree = Tree::<u32, u32>::new(Some("Sample Tree"));
        let node = Node::new(1, Some(2));
        tree.add_node(node.clone(), None).unwrap();
        assert_eq!(tree.get_root_node(), Some(node));
    }

    #[test]
    fn test_tree_get_node_height() {
        let mut tree = Tree::<u32, u32>::new(Some("Sample Tree"));
        let node_1 = tree.add_node(Node::new(1, Some(2)), None).unwrap();
        let node_2 = tree.add_node(Node::new(2, Some(3)), Some(&node_1)).unwrap();
        let node_3 = tree.add_node(Node::new(3, Some(6)), Some(&node_2)).unwrap();
        assert_eq!(tree.get_node_height(&node_1).unwrap(), 2);
        assert_eq!(tree.get_node_height(&node_2).unwrap(), 1);
        assert_eq!(tree.get_node_height(&node_3).unwrap(), 0);
    }

    #[test]
    fn test_tree_get_node_height_no_existent_node() {
        let tree = Tree::<u32, u32>::new(Some("Sample Tree"));
        let result = tree.get_node_height(&1);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), NodeNotFound("1".to_string()));
    }

    #[test]
    fn test_tree_get_node_depth() {
        let mut tree = Tree::<u32, u32>::new(Some("Sample Tree"));
        let node_1 = tree.add_node(Node::new(1, Some(2)), None).unwrap();
        let node_2 = tree.add_node(Node::new(2, Some(3)), Some(&node_1)).unwrap();
        let node_3 = tree.add_node(Node::new(3, Some(6)), Some(&node_2)).unwrap();
        assert_eq!(tree.get_node_depth(&node_3).unwrap(), 2);
        assert_eq!(tree.get_node_depth(&node_2).unwrap(), 1);
        assert_eq!(tree.get_node_depth(&node_1).unwrap(), 0);
    }

    #[test]
    fn test_tree_get_ancestor_ids() {
        let mut tree = Tree::<u32, u32>::new(Some("Sample Tree"));
        let node_1 = tree.add_node(Node::new(1, Some(2)), None).unwrap();
        let node_2 = tree.add_node(Node::new(2, Some(3)), Some(&node_1)).unwrap();
        let node_3 = tree.add_node(Node::new(3, Some(6)), Some(&node_2)).unwrap();
        let node_4 = tree.add_node(Node::new(4, Some(5)), Some(&node_2)).unwrap();
        assert_eq!(tree.get_ancestor_ids(&node_4).unwrap(), vec![2, 1]);
        assert_eq!(tree.get_ancestor_ids(&node_3).unwrap(), vec![2, 1]);
        assert_eq!(tree.get_ancestor_ids(&node_2).unwrap(), vec![1]);
        assert_eq!(tree.get_ancestor_ids(&node_1).unwrap(), Vec::<u32>::new());
    }

    #[test]
    fn test_tree_get_node_ancestor_ids_no_existent_node() {
        let tree = Tree::<u32, u32>::new(Some("Sample Tree"));
        let result = tree.get_ancestor_ids(&1);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), NodeNotFound("1".to_string()));
    }

    #[test]
    fn test_tree_get_node_depth_no_existent_node() {
        let tree = Tree::<u32, u32>::new(Some("Sample Tree"));
        let result = tree.get_node_depth(&1);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), NodeNotFound("1".to_string()));
    }

    #[test]
    fn test_tree_get_height() {
        let mut tree = Tree::<u32, u32>::new(Some("Sample Tree"));
        let node_1 = tree.add_node(Node::new(1, Some(2)), None).unwrap();
        let node_2 = tree.add_node(Node::new(2, Some(3)), Some(&node_1)).unwrap();
        tree.add_node(Node::new(3, Some(6)), Some(&node_2)).unwrap();
        assert_eq!(tree.get_height().unwrap(), 2);
    }

    #[test]
    fn test_tree_get_height_no_root_node() {
        let tree = Tree::<u32, u32>::new(Some("Sample Tree"));
        let result = tree.get_height();
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            InvalidOperation("Tree has no root node".to_string())
        );
    }

    #[test]
    fn test_tree_get_node_degree() {
        let mut tree = Tree::<u32, u32>::new(Some("Sample Tree"));
        let node_1 = tree.add_node(Node::new(1, Some(2)), None).unwrap();
        let node_2 = tree.add_node(Node::new(2, Some(3)), Some(&node_1)).unwrap();
        let node_3 = tree.add_node(Node::new(3, Some(6)), Some(&node_1)).unwrap();
        assert_eq!(tree.get_node_degree(&node_1).unwrap(), 2);
        assert_eq!(tree.get_node_degree(&node_2).unwrap(), 0);
        assert_eq!(tree.get_node_degree(&node_3).unwrap(), 0);
    }

    #[test]
    fn test_tree_get_node_degree_no_existent_node() {
        let tree = Tree::<u32, u32>::new(Some("Sample Tree"));
        let result = tree.get_node_degree(&1);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), NodeNotFound("1".to_string()));
    }

    #[test]
    fn test_tree_remove_node() -> Result<()> {
        let mut tree = Tree::<u32, u32>::new(Some("Sample Tree"));
        let node = Node::new(1, Some(2));
        tree.add_node(node.clone(), None)?;
        let node_2 = Node::new(2, Some(3));
        tree.add_node(node_2.clone(), Some(&1))?;
        let node_3 = Node::new(3, Some(6));
        tree.add_node(node_3.clone(), Some(&2))?;
        tree.remove_node(&2, NodeRemovalStrategy::RetainChildren)?;
        assert_eq!(tree.get_nodes().len(), 2);
        let node_4 = Node::new(4, Some(5));
        let node_5 = Node::new(5, Some(12));
        tree.add_node(node_4.clone(), Some(&3))?;
        tree.add_node(node_5.clone(), Some(&3))?;
        tree.remove_node(&3, NodeRemovalStrategy::RemoveNodeAndChildren)?;
        assert_eq!(tree.get_nodes().len(), 1);
        Ok(())
    }

    #[test]
    fn test_tree_remove_node_no_existent_node() {
        let mut tree = Tree::<u32, u32>::new(Some("Sample Tree"));
        let result = tree.remove_node(&1, NodeRemovalStrategy::RetainChildren);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), NodeNotFound("1".to_string()));
    }

    #[test]
    fn test_tree_remove_node_no_root_node() {
        let mut tree = Tree::<u32, u32>::new(Some("Sample Tree"));
        tree.add_node(Node::new(1, Some(2)), None).unwrap();
        let result = tree.remove_node(&1, NodeRemovalStrategy::RetainChildren);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            InvalidOperation("Cannot remove root node with RetainChildren strategy".to_string())
        );
    }

    #[test]
    fn test_tree_get_subsection() {
        let mut tree = Tree::<u32, u32>::new(Some("Sample Tree"));
        let node = Node::new(1, Some(2));
        tree.add_node(node.clone(), None).unwrap();
        let node_2 = Node::new(2, Some(3));
        tree.add_node(node_2.clone(), Some(&1)).unwrap();
        let node_3 = Node::new(3, Some(6));
        tree.add_node(node_3.clone(), Some(&2)).unwrap();
        let node_4 = Node::new(4, Some(5));
        tree.add_node(node_4.clone(), Some(&2)).unwrap();
        let node_5 = Node::new(5, Some(6));
        tree.add_node(node_5.clone(), Some(&3)).unwrap();
        let subsection = tree.get_subtree(&2, None).unwrap();
        assert_eq!(subsection.get_name(), Some("2"));
        assert_eq!(subsection.get_nodes().len(), 4);
        let subsection = tree.get_subtree(&2, Some(0)).unwrap();
        assert_eq!(subsection.get_nodes().len(), 1);
        let subsection = tree.get_subtree(&2, Some(1)).unwrap();
        assert_eq!(subsection.get_nodes().len(), 3);
    }

    #[test]
    fn test_tree_get_subsection_no_existent_node() {
        let tree = Tree::<u32, u32>::new(Some("Sample Tree"));
        let result = tree.get_subtree(&1, None);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), NodeNotFound("1".to_string()));
    }

    #[test]
    fn get_siblings() {
        let mut tree = Tree::<u32, u32>::new(Some("Sample Tree"));
        let node_1 = tree.add_node(Node::new(1, Some(2)), None).unwrap();
        let node_2 = tree.add_node(Node::new(2, Some(3)), Some(&node_1)).unwrap();
        tree.add_node(Node::new(3, Some(6)), Some(&node_1)).unwrap();
        tree.add_node(Node::new(4, Some(7)), Some(&node_1)).unwrap();
        let siblings = tree.get_sibling_ids(&node_2, false).unwrap();
        assert_eq!(siblings.len(), 2);
        let siblings = tree.get_sibling_ids(&node_2, true).unwrap();
        assert_eq!(siblings.len(), 3);
    }

    #[test]
    fn test_tree_get_siblings_no_existent_node() {
        let tree = Tree::<u32, u32>::new(Some("Sample Tree"));
        let result = tree.get_sibling_ids(&1, false);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), NodeNotFound("1".to_string()));
    }

    #[test]
    fn test_tree_add_subsection() {
        let mut tree = Tree::<u32, u32>::new(Some("Sample Tree"));
        let node_id = tree.add_node(Node::new(1, Some(2)), None).unwrap();
        let mut subtree = SubTree::<u32, u32>::new(Some("Sample Tree"));
        let node_2 = subtree.add_node(Node::new(2, Some(3)), None).unwrap();
        subtree
            .add_node(Node::new(3, Some(6)), Some(&node_2))
            .unwrap();
        tree.add_subtree(&node_id, subtree).unwrap();
        assert_eq!(tree.get_nodes().len(), 3);
    }

    #[test]
    fn test_tree_add_subsection_no_attaching_node() {
        let mut tree = Tree::<u32, u32>::new(Some("Sample Tree"));
        let mut subtree = SubTree::<u32, u32>::new(Some("Sample Tree"));
        let node_2 = subtree.add_node(Node::new(2, Some(3)), None).unwrap();
        subtree
            .add_node(Node::new(3, Some(6)), Some(&node_2))
            .unwrap();
        let result = tree.add_subtree(&1, subtree);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), NodeNotFound("1".to_string()));
    }

    #[test]
    fn test_tree_add_subsection_with_no_root_node() -> Result<()> {
        let mut tree = Tree::<u32, u32>::new(Some("Sample Tree"));
        let node_id = tree.add_node(Node::new(1, Some(2)), None)?;
        let mut subtree = SubTree::<u32, u32>::new(Some("Sample Tree"));
        let node_2 = Node::new(2, Some(3));
        let result = subtree.add_node(Node::new(3, Some(3)), Some(&node_2.get_node_id()?));
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), NodeNotFound("2".to_string()));
        let result = tree.add_subtree(&node_id, subtree);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            InvalidOperation("Subtree has no root node.".to_string())
        );
        Ok(())
    }

    #[test]
    fn test_tree_display() {
        let mut tree = Tree::<u32, u32>::new(Some("Sample Tree"));
        let node_1 = tree.add_node(Node::new(1, Some(2)), None).unwrap();
        let node_2 = tree.add_node(Node::new(2, Some(3)), Some(&node_1)).unwrap();
        let node_3 = tree.add_node(Node::new(3, Some(6)), Some(&node_2)).unwrap();
        tree.add_node(Node::new(4, Some(5)), Some(&node_2)).unwrap();
        tree.add_node(Node::new(5, Some(6)), Some(&node_3)).unwrap();
        #[cfg(feature = "print_node_id")]
		let expected_str = "Sample Tree\n***********\n1: 2\n└── 2: 3\n    ├── 3: 6\n    │   └── 5: 6\n    └── 4: 5\n";
        #[cfg(not(feature = "print_node_id"))]
        let expected_str =
            "Sample Tree\n***********\n2\n└── 3\n    ├── 6\n    │   └── 6\n    └── 5\n";

        assert_eq!(tree.to_string(), expected_str);
    }

    #[test]
    fn compare_tree() {
        let mut tree = Tree::<u32, u32>::new(Some("Sample Tree"));
        let node_1 = tree.add_node(Node::new(1, Some(2)), None).unwrap();
        let node_2 = tree.add_node(Node::new(2, Some(3)), Some(&node_1)).unwrap();
        let node_3 = tree.add_node(Node::new(3, Some(6)), Some(&node_2)).unwrap();
        tree.add_node(Node::new(4, Some(5)), Some(&node_2)).unwrap();
        tree.add_node(Node::new(5, Some(6)), Some(&node_3)).unwrap();
        let mut tree_2 = Tree::<u32, u32>::new(Some("Sample Tree"));
        let node_1 = tree_2.add_node(Node::new(1, Some(2)), None).unwrap();
        let node_2 = tree_2
            .add_node(Node::new(2, Some(3)), Some(&node_1))
            .unwrap();
        let node_3 = tree_2
            .add_node(Node::new(3, Some(6)), Some(&node_2))
            .unwrap();
        tree_2
            .add_node(Node::new(4, Some(5)), Some(&node_2))
            .unwrap();
        tree_2
            .add_node(Node::new(5, Some(6)), Some(&node_3))
            .unwrap();
        assert_eq!(tree, tree_2);
        let tree_3 = Tree::<u32, u32>::new(Some("Sample Tree"));
        assert_ne!(tree, tree_3);
    }

    #[test]
    fn test_tree_traverse() {
        let mut tree = Tree::<u32, u32>::new(Some("Sample Tree"));
        let node_1 = tree.add_node(Node::new(1, Some(2)), None).unwrap();
        let node_2 = tree.add_node(Node::new(2, Some(3)), Some(&node_1)).unwrap();
        let node_3 = tree.add_node(Node::new(3, Some(6)), Some(&node_1)).unwrap();
        let node_4 = tree.add_node(Node::new(4, Some(5)), Some(&node_2)).unwrap();
        let node_5 = tree.add_node(Node::new(5, Some(6)), Some(&node_2)).unwrap();
        let node_6 = tree.add_node(Node::new(6, Some(7)), Some(&node_3)).unwrap();
        let preorder_nodes = tree.traverse(&node_1, TraversalStrategy::PreOrder).unwrap();
        let expected_preorder = vec![node_1, node_2, node_4, node_5, node_3, node_6];
        assert_eq!(preorder_nodes, expected_preorder);

        let in_order_nodes = tree.traverse(&node_1, TraversalStrategy::InOrder).unwrap();
        let expected_in_order = vec![node_4, node_2, node_5, node_1, node_3, node_6];
        assert_eq!(in_order_nodes, expected_in_order);

        let post_order_nodes = tree
            .traverse(&node_1, TraversalStrategy::PostOrder)
            .unwrap();
        let expected_post_order = vec![node_4, node_5, node_2, node_6, node_3, node_1];
        assert_eq!(post_order_nodes, expected_post_order);
    }

    #[allow(deprecated)] // This is solely for testing hashing in no_std.
    #[test]
    fn test_hashing() {
        let mut tree = Tree::<u32, u32>::new(Some("Sample Tree"));
        let node_1 = tree.add_node(Node::new(1, Some(2)), None).unwrap();
        let node_2 = tree.add_node(Node::new(2, Some(3)), Some(&node_1)).unwrap();
        let node_3 = tree.add_node(Node::new(3, Some(6)), Some(&node_2)).unwrap();
        tree.add_node(Node::new(4, Some(5)), Some(&node_2)).unwrap();
        tree.add_node(Node::new(5, Some(6)), Some(&node_3)).unwrap();
        let mut tree_2 = Tree::<u32, u32>::new(Some("Sample Tree"));
        let node_1 = tree_2.add_node(Node::new(1, Some(2)), None).unwrap();
        let node_2 = tree_2
            .add_node(Node::new(2, Some(3)), Some(&node_1))
            .unwrap();
        let node_3 = tree_2
            .add_node(Node::new(3, Some(6)), Some(&node_2))
            .unwrap();
        tree_2
            .add_node(Node::new(4, Some(5)), Some(&node_2))
            .unwrap();
        tree_2
            .add_node(Node::new(5, Some(6)), Some(&node_3))
            .unwrap();
        assert_eq!(tree, tree_2);
        let mut hasher = DefaultHasher::new();
        tree.hash(&mut hasher);
        let tree_hash = hasher.finish();
        let mut hasher = DefaultHasher::new();
        tree_2.hash(&mut hasher);
        let tree_2_hash = hasher.finish();
        assert_eq!(tree_hash, tree_2_hash);
    }

    #[test]
    fn test_sort_children_by_height() -> Result<()> {
        let mut tree = Tree::<u32, u32>::new(Some("Sample Tree"));
        let node_1 = tree.add_node(Node::new(1, Some(1)), None)?;
        let _node_2 = tree.add_node(Node::new(2, Some(2)), Some(&node_1))?;
        let node_3 = tree.add_node(Node::new(3, Some(3)), Some(&node_1))?;
        let node_4 = tree.add_node(Node::new(4, Some(4)), Some(&node_3))?;
        let _node_5 = tree.add_node(Node::new(5, Some(5)), Some(&node_4))?;

        let root = tree.get_node_by_id(&node_1).unwrap();
        root.sort_children(|a, b| {
            let a_height = tree.get_node_height(a).unwrap();
            let b_height = tree.get_node_height(b).unwrap();
            a_height.cmp(&b_height).reverse()
        })?;

        assert_eq!(
            tree.get_node_by_id(&node_1).unwrap().get_children_ids()?,
            vec![3, 2]
        );
        Ok(())
    }
}

#[cfg(all(test, feature = "serde"))]
mod serde_tests {
    use crate::prelude::Node;

    use super::*;

    #[test]
    fn test_tree_serialize_and_deserialize() {
        let mut tree = Tree::new(None);
        let node_1 = tree.add_node(Node::new(1, Some(2)), None).unwrap();
        let node_2 = tree.add_node(Node::new(2, Some(3)), Some(&node_1)).unwrap();
        let node_3 = tree.add_node(Node::new(3, Some(6)), Some(&node_2)).unwrap();
        tree.add_node(Node::new(4, Some(5)), Some(&node_2)).unwrap();
        tree.add_node(Node::new(5, Some(6)), Some(&node_3)).unwrap();
        let serialized = serde_json::to_string(&tree).unwrap();
        let expected = r#"{"nodes":[{"node_id":1,"value":2,"parent":null,"children":[2]},{"node_id":2,"value":3,"parent":1,"children":[3,4]},{"node_id":3,"value":6,"parent":2,"children":[5]},{"node_id":4,"value":5,"parent":2,"children":[]},{"node_id":5,"value":6,"parent":3,"children":[]}]}"#;
        let deserialized: Tree<u32, u32> = serde_json::from_str(&serialized).unwrap();
        let expected_tree: Tree<u32, u32> = serde_json::from_str(expected).unwrap();
        assert_eq!(deserialized, expected_tree);
    }

    #[test]
    #[cfg_attr(not(feature = "compact_serde"), ignore)]
    fn test_tree_compact_serialize() {
        let mut tree = Tree::new(None);
        let node_1 = tree.add_node(Node::new(1, Some(2)), None).unwrap();
        let node_2 = tree.add_node(Node::new(2, Some(3)), Some(&node_1)).unwrap();
        let node_3 = tree.add_node(Node::new(3, Some(6)), Some(&node_2)).unwrap();
        tree.add_node(Node::new(4, Some(5)), Some(&node_2)).unwrap();
        tree.add_node(Node::new(5, Some(6)), Some(&node_3)).unwrap();
        let serialized = serde_json::to_string(&tree).unwrap();
        let expected = r#"{"nodes":[{"node_id":1,"value":2,"parent":null},{"node_id":2,"value":3,"parent":1},{"node_id":3,"value":6,"parent":2},{"node_id":4,"value":5,"parent":2},{"node_id":5,"value":6,"parent":3}]}"#;
        assert_eq!(serialized, expected);
    }

    #[test]
    #[cfg_attr(not(feature = "compact_serde"), ignore)]
    fn test_tree_compact_deserialize() {
        let tree_str = r#"{"nodes":[{"node_id":1,"value":2,"parent":null},{"node_id":2,"value":3,"parent":1},{"node_id":3,"value":6,"parent":2},{"node_id":4,"value":5,"parent":2},{"node_id":5,"value":6,"parent":3}]}"#;
        let deserialized: Tree<u32, u32> = serde_json::from_str(tree_str).unwrap();
        let mut tree = Tree::new(None);
        let node_1 = tree.add_node(Node::new(1, Some(2)), None).unwrap();
        let node_2 = tree.add_node(Node::new(2, Some(3)), Some(&node_1)).unwrap();
        let node_3 = tree.add_node(Node::new(3, Some(6)), Some(&node_2)).unwrap();
        tree.add_node(Node::new(4, Some(5)), Some(&node_2)).unwrap();
        tree.add_node(Node::new(5, Some(6)), Some(&node_3)).unwrap();
        assert_eq!(deserialized, tree);
    }

    #[cfg(feature = "auto_id")]
    #[test]
    fn test_tree_serialize_and_deserialize_with_auto_id_ensuring_uniqueness() {
        let mut tree = Tree::<crate::prelude::AutomatedId, i32>::new(Some("Sample Tree"));
        let root = tree
            .add_node(Node::new_with_auto_id(Some(2)), None)
            .unwrap();
        let child_1 = tree
            .add_node(Node::new_with_auto_id(Some(3)), Some(&root))
            .unwrap();
        let child_2 = tree
            .add_node(Node::new_with_auto_id(Some(4)), Some(&child_1))
            .unwrap();
        let child_3 = tree
            .add_node(Node::new_with_auto_id(Some(5)), Some(&child_2))
            .unwrap();
        let serialized_tree = serde_json::to_string(&tree).unwrap();
        let mut deserialized_tree: Tree<crate::prelude::AutomatedId, i32> =
            serde_json::from_str(&serialized_tree).unwrap();
        deserialized_tree
            .add_node(Node::new_with_auto_id(Some(6)), Some(&child_3))
            .unwrap();
        let mut node_ids = deserialized_tree
            .get_nodes()
            .iter()
            .map(|node| node.get_node_id().unwrap())
            .collect::<Vec<_>>();
        node_ids.sort();
        node_ids.dedup();
        assert_eq!(node_ids.len(), deserialized_tree.get_nodes().len());
    }

    #[cfg(feature = "auto_id")]
    #[test]
    #[cfg_attr(feature = "no_std", ignore)]
    fn test_tree_deserialize_from_disk_with_auto_id_ensuring_uniqueness() {
        let tree_str = serde_json::json!({"name":"Sample Tree","nodes":[{"node_id":3,"value":2,"children":[4],"parent":null},{"node_id":4,"value":3,"children":[5],"parent":3},{"node_id":5,"value":4,"children":[6],"parent":4},{"node_id":6,"value":5,"children":[],"parent":5}]});
        let mut deserialized_tree =
            serde_json::from_value::<Tree<crate::prelude::AutomatedId, i32>>(tree_str).unwrap();
        deserialized_tree
            .add_node(Node::new_with_auto_id(Some(6)), Some(&6))
            .unwrap();
        let mut node_ids = deserialized_tree
            .get_nodes()
            .iter()
            .map(|node| node.get_node_id().unwrap())
            .collect::<Vec<_>>();
        node_ids.sort();
        node_ids.dedup();
        assert_eq!(node_ids.len(), deserialized_tree.get_nodes().len());
    }
}
