# Tree-ds

[![Build Status](https://github.com/clementwanjau/tree-ds/actions/workflows/build.yaml/badge.svg)](https://github.com/clementwanjau/tree-ds/actions/workflows/build.yaml)
[![Crates.io](https://img.shields.io/crates/v/tree-ds.svg)](https://crates.io/crates/tree-ds)
[![Documentation](https://docs.rs/tree-ds/badge.svg)](https://docs.rs/tree-ds)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

This library provides a tree data structure that can be used to represent
hierarchical data in Rust. The library
allows you to perform the following operations on a tree:

- Enumerating all nodes in the tree
- Enumerating a section of the tree
- Finding a node in the tree
- Finding a section of the tree
- Adding a node to a certain position in the tree
- Removing a node from the tree
- Pruning a section of the tree
- Grafting a whole section of the tree onto another tree
- Finding the root of any node
- Finding the lowest common ancestor of two nodes

## Why use this library?

There are many crates that make tree data structures available in Rust, but
this library is unique in that it provides a tree data structure that is
feature rich, easy to use and has a simple API. The library is also
well-documented and has a comprehensive test suite that ensures that it works
as expected. Most importantly, the library is designed to be fast and
efficient, making it suitable for use in performance-critical applications.

## Usage

Add the following to your `Cargo.toml` file:

```toml copy
[dependencies]
tree-ds = "0.1"
```

A crude example of how to use the library is shown below:

```rust
use tree_ds::prelude::{Node, NodeRemovalStrategy, Result, Tree};

fn main() -> Result<()> {
	let mut tree = Tree::new(Some("Finances Tree"));
	let root = tree.add_node(Node::new("Risk".to_string(), Some(5000)), None)?;
	let fixed_income_node = tree.add_node(Node::new("Fixed Income".to_string(), Some(2000)), Some(&root))?;
	let equity_node = tree.add_node(Node::new("Equity".to_string(), Some(3000)), Some(&root))?;
	let debt_node = tree.add_node(Node::new("Debt".to_string(), Some(1000)), Some(&fixed_income_node))?;
	let mutual_funds_node = tree.add_node(Node::new("Mutual Funds".to_string(), Some(1000)), Some(&equity_node))?;
	let stocks_node = tree.add_node(Node::new("Stocks".to_string(), Some(2000)), Some(&equity_node))?;
	tree.add_node(Node::new("Debt Mutual Funds".to_string(), Some(500)), Some(&debt_node))?;
	tree.add_node(Node::new("Equity Mutual Funds".to_string(), Some(500)), Some(&mutual_funds_node))?;
	tree.add_node(Node::new("Large Cap Stocks".to_string(), Some(1000)), Some(&stocks_node))?;
	tree.add_node(Node::new("Mid Cap Stocks".to_string(), Some(1000)), Some(&stocks_node))?;
	tree.add_node(Node::new("Small Cap Stocks".to_string(), Some(1000)), Some(&stocks_node))?;

	println!("{}", tree);


	tree.remove_node(&stocks_node, NodeRemovalStrategy::RemoveNodeAndChildren);
	println!("After Removing The Stocks Node");
	println!("*******************");
	println!("{}", tree);


	let equity_sub_tree = tree.get_subtree(&equity_node, None);
	println!("{}", equity_sub_tree);
	Ok(())
}

```

This will output:

```
Finances Tree
*************
Risk: 5000
├── Fixed Income: 2000
│   └── Debt: 1000
│       └── Debt Mutual Funds: 500
└── Equity: 3000
    ├── Mutual Funds: 1000
    │   └── Equity Mutual Funds: 500
    └── Stocks: 2000
        ├── Large Cap Stocks: 1000
        ├── Mid Cap Stocks: 1000
        └── Small Cap Stocks: 1000

After Removing The Stocks Node
*******************
Finances Tree
*************
Risk: 5000
├── Fixed Income: 2000
│   └── Debt: 1000
│       └── Debt Mutual Funds: 500
└── Equity: 3000
    └── Mutual Funds: 1000
        └── Equity Mutual Funds: 500

Equity
******
Equity: 3000
└── Mutual Funds: 1000
    └── Equity Mutual Funds: 500
```

## Traversal

You can traverse the tree using the `traverse` method. The `traverse` method
returns an iterator that allows you to traverse the tree in any order you want.
The following example shows how to traverse the tree in a pre-order fashion:

```rust
use tree_ds::prelude::{Node, Result, Tree, TraversalOrder};

fn main() -> Result<()> {
	let mut tree = Tree::new();
	let node_1 = tree.add_node(Node::new(1, Some(2)), None).unwrap();
	let node_2 = tree.add_node(Node::new(2, Some(3)), Some(&node_1)).unwrap();
	let node_3 = tree.add_node(Node::new(3, Some(6)), Some(&node_1)).unwrap();
	let node_4 = tree.add_node(Node::new(4, Some(5)), Some(&node_2)).unwrap();
	let node_5 = tree.add_node(Node::new(5, Some(6)), Some(&node_2)).unwrap();
	let node_6 = tree.add_node(Node::new(6, Some(7)), Some(&node_3)).unwrap();
	let preorder_nodes = tree.traverse(TraversalStrategy::PreOrder, &node_1);
	let expected_preorder = vec![node_1, node_2, node_4, node_5, node_3, node_6];
	assert_eq!(preorder_nodes, expected_preorder);

	let in_order_nodes = tree.traverse(TraversalStrategy::InOrder, &node_1);
	let expected_in_order = vec![node_4, node_2, node_5, node_1, node_3, node_6];
	assert_eq!(in_order_nodes, expected_in_order);

	let post_order_nodes = tree.traverse(TraversalStrategy::PostOrder, &node_1);
	let expected_post_order = vec![node_4, node_5, node_2, node_6, node_3, node_1];
	assert_eq!(post_order_nodes, expected_post_order);
	Ok(())
}
```

You can also perform an action on the nodes while traversing the tree on the iterator returned by the `traverse` method.
The following example shows how to traverse the tree in a pre-order fashion and perform an action on the nodes:

```rust,ignore
let nodes = tree.traverse(TraversalOrder::PreOrder)
    .map(|node| {
        println!("{}", node);
        node
    })
    .collect::<Vec<_>>();

```

## Roadmap

- Add support for more tree operations.
    - Add node rotation.
- Add a macro to create trees from a DSL.
- Add support for weighted nodes.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
