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
    /// The node was not found in the tree.
    #[error("Node {0} not found in the tree.")]
    NodeNotFound(String),
    /// An error occurred while formatting the output.
    #[allow(clippy::enum_variant_names)]
    #[error("{0}")]
    FmtError(#[from] std::fmt::Error),
}
