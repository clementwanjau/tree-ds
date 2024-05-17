mod tree;
mod node;

pub mod prelude {
	pub use crate::{
		node::Node,
		tree::{NodeRemovalStrategy, SubTree, Tree},
	};
}
