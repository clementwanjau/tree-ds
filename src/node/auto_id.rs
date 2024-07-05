#[cfg(feature = "no_std")]
use lazy_static::lazy_static;
#[cfg(not(feature = "no_std"))]
use sequential_gen::prelude::EpochBasedGenerator;
use sequential_gen::prelude::Generator;
#[cfg(feature = "no_std")]
use sequential_gen::prelude::SimpleGenerator;

use crate::lib::*;
#[cfg(feature = "async")]
use crate::lib::Arc;
#[cfg(not(feature = "async"))]
use crate::lib::Rc;
use crate::node::{_Node, Node};

#[cfg(feature = "no_std")]
lazy_static! {
    static ref GENERATOR: SimpleGenerator<usize> = SimpleGenerator::new(1usize);
}

#[cfg(not(feature = "no_std"))]
pub const GENERATOR: EpochBasedGenerator = EpochBasedGenerator;

impl<Q, T> Node<Q, T>
where
    Q: PartialEq + Eq + Clone + From<u128>,
    T: PartialEq + Eq + Clone,
{
    /// Creates a new node with an auto-generated ID.
    ///
    /// The ID is generated using a sequence generator, meaning that the ID is sequential and unique.
    /// This is useful when you want to create a node without specifying the ID. For a node to be
    /// created with an auto-generated ID, the `Q` type must implement the `From<i32>` trait.
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
    /// let node = Node::<AutomatedId, &str>::new_with_auto_id(Some("Harry Doe"));
    /// let node_2 = Node::<AutomatedId, &str>::new_with_auto_id(Some("Jane Doe"));
    /// assert_ne!(node.get_node_id(), node_2.get_node_id());
    /// ```
    ///
    /// This is available only when the `auto_id` feature is enabled.
    pub fn new_with_auto_id(value: Option<T>) -> Self {
        #[cfg(not(feature = "async"))]
        {
            Self(Rc::new(RefCell::new(_Node {
                node_id: Q::from(GENERATOR.generate() as u128),
                value,
                children: vec![],
                parent: None,
            })))
        }
        #[cfg(feature = "async")]
        {
            Self(Arc::new(RefCell::new(_Node {
                node_id: Q::from(GENERATOR.generate()),
                value,
                children: vec![],
                parent: None,
            })))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::AutomatedId;

    use super::*;

    #[cfg(not(feature = "async"))]
    #[test]
    fn test_new_with_auto_id() {
        let node = Node::<AutomatedId, &str>::new_with_auto_id(Some("Harry Doe"));
        let node_2 = Node::<AutomatedId, &str>::new_with_auto_id(Some("Jane Doe"));
        assert_eq!(node.get_value(), Some("Harry Doe"));
        assert_ne!(node.get_node_id(), node_2.get_node_id());
    }

    #[cfg(feature = "async")]
    #[test]
    fn test_new_with_auto_id_async() {
        let node = Node::<AutomatedId, &str>::new_with_auto_id(Some("Harry Doe"));
        let node_2 = Node::<AutomatedId, &str>::new_with_auto_id(Some("Jane Doe"));
        assert_eq!(node.get_value(), Some("Harry Doe"));
        assert_ne!(node.get_node_id(), node_2.get_node_id());
    }
}
