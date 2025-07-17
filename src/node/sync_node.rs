use crate::lib::*;
use crate::node::_Node;

#[cfg(feature = "auto_id")]
use crate::node::GENERATOR;
#[cfg(feature = "auto_id")]
use sequential_gen::prelude::Generator;
#[cfg(feature = "serde")]
use ::serde::{Deserialize, Serialize};

/// A node in a tree.
///
/// This struct represents a node in a tree. The node has a unique id, a value, children and a parent. The unique id
/// is used to identify the node. The value is the value of the node. The children are the children of the node and
/// the parent is the parent of the node.
///
/// # Type Parameters
///
/// * `Q` - The type of the unique id of the node. Odd, I know but this is for flexibility. Some people might want to use
///   a string as the unique id of the node. Others might want to use an integer. This is why the unique id is a generic type.
/// * `T` - The type of the value of the node.
///
/// # Fields
///
/// * `node_id` - The unique id of the node.
/// * `value` - The value of the node.
/// * `children` - The children of the node.
/// * `parent` - The parent of the node.
///
/// # Example
///
/// ```rust
/// # use tree_ds::prelude::Node;
///
/// let node: Node<i32, i32> = Node::new(1, Some(2));
/// ```
#[derive(Clone, Debug, Eq)]
pub struct Node<Q, T>(Rc<RefCell<_Node<Q, T>>>)
where
    Q: PartialEq + Eq + Clone,
    T: PartialEq + Eq + Clone;

