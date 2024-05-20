use thiserror::Error;

#[derive(Clone, Debug, Error)]
pub enum Error {
	#[error("Root node already present in the tree. You cannot add another root node.")]
	RootNodeAlreadyPresent
}
