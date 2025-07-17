#[cfg(not(feature = "no_std"))]
use thiserror::Error;

use crate::lib::*;

/// The error type for this crate.
#[cfg_attr(not(feature = "no_std"), derive(Error))]
#[derive(Clone, PartialEq)]
pub enum Error {
    /// The root node is already present in the tree.
    #[cfg_attr(
        not(feature = "no_std"),
        error("Error: Root node already present in the tree. You cannot add another root node.")
    )]
    RootNodeAlreadyPresent,
    /// An invalid operation was performed on the tree.
    #[cfg_attr(not(feature = "no_std"), error("Error: {0}"))]
    InvalidOperation(String),
    /// The node was not found in the tree.
    #[cfg_attr(
        not(feature = "no_std"),
        error("Error: Node {0} not found in the tree.")
    )]
    NodeNotFound(String),
    /// An error occurred while formatting the output.
    #[allow(clippy::enum_variant_names)]
    #[cfg_attr(not(feature = "no_std"), error(transparent))]
    FmtError(FmtError),
}

#[cfg(feature = "no_std")]
impl Display for Error {
    /// Formats the error message.
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Error::RootNodeAlreadyPresent => write!(
                f,
                "Error: Root node already present in the tree. You cannot add another root node."
            ),
            Error::InvalidOperation(s) => write!(f, "Error: {s}"),
            Error::NodeNotFound(s) => write!(f, "Error: Node {s} not found in the tree."),
            Error::FmtError(_) => {
                write!(f, "Error: An error occurred while formatting the output.")
            }
        }
    }
}

impl Debug for Error {
    /// Formats the error message.
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{self}")
    }
}

impl From<FmtError> for Error {
    fn from(err: FmtError) -> Self {
        Error::FmtError(err)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_fmt() {
        let err = Error::InvalidOperation("Invalid operation".to_string());
        assert_eq!(format!("{err:?}"), "Error: Invalid operation");
    }

    #[test]
    fn test_error_fmt_root_node_already_present() {
        let err = Error::RootNodeAlreadyPresent;
        assert_eq!(
            format!("{err:?}"),
            "Error: Root node already present in the tree. You cannot add another root node."
        );
    }

    #[test]
    fn test_error_from_fmt_error() {
        let err = Error::FmtError(FmtError);
        assert_eq!(Error::from(FmtError), err);
    }
}