impl<Q, T> Node<Q, T>
where
    Q: PartialEq + Eq + Clone,
    T: PartialEq + Eq + Clone,
{
    /// Create a new node.
    ///
    /// This method creates a new node with the given node id and value. The node id is used to identify the node
    /// and the value is the value of the node. The value can be used to store any data that you want to associate
    /// with the node.
    ///
    /// # Arguments
    ///
    /// * `node_id` - The id of the node.
    /// * `value` - The value of the node.
    ///
    /// # Returns
    ///
    /// A new node with the given node id and value.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use tree_ds::prelude::Node;
    ///
    /// let node = Node::new(1, Some(2));
    /// ```
    pub fn new(node_id: Q, value: Option<T>) -> Self {
        Node(Rc::new(RefCell::new(_Node {
            node_id,
            value,
            children: vec![],
            parent: None,
        })))
    }

    /// Add a child to the node.
    ///
    /// This method adds a child to the node. The child is added to the children of the node and the parent
    /// of the child is set to the node.
    ///
    /// # Arguments
    ///
    /// * `child` - The child to add to the node.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use tree_ds::prelude::Node;
    ///
    /// let parent_node = Node::new(1, Some(2));
    /// parent_node.add_child(Node::new(2, Some(3))).unwrap();
    /// ```
    pub fn add_child(&self, child: Node<Q, T>) -> crate::prelude::Result<()> {
        {
            // This block is to ensure that the borrow_mut() is dropped before the next borrow_mut() call.
            let mut node = self.0.borrow_mut();
            node.children.push(child.get_node_id()?);
        }
        let mut child = child.0.borrow_mut();
        child.parent = Some(self.get_node_id()?);
        Ok(())
    }

    /// Remove a child from the node.
    ///
    /// This method removes a child from the node. The child is removed from the children of the node and the parent
    /// of the child is set to `None`.
    /// The reason as to why we pass the child as an argument instead of the node id is because we want to ensure that the
    /// parent of the child is set to `None` when the child is removed from this parent. If we were to pass the node id
    /// of the child as an argument, we would have to get the child from the tree and then set the parent to `None`. And
    /// at this level we have no knowledge of the tree.
    ///
    /// # Arguments
    ///
    /// * `child` - The child to remove from the node.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use tree_ds::prelude::Node;
    ///
    /// let parent_node = Node::new(1, Some(2));
    /// let child_node = Node::new(2, Some(3));
    /// parent_node.add_child(child_node.clone()).unwrap();
    /// parent_node.remove_child(child_node).unwrap();
    /// ```
    pub fn remove_child(&self, child: Node<Q, T>) -> crate::prelude::Result<()> {
        let mut node = self.0.borrow_mut();
        node.children.retain(|x| {
            x != &child
                .get_node_id()
                .expect("Error: Could not fetch id of Node.")
        });
        let mut child = child.0.borrow_mut();
        child.parent = None;
        Ok(())
    }

    /// Get the unique Id of the node.
    ///
    /// This method returns the unique Id of the node. The unique Id is used to identify the node.
    ///
    /// # Returns
    ///
    /// The unique Id of the node.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use tree_ds::prelude::Node;
    ///
    /// let node = Node::new(1, Some(2));
    /// assert_eq!(node.get_node_id().unwrap(), 1);
    /// ```
    pub fn get_node_id(&self) -> crate::prelude::Result<Q> {
        Ok(self.0.borrow().node_id.clone())
    }

    /// Get the ids of the children of the node.
    ///
    /// This method returns the ids of the children of the node.
    ///
    /// # Returns
    ///
    /// The ids of the children of the node.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use tree_ds::prelude::Node;
    ///
    /// let node = Node::new(1, Some(2));
    /// let child = Node::new(2, Some(3));
    /// node.add_child(child).unwrap();
    ///
    /// // In this case Getting the children ids should not
    /// // panic as we are using the sync version of the node.
    ///
    /// // |>Note that the async version of the node might panic
    /// // due to poisoning so handle accordingly.
    /// assert_eq!(node.get_children_ids().unwrap().len(), 1);
    /// ```
    pub fn get_children_ids(&self) -> crate::prelude::Result<Vec<Q>> {
        Ok(self.0.borrow().children.clone())
    }

    /// Get the node id of the parent of the node.
    ///
    /// This method returns the node_id of the parent of the node. In the case where the node is a root node in a tree,
    /// the parent of the node will be `None`.
    ///
    /// # Returns
    ///
    /// The optional node_id of the parent of the node.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use tree_ds::prelude::Node;
    ///
    /// let parent_node = Node::new(1, Some(2));
    /// let child_node = Node::new(2, Some(3));
    /// parent_node.add_child(child_node.clone()).unwrap();
    /// assert_eq!(child_node.get_parent_id().unwrap().as_ref(), Some(&parent_node.get_node_id().unwrap()));
    /// assert!(parent_node.get_parent_id().unwrap().is_none());
    /// ```
    pub fn get_parent_id(&self) -> crate::prelude::Result<Option<Q>> {
        Ok(self.0.borrow().parent.clone())
    }

    /// Get the value of the node.
    ///
    /// This method returns the value of the node. If the node has no value, `None` is returned.
    ///
    /// # Returns
    ///
    /// The value of the node.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use tree_ds::prelude::Node;
    ///
    /// let node = Node::new(1, Some(2));
    /// assert_eq!(node.get_value().unwrap(), Some(2));
    /// ```
    pub fn get_value(&self) -> crate::prelude::Result<Option<T>> {
        Ok(self.0.borrow().value.clone())
    }

    /// Set the value of the node.
    ///
    /// This method sets the value of the node.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to set.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use tree_ds::prelude::Node;
    ///
    /// let node = Node::new(1, Some(2));
    /// assert_eq!(node.get_value().unwrap(), Some(2));
    /// node.set_value(Some(3)).unwrap();
    /// assert_eq!(node.get_value().unwrap(), Some(3));
    /// ```
    pub fn set_value(&self, value: Option<T>) -> crate::prelude::Result<()> {
        self.0.borrow_mut().value = value;
        Ok(())
    }

    /// Update the value of the node.
    ///
    /// This method updates the value of the node using a closure.
    ///
    /// # Arguments
    ///
    /// * `modifier` - The closure to use for updating the value.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use tree_ds::prelude::Node;
    ///
    /// let node = Node::new(1, Some(2));
    /// node.update_value(|value| *value = Some(3)).unwrap();
    /// assert_eq!(node.get_value().unwrap(), Some(3));
    /// ```
    pub fn update_value(
        &self,
        modifier: impl FnOnce(&mut Option<T>),
    ) -> crate::prelude::Result<()> {
        let mut node = self.0.borrow_mut();
        modifier(&mut node.value);
        Ok(())
    }

    /// Set the parent of the node.
    ///
    /// This method sets the parent of the node.
    ///
    /// # Arguments
    ///
    /// * `parent` - The parent to set.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use tree_ds::prelude::Node;
    ///
    /// let parent_node = Node::new(1, Some(2));
    /// let child_node = Node::new(2, Some(3));
    /// child_node.set_parent(Some(parent_node.clone())).unwrap();
    /// assert_eq!(child_node.get_parent_id().unwrap().as_ref(), Some(&parent_node.get_node_id().unwrap()));
    /// ```
    pub fn set_parent(&self, parent: Option<Node<Q, T>>) -> crate::prelude::Result<()> {
        if let Some(parent) = parent.as_ref() {
            parent.add_child(self.clone())?;
        }
        self.0.borrow_mut().parent =
            parent.map(|x| x.get_node_id().expect("Error: Could not fetch id of Node."));
        Ok(())
    }

    /// Sort the children of the node by an ordering closure.
    ///
    /// # Arguments
    ///
    /// * `compare` - The closure to use for comparison.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use tree_ds::prelude::Node;
    ///
    /// let parent_node = Node::new(1, Some(2));
    /// let child1 = Node::new(2, Some(3));
    /// let child2 = Node::new(3, Some(4));
    /// parent_node.add_child(child1).unwrap();
    /// parent_node.add_child(child2).unwrap();
    ///
    /// parent_node.sort_children(|a, b| a.cmp(&b).reverse()).unwrap();
    /// assert_eq!(parent_node.get_children_ids().unwrap(), vec![3, 2]);
    /// ```
    pub fn sort_children(&self, compare: impl Fn(&Q, &Q) -> Ordering) -> crate::prelude::Result<()>
    where
        Q: Debug,
    {
        let mut children = self.0.borrow().children.clone();
        children.sort_by(|a, b| compare(a, b));
        self.update_children(children)?;
        Ok(())
    }

    fn update_children(&self, update: impl AsRef<[Q]>) -> crate::prelude::Result<()> {
        let children = &mut self.0.borrow_mut().children;
        children.clear();
        children.extend_from_slice(update.as_ref());
        Ok(())
    }
}

