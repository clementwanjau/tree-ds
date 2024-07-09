use ::serde::{Deserialize, ser::SerializeStruct, Serialize};

use crate::lib::*;
use crate::node::Nodes;
use crate::tree::Tree;

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

#[cfg(test)]
mod tests {
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
            .map(|node| node.get_node_id())
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
            .map(|node| node.get_node_id())
            .collect::<Vec<_>>();
        node_ids.sort();
        node_ids.dedup();
        assert_eq!(node_ids.len(), deserialized_tree.get_nodes().len());
    }
}
