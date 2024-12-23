use core::cmp::Ordering;

#[cfg(feature = "async")]
use crate::lib::Arc;
#[cfg(not(feature = "async"))]
use crate::lib::Rc;
use crate::lib::*;

#[cfg(feature = "auto_id")]
mod auto_id;
#[cfg(feature = "serde")]
mod serde;

/// A node in a tree.
///
/// This struct represents a node in a tree. The node has a unique id, a value, children and a parent. The unique id
/// is used to identify the node. The value is the value of the node. The children are the children of the node and
/// the parent is the parent of the node.
///
/// # Type Parameters
///
/// * `Q` - The type of the unique id of the node. Odd, I know but this is for flexibility. Some people might want to use
/// a string as the unique id of the node. Others might want to use an integer. This is why the unique id is a generic type.
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
#[cfg(not(feature = "async"))]
#[derive(Clone, Debug, Eq)]
pub struct Node<Q, T>(Rc<RefCell<_Node<Q, T>>>)
where
    Q: PartialEq + Eq + Clone,
    T: PartialEq + Eq + Clone;

/// A node in a tree.
///
/// This struct represents a node in a tree. The node has a unique id, a value, children and a parent. The unique id
/// is used to identify the node. The value is the value of the node. The children are the children of the node and
/// the parent is the parent of the node.
///
/// # Type Parameters
///
/// * `Q` - The type of the unique id of the node. Odd, I know but this is for flexibility. Some people might want to use a string as the unique id of the node. Others might want to use an integer. This is why the unique id is a generic type.
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
#[cfg(feature = "async")]
#[derive(Clone, Debug, Eq)]
pub struct Node<Q, T>(Arc<RefCell<_Node<Q, T>>>)
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
        #[cfg(not(feature = "async"))]
        {
            Node(Rc::new(RefCell::new(_Node {
                node_id,
                value,
                children: vec![],
                parent: None,
            })))
        }
        #[cfg(feature = "async")]
        {
            Node(Arc::new(RefCell::new(_Node {
                node_id,
                value,
                children: vec![],
                parent: None,
            })))
        }
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
    /// parent_node.add_child(Node::new(2, Some(3)));
    /// ```
    pub fn add_child(&self, child: Node<Q, T>) {
        {
            // This block is to ensure that the borrow_mut() is dropped before the next borrow_mut() call.
            let mut node = self.0.borrow_mut();
            node.children.push(child.get_node_id());
        }
        let mut child = child.0.borrow_mut();
        child.parent = Some(self.get_node_id());
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
    /// parent_node.add_child(child_node.clone());
    /// parent_node.remove_child(child_node);
    /// ```
    pub fn remove_child(&self, child: Node<Q, T>) {
        let mut node = self.0.borrow_mut();
        node.children.retain(|x| x != &child.get_node_id());
        let mut child = child.0.borrow_mut();
        child.parent = None;
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
    /// assert_eq!(node.get_node_id(), 1);
    /// ```
    pub fn get_node_id(&self) -> Q {
        self.0.borrow().node_id.clone()
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
    /// node.add_child(child);
    /// assert_eq!(node.get_children_ids().len(), 1);
    /// ```
    pub fn get_children_ids(&self) -> Vec<Q> {
        self.0.borrow().children.clone()
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
    /// parent_node.add_child(child_node.clone());
    /// assert_eq!(child_node.get_parent_id().as_ref(), Some(&parent_node.get_node_id()));
    /// assert!(parent_node.get_parent_id().is_none());
    /// ```
    pub fn get_parent_id(&self) -> Option<Q> {
        self.0.borrow().parent.clone()
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
    /// assert_eq!(node.get_value(), Some(2));
    /// ```
    pub fn get_value(&self) -> Option<T> {
        self.0.borrow().value.clone()
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
    /// assert_eq!(node.get_value(), Some(2));
    /// node.set_value(Some(3));
    /// assert_eq!(node.get_value(), Some(3));
    /// ```
    pub fn set_value(&self, value: Option<T>) {
        self.0.borrow_mut().value = value;
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
    /// node.update_value(|value| *value = Some(3));
    /// assert_eq!(node.get_value(), Some(3));
    /// ```
    pub fn update_value(&self, modifier: impl FnOnce(&mut Option<T>)) {
        let mut node = self.0.borrow_mut();
        modifier(&mut node.value);
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
    /// child_node.set_parent(Some(parent_node.clone()));
    /// assert_eq!(child_node.get_parent_id().as_ref(), Some(&parent_node.get_node_id()));
    /// ```
    pub fn set_parent(&self, parent: Option<Node<Q, T>>) {
        if let Some(parent) = parent.as_ref() {
            parent.add_child(self.clone());
        }
        self.0.borrow_mut().parent = parent.map(|x| x.get_node_id());
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
    /// parent_node.add_child(child1);
    /// parent_node.add_child(child2);
    ///
    /// parent_node.sort_children(|a, b| a.cmp(&b).reverse());
    /// assert_eq!(parent_node.get_children_ids(), vec![3, 2]);
    /// ```
    pub fn sort_children(&self, compare: impl Fn(&Q, &Q) -> Ordering)
    where
        Q: Debug,
    {
        let mut children = self.0.borrow().children.clone();
        children.sort_by(|a, b| compare(a, b));
        self.update_children(children);
    }

    fn update_children(&self, update: impl AsRef<[Q]>) {
        let children = &mut self.0.borrow_mut().children;
        children.clear();
        children.extend_from_slice(update.as_ref());
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
            self.get_node_id(),
            self.get_value().as_ref().cloned().unwrap_or_default()
        );
        #[cfg(not(feature = "print_node_id"))]
        return write!(
            f,
            "{}",
            self.get_value().as_ref().cloned().unwrap_or_default()
        );
    }
}

impl<Q, T> Hash for Node<Q, T>
where
    Q: PartialEq + Eq + Clone + Hash,
    T: PartialEq + Eq + Clone + Hash,
{
    /// Hash the node.
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.get_node_id().hash(state);
        self.get_value().hash(state);
        self.get_children_ids().hash(state);
        self.get_parent_id().hash(state);
    }
}

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
    T: PartialEq + Eq + Clone {
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
        Nodes{
            nodes,
            index: 0
        }
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
    /// assert_eq!(nodes.get(0).unwrap().get_node_id(), 1);
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
    /// assert_eq!(nodes.get_by_node_id(&1).unwrap().get_node_id(), 1);
    /// ```
    pub fn get_by_node_id(&self, node_id: &Q) -> Option<&Node<Q, T>> {
        self.nodes.iter().find(|x| &x.get_node_id() == node_id)
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
    /// assert_eq!(removed_node.get_node_id(), 1);
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
    /// nodes.retain(|node| node.get_node_id() == 1);
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
    /// assert_eq!(nodes.first().unwrap().get_node_id(), 1);
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
            write!(f, "{}", node)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::lib::*;

    use super::*;

    #[test]
    fn test_node_new() {
        let node = Node::new(1, Some(2));
        assert_eq!(node.get_node_id(), 1);
        assert_eq!(node.get_value(), Some(2));
        assert_eq!(node.get_children_ids().len(), 0);
        assert!(node.get_parent_id().is_none());
    }

    #[test]
    fn test_node_adding_children() {
        let node = Node::new(1, Some(2));
        let child = Node::new(2, Some(3));
        node.add_child(child);
        assert_eq!(node.get_children_ids().len(), 1);
    }

    #[test]
    fn test_node_get_node_id() {
        let node = Node::new(1, Some(2));
        assert_eq!(node.get_node_id(), 1);
    }

    #[test]
    fn test_node_get_parent() {
        let parent_node = Node::new(1, Some(2));
        let child_node = Node::new(2, Some(3));
        parent_node.add_child(child_node.clone());
        assert_eq!(
            child_node.get_parent_id().as_ref(),
            Some(&parent_node.get_node_id())
        );
        assert!(parent_node.get_parent_id().is_none());
    }

    #[test]
    fn test_node_get_value() {
        let node = Node::new(1, Some(2));
        assert_eq!(node.get_value(), Some(2));
    }

    #[test]
    fn test_node_set_value() {
        let node = Node::new(1, Some(2));
        assert_eq!(node.get_value(), Some(2));
        node.set_value(Some(3));
        assert_eq!(node.get_value(), Some(3));
    }

    #[test]
    fn test_node_set_parent() {
        let parent_node = Node::new(1, Some(2));
        let child_node = Node::new(2, Some(3));
        child_node.set_parent(Some(parent_node.clone()));
        assert_eq!(
            child_node.get_parent_id().as_ref(),
            Some(&parent_node.get_node_id())
        );
    }

    #[test]
    fn test_node_remove_child() {
        let parent_node = Node::new(1, Some(2));
        let child_node = Node::new(2, Some(3));
        parent_node.add_child(child_node.clone());
        parent_node.remove_child(child_node);
        assert_eq!(parent_node.get_children_ids().len(), 0);
    }

    #[test]
    fn test_node_update_value() {
        let node = Node::new(1, Some(2));
        node.update_value(|value| *value = value.map(|x| x + 1));
        assert_eq!(node.get_value(), Some(3));
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
        assert_eq!(format!("{}", node), "1: 2");
    }

    #[test]
    #[cfg_attr(feature = "print_node_id", ignore)]
    fn test_node_display_without_id() {
        let node = Node::new(1, Some(2));
        assert_eq!(format!("{}", node), "2");
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
    fn test_nodes_get() {
        let nodes = Nodes::new(vec![Node::new(1, Some(2))]);
        assert_eq!(nodes.get(0).unwrap().get_node_id(), 1);
    }

    #[test]
    fn test_nodes_get_by_node_id() {
        let nodes = Nodes::new(vec![Node::new(1, Some(2))]);
        assert_eq!(nodes.get_by_node_id(&1).unwrap().get_node_id(), 1);
    }

    #[test]
    fn test_nodes_push() {
        let mut nodes = Nodes::new(vec![Node::new(1, Some(2))]);
        nodes.push(Node::new(2, Some(3)));
        assert_eq!(nodes.len(), 2);
    }

    #[test]
    fn test_nodes_remove() {
        let mut nodes = Nodes::new(vec![Node::new(1, Some(2))]);
        let removed_node = nodes.remove(0);
        assert_eq!(removed_node.get_node_id(), 1);
        assert_eq!(nodes.len(), 0);
    }

    #[test]
    fn test_nodes_retain() {
        let mut nodes = Nodes::new(vec![Node::new(1, Some(2)), Node::new(2, Some(3))]);
        nodes.retain(|node| node.get_node_id() == 1);
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
    fn test_nodes_first() {
        let nodes = Nodes::new(vec![Node::new(1, Some(2)), Node::new(2, Some(3))]);
        assert_eq!(nodes.first().unwrap().get_node_id(), 1);
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
    fn test_nodes_iterator() {
        let nodes = Nodes::new(vec![Node::new(1, Some(2)), Node::new(2, Some(3))]);
        let mut iter = nodes.iter();
        assert_eq!(iter.next().unwrap().get_node_id(), 1);
        assert_eq!(iter.next().unwrap().get_node_id(), 2);
    }
    
    #[test]
    fn test_nodes_next() {
        let mut nodes = Nodes::new(vec![Node::new(1, Some(2)), Node::new(2, Some(3))]);
        assert_eq!(nodes.next().unwrap().get_node_id(), 1);
        assert_eq!(nodes.next().unwrap().get_node_id(), 2);
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
        assert_eq!(format!("{}", nodes), "1: 2");
        #[cfg(not(feature = "print_node_id"))]
        assert_eq!(format!("{}", nodes), "2");
    }
}