impl<Q, T> PartialEq for Node<Q, T>
where
    Q: PartialEq + Eq + Clone,
    T: PartialEq + Eq + Clone,
{
    /// Compare two nodes for equality.
    fn eq(&self, other: &Self) -> bool {
        self.get_node_id() == other.get_node_id() && self.get_value() == other.get_value()
    }
}

impl<Q, T> Display for Node<Q, T>
where
    Q: PartialEq + Eq + Clone + Display,
    T: PartialEq + Eq + Clone + Display + Default,
{
    /// Display the node.
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        #[cfg(feature = "print_node_id")]
        return write!(
            f,
            "{}: {}",
            self.get_node_id()
                .expect("Error: Could not fetch id of Node."),
            self.get_value()
                .expect("Error: Could not fetch value of Node.")
                .as_ref()
                .cloned()
                .unwrap_or_default()
        );
        #[cfg(not(feature = "print_node_id"))]
        write!(
            f,
            "{}",
            self.get_value()
                .expect("Error: Could not fetch value of Node.")
                .as_ref()
                .cloned()
                .unwrap_or_default()
        )
    }
}

impl<Q, T> Hash for Node<Q, T>
where
    Q: PartialEq + Eq + Clone + Hash,
    T: PartialEq + Eq + Clone + Hash,
{
    /// Hash the node.
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.get_node_id()
            .expect("Error: Could not fetch id of Node.")
            .hash(state);
        self.get_value()
            .expect("Error: Could not fetch value of Node.")
            .hash(state);
        self.get_children_ids()
            .expect("Error: Could not fetch the children ids of  the Node.")
            .hash(state);
        self.get_parent_id()
            .expect("Error: Could not fetch the parent id of the Node.")
            .hash(state);
    }
}

