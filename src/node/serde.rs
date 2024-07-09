use ::serde::{Deserialize, ser::SerializeStruct, Serialize};

use crate::lib::*;
#[cfg(feature = "async")]
use crate::lib::Arc;
#[cfg(not(feature = "async"))]
use crate::lib::Rc;
use crate::node::{_Node, Node, Nodes};

impl<Q, T> Serialize for Node<Q, T>
where
    Q: PartialEq + Eq + Clone + Serialize,
    T: PartialEq + Eq + Clone + Serialize,
{
    /// Serialize the node.
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.0.borrow().serialize(serializer)
    }
}

impl<'de, Q, T> Deserialize<'de> for Node<Q, T>
where
    Q: PartialEq + Eq + Clone + Deserialize<'de>,
    T: PartialEq + Eq + Clone + Deserialize<'de>,
{
    /// Deserialize the node.
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let node: _Node<Q, T> = Deserialize::deserialize(deserializer)?;

        #[cfg(not(feature = "async"))]
        return Ok(Node(Rc::new(RefCell::new(node))));
        #[cfg(feature = "async")]
        return Ok(Node(Arc::new(RefCell::new(node))));
    }
}

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

impl<Q, T> Serialize for Nodes<Q, T>
where
    Q: PartialEq + Eq + Clone + Serialize,
    T: PartialEq + Eq + Clone + Serialize,
{
    /// Serialize the nodes list.
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.0.serialize(serializer)
    }
}

impl<'de, Q, T> Deserialize<'de> for Nodes<Q, T>
where
    Q: PartialEq + Eq + Clone + Deserialize<'de>,
    T: PartialEq + Eq + Clone + Deserialize<'de>,
{
    /// Deserialize the nodes list.
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let nodes: Vec<Node<Q, T>> = Deserialize::deserialize(deserializer)?;
        if cfg!(feature = "compact_serde") {
            // Rebuild the children data from the parent data.
            for node in nodes.iter() {
                // Find the parent of this node and add this node as a child to that parent node
                if let Some(parent_node_id) = node.get_parent_id() {
                    if let Some(parent_node) =
                        nodes.iter().find(|x| x.get_node_id() == parent_node_id)
                    {
                        parent_node.add_child(node.clone())
                    }
                }
            }
            return Ok(Nodes(nodes));
        }
        Ok(Nodes(nodes))
    }
}

#[cfg(test)]
mod tests {
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
        root_node.add_child(child_node.clone());
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
