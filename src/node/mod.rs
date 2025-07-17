use crate::lib::*;
#[cfg(feature = "async")]
pub use crate::node::async_node::{Node, Nodes};
#[cfg(feature = "serde")]
use ::serde::{ser::SerializeStruct, Deserialize, Serialize};

#[cfg(not(feature = "async"))]
pub use crate::node::sync_node::{Node, Nodes};

#[cfg(feature = "async")]
mod async_node;

#[cfg(not(feature = "async"))]
mod sync_node;

#[cfg(all(feature = "no_std", feature = "auto_id"))]
lazy_static::lazy_static! {
    static ref GENERATOR: sequential_gen::prelude::SimpleGenerator<u128> =
        sequential_gen::prelude::SimpleGenerator::new(1u128);
}

#[cfg(all(feature = "auto_id", not(feature = "no_std")))]
pub const GENERATOR: sequential_gen::prelude::EpochBasedGenerator =
    sequential_gen::prelude::EpochBasedGenerator;

#[doc(hidden)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct _Node<Q, T>
where
    Q: PartialEq + Eq + Clone,
    T: PartialEq + Eq + Clone,
{
    /// The user supplied id of the node.
    node_id: Q,
    /// The value of the node.
    value: Option<T>,
    /// The children of the node.
    children: Vec<Q>,
    /// The parent of the node.
    parent: Option<Q>,
}

#[cfg(feature = "serde")]
impl<Q, T> Serialize for _Node<Q, T>
where
    Q: PartialEq + Eq + Clone + Serialize,
    T: PartialEq + Eq + Clone + Serialize,
{
    /// Serialize the node.
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("Node", 4)?;
        state.serialize_field("node_id", &self.node_id)?;
        state.serialize_field("value", &self.value)?;
        #[cfg(not(feature = "compact_serde"))]
        state.serialize_field("children", &self.children)?;
        state.serialize_field("parent", &self.parent)?;
        state.end()
    }
}