/// An iterator over the nodes in a tree.
///
/// This struct represents an iterator over the nodes in a tree. The iterator is created by calling the `iter` method
/// on the tree. The iterator yields the nodes in the tree in the order that they were added to the tree.
///
/// # Type Parameters
///
/// * `Q` - The type of the unique id of the node.
/// * `T` - The type of the value of the node.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Nodes<Q, T>
where
    Q: PartialEq + Eq + Clone,
    T: PartialEq + Eq + Clone,
{
    nodes: Vec<Node<Q, T>>,
    index: usize,
}

impl<Q, T> Nodes<Q, T>
where
    Q: PartialEq + Eq + Clone,
    T: PartialEq + Eq + Clone,
{
    /// Create a new iterator over the nodes in a tree.
    ///
    /// This method creates a new iterator over the nodes in a tree.
    ///
    /// # Arguments
    ///
    /// * `nodes` - The nodes in the tree.
    ///
    /// # Returns
    ///
    /// A new iterator over the nodes in a tree.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use tree_ds::prelude::{Node, Nodes};
    ///
    /// let nodes = Nodes::new(vec![Node::new(1, Some(2))]);
    /// ```
    pub fn new(nodes: Vec<Node<Q, T>>) -> Self {
        Nodes { nodes, index: 0 }
    }

    /// Get an iterator over the nodes in the tree.
    ///
    /// This method returns an iterator over the nodes in the tree.
    ///
    /// # Returns
    ///
    /// An iterator over the nodes in the tree.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use tree_ds::prelude::{Node, Nodes};
    ///
    /// let nodes = Nodes::new(vec![Node::new(1, Some(2))]);
    ///
    /// for node in nodes.iter() {
    ///     // Do something with the node.
    /// }
    /// ```
    pub fn iter(&self) -> Iter<Node<Q, T>> {
        self.nodes.iter()
    }

    /// Get the number of nodes in the tree.
    ///
    /// This method returns the number of nodes in the tree.
    ///
    /// # Returns
    ///
    /// The number of nodes in the tree.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use tree_ds::prelude::{Node, Nodes};
    ///
    /// let nodes = Nodes::new(vec![Node::new(1, Some(2))]);
    /// assert_eq!(nodes.len(), 1);
    /// ```
    pub fn len(&self) -> usize {
        self.nodes.len()
    }

    /// Check if the tree is empty.
    ///
    /// This method checks if the tree is empty.
    ///
    /// # Returns
    ///
    /// `true` if the tree is empty, `false` otherwise.
    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }

    /// Get a node at the specified index.
    ///
    /// This method returns a node at the specified index.
    ///
    /// # Arguments
    ///
    /// * `index` - The index of the node to get.
    ///
    /// # Returns
    ///
    /// The node at the specified index.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use tree_ds::prelude::{Node, Nodes};
    ///
    /// let nodes = Nodes::new(vec![Node::new(1, Some(2))]);
    /// assert_eq!(nodes.get(0).unwrap().get_node_id().unwrap(), 1);
    /// ```
    pub fn get(&self, index: usize) -> Option<&Node<Q, T>> {
        self.nodes.get(index)
    }

    /// Get a node by the node id.
    ///
    /// This method returns a node by the node id.
    ///
    /// # Arguments
    ///
    /// * `node_id` - The node id of the node to get.
    ///
    /// # Returns
    ///
    /// The node with the specified node id.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use tree_ds::prelude::{Node, Nodes};
    ///
    /// let nodes = Nodes::new(vec![Node::new(1, Some(2))]);
    /// assert_eq!(nodes.get_by_node_id(&1).unwrap().get_node_id().unwrap(), 1);
    /// ```
    pub fn get_by_node_id(&self, node_id: &Q) -> Option<&Node<Q, T>> {
        self.nodes.iter().find(|x| {
            &x.get_node_id()
                .expect("Error: Could not get teh id of the node")
                == node_id
        })
    }

    /// Push a node to the nodes list.
    ///
    /// This method pushes a node to the nodes list.
    ///
    /// # Arguments
    ///
    /// * `node` - The node to push.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use tree_ds::prelude::{Node, Nodes};
    ///
    /// let mut nodes = Nodes::new(vec![Node::new(1, Some(2))]);
    /// nodes.push(Node::new(2, Some(3)));
    /// assert_eq!(nodes.len(), 2);
    /// ```
    pub fn push(&mut self, node: Node<Q, T>) {
        self.nodes.push(node);
    }

    /// Remove a node at the specified index.
    ///
    /// This method removes a node at the specified index.
    ///
    /// # Arguments
    ///
    /// * `index` - The index of the node to remove.
    ///
    /// # Returns
    ///
    /// The removed node.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use tree_ds::prelude::{Node, Nodes};
    ///
    /// let mut nodes = Nodes::new(vec![Node::new(1, Some(2))]);
    /// let removed_node = nodes.remove(0);
    /// assert_eq!(removed_node.get_node_id().unwrap(), 1);
    /// assert_eq!(nodes.len(), 0);
    /// ```
    pub fn remove(&mut self, index: usize) -> Node<Q, T> {
        self.nodes.remove(index)
    }

    /// Retain only the nodes that satisfy the predicate.
    ///
    /// This method retains only the nodes that satisfy the predicate.
    /// The predicate is a function that takes a node and returns a boolean value.
    /// If the predicate returns `true`, the node is retained. If the predicate returns `false`, the node is removed.
    /// The nodes are retained in the order that they were added to the tree.
    ///
    /// # Arguments
    ///
    /// * `f` - The predicate function.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use tree_ds::prelude::{Node, Nodes};
    ///
    /// let mut nodes = Nodes::new(vec![Node::new(1, Some(2)), Node::new(2, Some(3))]);
    /// nodes.retain(|node| node.get_node_id().expect("Error: Could not fetch the node id.") == 1);
    /// assert_eq!(nodes.len(), 1);
    /// ```
    pub fn retain<F>(&mut self, f: F)
    where
        F: FnMut(&Node<Q, T>) -> bool,
    {
        self.nodes.retain(f);
    }

    /// Clear the nodes list.
    ///
    /// This method clears the nodes list.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use tree_ds::prelude::{Node, Nodes};
    ///
    /// let mut nodes = Nodes::new(vec![Node::new(1, Some(2)), Node::new(2, Some(3))]);
    /// nodes.clear();
    /// assert_eq!(nodes.len(), 0);
    /// ```
    pub fn clear(&mut self) {
        self.nodes.clear();
    }

    /// Append the nodes from another nodes list.
    ///
    /// This method appends the nodes from another nodes list.
    ///
    /// # Arguments
    ///
    /// * `other` - The other nodes list.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use tree_ds::prelude::{Node, Nodes};
    ///
    /// let mut nodes = Nodes::new(vec![Node::new(1, Some(2))]);
    /// let mut other_nodes = Nodes::new(vec![Node::new(2, Some(3))]);
    /// nodes.append(&mut other_nodes);
    /// assert_eq!(nodes.len(), 2);
    /// ```
    pub fn append(&mut self, other: &mut Self) {
        self.nodes.append(&mut other.nodes);
    }

    /// Append the nodes from another nodes list.
    ///
    /// This method appends the nodes from another nodes list. This method is useful when you want
    /// to append the nodes as a raw vector.
    ///
    /// # Arguments
    ///
    /// * `other` - The other nodes list.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use tree_ds::prelude::{Node, Nodes};
    ///
    /// let mut nodes = Nodes::new(vec![Node::new(1, Some(2))]);
    /// let mut other_nodes = vec![Node::new(2, Some(3))];
    /// nodes.append_raw(&mut other_nodes);
    /// assert_eq!(nodes.len(), 2);
    /// ```
    pub fn append_raw(&mut self, other: &mut Vec<Node<Q, T>>) {
        self.nodes.append(other);
    }

    /// Get the first node in the nodes list.
    ///
    /// This method returns the first node in the nodes list.
    ///
    /// # Returns
    ///
    /// The first node in the nodes list.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use tree_ds::prelude::{Node, Nodes};
    ///
    /// let nodes = Nodes::new(vec![Node::new(1, Some(2)), Node::new(2, Some(3))]);
    /// assert_eq!(nodes.first().unwrap().get_node_id().unwrap(), 1);
    /// ```
    pub fn first(&self) -> Option<&Node<Q, T>> {
        self.nodes.first()
    }
}

