use thiserror::Error;

/// The error type for this crate.
#[derive(Clone, Debug, Error)]
pub enum Error {
    /// The root node is already present in the tree.
    #[error("Root node already present in the tree. You cannot add another root node.")]
    RootNodeAlreadyPresent,
    /// An invalid operation was performed on the tree.
    #[error("{0}")]
    InvalidOperation(String),
}