#[cfg(feature = "serde")]
impl<'de, Q, T> Deserialize<'de> for _Node<Q, T>
where
    Q: PartialEq + Eq + Clone + Deserialize<'de>,
    T: PartialEq + Eq + Clone + Deserialize<'de>,
{
    /// Deserialize the node.
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[cfg(not(feature = "compact_serde"))]
        #[derive(Deserialize)]
        struct Node<Q, T> {
            node_id: Q,
            value: Option<T>,
            children: Vec<Q>,
            parent: Option<Q>,
        }

        #[cfg(feature = "compact_serde")]
        #[derive(Deserialize)]
        struct Node<Q, T> {
            node_id: Q,
            value: Option<T>,
            parent: Option<Q>,
        }

        let node: Node<Q, T> = Deserialize::deserialize(deserializer)?;

        #[cfg(feature = "compact_serde")]
        let children = vec![];
        #[cfg(not(feature = "compact_serde"))]
        let children = node.children;

        Ok(_Node {
            node_id: node.node_id,
            value: node.value,
            children,
            parent: node.parent,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::lib::*;

    use super::*;
    use crate::prelude::Result;

    #[test]
    fn test_node_new() -> Result<()> {
        let node = Node::new(1, Some(2));
        assert_eq!(node.get_node_id()?, 1);
        assert_eq!(node.get_value()?, Some(2));
        assert_eq!(node.get_children_ids()?.len(), 0);
        assert!(node.get_parent_id()?.is_none());
        Ok(())
    }

    #[test]
    fn test_node_adding_children() -> Result<()> {
        let node = Node::new(1, Some(2));
        let child = Node::new(2, Some(3));
        node.add_child(child)?;
        assert_eq!(node.get_children_ids()?.len(), 1);
        Ok(())
    }

    #[test]
    fn test_node_get_node_id() -> Result<()> {
        let node = Node::new(1, Some(2));
        assert_eq!(node.get_node_id()?, 1);
        Ok(())
    }

    #[test]
    fn test_node_get_parent() -> Result<()> {
        let parent_node = Node::new(1, Some(2));
        let child_node = Node::new(2, Some(3));
        parent_node.add_child(child_node.clone())?;
        assert_eq!(
            child_node.get_parent_id()?.as_ref(),
            Some(&parent_node.get_node_id()?)
        );
        assert!(parent_node.get_parent_id()?.is_none());
        Ok(())
    }

    #[test]
    fn test_node_get_value() -> Result<()> {
        let node = Node::new(1, Some(2));
        assert_eq!(node.get_value()?, Some(2));
        Ok(())
    }

    #[test]
    fn test_node_set_value() -> Result<()> {
        let node = Node::new(1, Some(2));
        assert_eq!(node.get_value()?, Some(2));
        node.set_value(Some(3))?;
        assert_eq!(node.get_value()?, Some(3));
        Ok(())
    }

    #[test]
    fn test_node_set_parent() -> Result<()> {
        let parent_node = Node::new(1, Some(2));
        let child_node = Node::new(2, Some(3));
        child_node.set_parent(Some(parent_node.clone()))?;
        assert_eq!(
            child_node.get_parent_id()?.as_ref(),
            Some(&parent_node.get_node_id()?)
        );
        Ok(())
    }

    #[test]
    fn test_node_remove_child() -> Result<()> {
        let parent_node = Node::new(1, Some(2));
        let child_node = Node::new(2, Some(3));
        parent_node.add_child(child_node.clone())?;
        parent_node.remove_child(child_node)?;
        assert_eq!(parent_node.get_children_ids()?.len(), 0);
        Ok(())
    }

    #[test]
    fn test_node_update_value() -> Result<()> {
        let node = Node::new(1, Some(2));
        node.update_value(|value| *value = value.map(|x| x + 1))?;
        assert_eq!(node.get_value()?, Some(3));
        Ok(())
    }

    #[test]
    fn test_node_eq() {
        let node1 = Node::new(1, Some(2));
        let node2 = Node::new(1, Some(2));
        assert_eq!(node1, node2);
    }

    #[test]
    #[cfg_attr(not(feature = "print_node_id"), ignore)]
    fn test_node_display_with_id() {
        let node = Node::new(1, Some(2));
        assert_eq!(format!("{node}"), "1: 2");
    }

    #[test]
    #[cfg_attr(feature = "print_node_id", ignore)]
    fn test_node_display_without_id() {
        let node = Node::new(1, Some(2));
        assert_eq!(format!("{node}"), "2");
    }

    #[test]
    fn test_nodes() {
        let nodes = Nodes::new(vec![Node::new(1, Some(2))]);
        assert_eq!(nodes.len(), 1);
    }

    #[test]
    fn test_nodes_len() {
        let nodes = Nodes::new(vec![Node::new(1, Some(2))]);
        assert_eq!(nodes.len(), 1);
    }

    #[test]
    fn test_nodes_is_empty() {
        let nodes: Nodes<i32, String> = Nodes::default();
        assert!(nodes.is_empty());
    }

    #[test]
    fn test_nodes_get() -> Result<()> {
        let nodes = Nodes::new(vec![Node::new(1, Some(2))]);
        assert_eq!(nodes.get(0).unwrap().get_node_id()?, 1);
        Ok(())
    }

    #[test]
    fn test_nodes_get_by_node_id() -> Result<()> {
        let nodes = Nodes::new(vec![Node::new(1, Some(2))]);
        assert_eq!(nodes.get_by_node_id(&1).unwrap().get_node_id()?, 1);
        Ok(())
    }

    #[test]
    fn test_nodes_push() {
        let mut nodes = Nodes::new(vec![Node::new(1, Some(2))]);
        nodes.push(Node::new(2, Some(3)));
        assert_eq!(nodes.len(), 2);
    }

    #[test]
    fn test_nodes_remove() -> Result<()> {
        let mut nodes = Nodes::new(vec![Node::new(1, Some(2))]);
        let removed_node = nodes.remove(0);
        assert_eq!(removed_node.get_node_id()?, 1);
        assert_eq!(nodes.len(), 0);
        Ok(())
    }

    #[test]
    fn test_nodes_retain() {
        let mut nodes = Nodes::new(vec![Node::new(1, Some(2)), Node::new(2, Some(3))]);
        nodes.retain(|node| {
            node.get_node_id()
                .expect("Error: Failed to fetch the node Id")
                == 1
        });
        assert_eq!(nodes.len(), 1);
    }

    #[test]
    fn test_nodes_clear() {
        let mut nodes = Nodes::new(vec![Node::new(1, Some(2)), Node::new(2, Some(3))]);
        nodes.clear();
        assert_eq!(nodes.len(), 0);
    }

    #[test]
    fn test_nodes_append() {
        let mut nodes = Nodes::new(vec![Node::new(1, Some(2))]);
        let mut other_nodes = Nodes::new(vec![Node::new(2, Some(3))]);
        nodes.append(&mut other_nodes);
        assert_eq!(nodes.len(), 2);
    }

    #[test]
    fn test_nodes_append_raw() {
        let mut nodes = Nodes::new(vec![Node::new(1, Some(2))]);
        let mut other_nodes = vec![Node::new(2, Some(3))];
        nodes.append_raw(&mut other_nodes);
        assert_eq!(nodes.len(), 2);
    }

    #[test]
    fn test_nodes_first() -> Result<()> {
        let nodes = Nodes::new(vec![Node::new(1, Some(2)), Node::new(2, Some(3))]);
        assert_eq!(nodes.first().unwrap().get_node_id()?, 1);
        Ok(())
    }

    #[test]
    fn test_nodes_default() {
        let nodes: Nodes<i32, String> = Nodes::default();
        assert!(nodes.is_empty());
    }

    #[test]
    fn test_nodes_from_iter() {
        let nodes = Nodes::from_iter(vec![Node::new(1, Some(2))]);
        assert_eq!(nodes.len(), 1);
    }

    #[test]
    fn test_nodes_iterator() -> Result<()> {
        let nodes = Nodes::new(vec![Node::new(1, Some(2)), Node::new(2, Some(3))]);
        let mut iter = nodes.iter();
        assert_eq!(iter.next().unwrap().get_node_id()?, 1);
        assert_eq!(iter.next().unwrap().get_node_id()?, 2);
        Ok(())
    }

    #[test]
    fn test_nodes_next() -> Result<()> {
        let mut nodes = Nodes::new(vec![Node::new(1, Some(2)), Node::new(2, Some(3))]);
        assert_eq!(nodes.next().unwrap().get_node_id()?, 1);
        assert_eq!(nodes.next().unwrap().get_node_id()?, 2);
        Ok(())
    }

    #[test]
    fn test_nodes_size_hint() {
        let nodes = Nodes::new(vec![Node::new(1, Some(2))]);
        let (lower, upper) = nodes.size_hint();
        assert_eq!(lower, 1);
        assert_eq!(upper, Some(1));
    }

    #[test]
    fn test_nodes_as_ref() {
        let nodes = Nodes::new(vec![Node::new(1, Some(2))]);
        assert_eq!(nodes.as_ref().len(), 1);
    }

    #[test]
    fn test_nodes_eq() {
        let nodes1 = Nodes::new(vec![Node::new(1, Some(2))]);
        let nodes2 = Nodes::new(vec![Node::new(1, Some(2))]);
        assert_eq!(nodes1, nodes2);
    }

    #[test]
    fn test_nodes_display() {
        let nodes = Nodes::new(vec![Node::new(1, Some(2))]);
        #[cfg(feature = "print_node_id")]
        assert_eq!(format!("{nodes}"), "1: 2");
        #[cfg(not(feature = "print_node_id"))]
        assert_eq!(format!("{nodes}"), "2");
    }
}

#[cfg(all(test, feature = "serde"))]
mod serde_tests {
    use super::*;

    #[test]
    #[cfg_attr(feature = "compact_serde", ignore)]
    fn test_node_serialize() {
        let node = Node::new(1, Some(2));
        let serialized = serde_json::to_string(&node).unwrap();
        assert_eq!(
            serialized,
            r#"{"node_id":1,"value":2,"children":[],"parent":null}"#
        );
    }

    #[test]
    fn test_node_deserialize() {
        let node = Node::new(1, Some(2));
        let serialized = serde_json::to_string(&node).unwrap();
        let deserialized: Node<i32, i32> = serde_json::from_str(&serialized).unwrap();
        assert_eq!(node, deserialized);
    }

    #[test]
    #[cfg_attr(feature = "compact_serde", ignore)]
    fn test_nodes_serialize() {
        let nodes = Nodes::new(vec![Node::new(1, Some(2))]);
        let serialized = serde_json::to_string(&nodes).unwrap();
        assert_eq!(
            serialized,
            r#"[{"node_id":1,"value":2,"children":[],"parent":null}]"#
        );
    }

    #[test]
    #[cfg_attr(not(feature = "compact_serde"), ignore)]
    fn test_nodes_compact_serialize() {
        let root_node = Node::new(1, Some(2));
        let child_node = Node::new(2, Some(3));
        root_node
            .add_child(child_node.clone())
            .expect("Error: Could not add child to Node.");
        let nodes = Nodes::new(vec![root_node, child_node]);
        let serialized = serde_json::to_string(&nodes).unwrap();
        assert_eq!(
            serialized,
            r#"[{"node_id":1,"value":2,"parent":null},{"node_id":2,"value":3,"parent":1}]"#
        );
    }

    #[test]
    #[cfg_attr(not(feature = "compact_serde"), ignore)]
    fn test_nodes_compact_deserialize() {
        let serialized =
            r#"[{"node_id":1,"value":2,"parent":null},{"node_id":2,"value":3,"parent":1}]"#;
        let nodes = Nodes::new(vec![Node::new(1, Some(2)), Node::new(2, Some(3))]);
        let deserialized: Nodes<i32, i32> = serde_json::from_str(serialized).unwrap();
        assert_eq!(nodes, deserialized);
    }

    #[test]
    fn test_nodes_deserialize() {
        let nodes = Nodes::new(vec![Node::new(1, Some(2))]);
        let serialized = serde_json::to_string(&nodes).unwrap();
        let deserialized: Nodes<i32, i32> = serde_json::from_str(&serialized).unwrap();
        assert_eq!(nodes, deserialized);
    }
}

#[cfg(all(feature = "auto_id", test))]
mod auto_id_tests {
    use super::*;
    use crate::prelude::AutomatedId;
    use crate::prelude::Result;

    #[test]
    fn test_new_with_auto_id() -> Result<()> {
        let node = Node::<AutomatedId, &str>::new_with_auto_id(Some("Harry Doe"));
        let node_2 = Node::<AutomatedId, &str>::new_with_auto_id(Some("Jane Doe"));
        assert_eq!(node.get_value()?, Some("Harry Doe"));
        assert_ne!(node.get_node_id()?, node_2.get_node_id()?);
        Ok(())
    }
}