impl<Q, T> AsRef<Nodes<Q, T>> for Nodes<Q, T>
where
    Q: PartialEq + Eq + Clone,
    T: PartialEq + Eq + Clone,
{
    /// Get a reference to the nodes list.
    fn as_ref(&self) -> &Nodes<Q, T> {
        self
    }
}

impl<Q, T> FromIterator<Node<Q, T>> for Nodes<Q, T>
where
    Q: PartialEq + Eq + Clone,
    T: PartialEq + Eq + Clone,
{
    /// Create a nodes list from an iterator.
    fn from_iter<I: IntoIterator<Item = Node<Q, T>>>(iter: I) -> Self {
        Nodes::new(iter.into_iter().collect())
    }
}

impl<Q, T> Iterator for Nodes<Q, T>
where
    Q: PartialEq + Eq + Clone,
    T: PartialEq + Eq + Clone,
{
    type Item = Node<Q, T>;

    /// Get the next node in the nodes list.
    #[allow(clippy::iter_next_slice)]
    fn next(&mut self) -> Option<Self::Item> {
        let node = self.nodes.get(self.index).cloned();
        self.index += 1;
        node
    }

    /// Get the size hint of the nodes list.
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.nodes.iter().size_hint()
    }
}

impl<Q, T> Default for Nodes<Q, T>
where
    Q: PartialEq + Eq + Clone,
    T: PartialEq + Eq + Clone,
{
    /// Create an empty nodes list.
    fn default() -> Self {
        Nodes::new(vec![])
    }
}

