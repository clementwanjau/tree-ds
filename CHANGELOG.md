# Changelog

## v0.1.5

- Changed the signature of the `Tree::traverse(&self, order: TraversalStrategy, node_id: &Q) -> Result<Vec<Q>>` method
  to `Tree::traverse(&self, node_id: &Q, order: TraversalStrategy) -> Result<Vec<Q>>` to conform to the project method
  signature convention.
- Added more bug fixes and tests.
- The auto id feature now depends on epoch based id generation to generate unique ids for nodes.
- Added a `print_node_id` feature to enable printing of node ids in the tree visualization. It is disabled by default.

## v0.1.4

- Bug fixes.
- Improved api ergonomics by renaming functions to be more concise. The affected methods are:
    - `Node::get_parent` -> `Node::get_parent_id`
    - `Node::get_children` -> `Node::get_children_ids`
- Clear the unwrapping landmines by returning `Result` instead of unwrapping internally which could lead to panicking in
  some cases.
- Added support for `no_std` environments.
- Changed the `unique_id` dependency to `sequential_gen` to generate unique ids for nodes.
- Add a way to get the ids of the ancestors of a node with the `Tree::get_ancestor_ids` method. Thanks to
  [@PenguinWithATie](https://github.com/PenguinWithATie) for the
  contribution.

## v0.1.3

- Bug fixes.
- Improved documentation.
- Added automated node id generation behind the `auto_id` feature flag. This feature is disabled by default.

## V0.1.2

- Added support for tree and node hashing.
- Added feature to traverse the tree in any order.
- Added support for naming trees. The name is `optional` for backwards compatibility. It is specified when creating a
  tree.
- Multiple nodes are now represented by the `Nodes` type, which is a collection of `Node`s.

## V0.1.1

- Improved support for serialization.
- Added more unit and regression tests.
- Getting children of a node now returns a reference to the ids of the children rather than the actual nodes.
- Changed the signatures of some tree methods to receive references to the node ids instead of the nodes themselves. The
  affected methods are:
    - `Tree::get_node`
    - `Tree::remove_node`
    - `Tree::get_subtree`
    - `Tree::add_subtree`

## V0.1.0

Initial release.

- Basic node creation and deletion.
- Basic tree structure.
