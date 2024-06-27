#[cfg(not(feature = "no_std"))]
use thiserror::Error;

use crate::lib::*;

/// The error type for this crate.
#[cfg_attr(not(feature = "no_std"), derive(Error))]
#[derive(Clone, Debug)]
pub enum Error {
    /// The root node is already present in the tree.
    #[cfg_attr(
        not(feature = "no_std"),
        error("Root node already present in the tree. You cannot add another root node.")
    )]
    RootNodeAlreadyPresent,
    /// An invalid operation was performed on the tree.
    #[cfg_attr(not(feature = "no_std"), error("{0}"))]
    InvalidOperation(String),
    /// The node was not found in the tree.
    #[cfg_attr(not(feature = "no_std"), error("Node {0} not found in the tree."))]
    NodeNotFound(String),
    /// An error occurred while formatting the output.
    #[allow(clippy::enum_variant_names)]
    #[cfg_attr(not(feature = "no_std"), error("{0}"))]
    FmtError(FmtError),
}

impl From<FmtError> for Error {
    fn from(err: FmtError) -> Self {
        Error::FmtError(err)
    }
}