impl<Q, T> Display for Nodes<Q, T>
where
    Q: PartialEq + Eq + Clone + Display,
    T: PartialEq + Eq + Clone + Display + Default,
{
    /// Display the nodes list.
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        for node in self.iter() {
            write!(f, "{node}")?;
        }
        Ok(())
    }
}

#[cfg(feature = "serde")]
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

#[cfg(feature = "serde")]
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

        Ok(Node(Rc::new(RefCell::new(node))))
    }
}

#[cfg(feature = "serde")]
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
        self.nodes.serialize(serializer)
    }
}

#[cfg(feature = "serde")]
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
                if let Some(parent_node_id) = node
                    .get_parent_id()
                    .expect("Error: Could not fetch parent id of Node.")
                {
                    if let Some(parent_node) = nodes.iter().find(|x| {
                        x.get_node_id()
                            .expect("Error: Could not fetch the node id.")
                            == parent_node_id
                    }) {
                        parent_node
                            .add_child(node.clone())
                            .expect("Error: Could not add child to Node.");
                    }
                }
            }
            return Ok(Nodes::new(nodes));
        }
        Ok(Nodes::new(nodes))
    }
}

#[cfg(feature = "auto_id")]
impl<Q, T> Node<Q, T>
where
    Q: PartialEq + Eq + Clone + From<u128>,
    T: PartialEq + Eq + Clone,
{
    /// Creates a new node with an auto-generated ID.
    ///
    /// The ID is generated using a sequence generator, meaning that the ID is sequential and unique.
    /// This is useful when you want to create a node without specifying the ID. For a node to be
    /// created with an auto-generated ID, the `Q` type must be of type `AutomatedId`.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to store in the node.
    ///
    /// # Returns
    ///
    /// A new node with an auto-generated ID.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use tree_ds::prelude::*;
    ///
    /// # #[cfg(feature = "auto_id")]
    /// # {
    /// let node = Node::<AutomatedId, &str>::new_with_auto_id(Some("Harry Doe"));
    /// let node_2 = Node::<AutomatedId, &str>::new_with_auto_id(Some("Jane Doe"));
    /// assert_ne!(node.get_node_id().unwrap(), node_2.get_node_id().unwrap());
    /// # }
    /// ```
    ///
    /// This is available only when the `auto_id` feature is enabled.
    pub fn new_with_auto_id(value: Option<T>) -> Self {
        Self(Rc::new(RefCell::new(_Node {
            node_id: Q::from(GENERATOR.generate()),
            value,
            children: vec![],
            parent: None,
        })))
    }
}
