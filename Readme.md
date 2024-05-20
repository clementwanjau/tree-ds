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

```toml
[dependencies]
tree-ds = "0.1"
```

A crude example of how to use the library is shown below:

```rust
use tree_ds::prelude::{Node, NodeRemovalStrategy, Result, Tree};

fn main() -> Result<()> {
	let mut tree = Tree::new();
	let root = tree.add_node(Node::new("Risk", Some(5000)), None)?;
	let fixed_income_node = tree.add_node(Node::new("Fixed Income", Some(2000)), Some(root))?;
	let equity_node = tree.add_node(Node::new("Equity", Some(3000)), Some(root))?;
	let debt_node = tree.add_node(Node::new("Debt", Some(1000)), Some(fixed_income_node))?;
	let mutual_funds_node = tree.add_node(Node::new("Mutual Funds", Some(1000)), Some(equity_node))?;
	let stocks_node = tree.add_node(Node::new("Stocks", Some(2000)), Some(equity_node))?;
	tree.add_node(Node::new("Debt Mutual Funds", Some(500)), Some(debt_node))?;
	tree.add_node(Node::new("Equity Mutual Funds", Some(500)), Some(mutual_funds_node))?;
	tree.add_node(Node::new("Large Cap Stocks", Some(1000)), Some(stocks_node))?;
	tree.add_node(Node::new("Mid Cap Stocks", Some(1000)), Some(stocks_node))?;
	tree.add_node(Node::new("Small Cap Stocks", Some(1000)), Some(stocks_node))?;

	println!("Original Tree");
	println!("*********************");
	println!("{}", tree);


	tree.remove_node(stocks_node, NodeRemovalStrategy::RemoveNodeAndChildren);
	println!("After Removing The Stocks Node");
	println!("*******************");
	println!("{}", tree);


	let equity_sub_tree = tree.get_subtree(equity_node, None);
	println!("Equity Sub Tree");
	println!("*********************");
	println!("{}", equity_sub_tree);
	Ok(())
}

```

This will output:

```
Original Tree
*********************
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
Risk: 5000
├── Fixed Income: 2000
│   └── Debt: 1000
│       └── Debt Mutual Funds: 500
└── Equity: 3000
    └── Mutual Funds: 1000
        └── Equity Mutual Funds: 500

Equity Sub Tree
*********************
Equity: 3000
└── Mutual Funds: 1000
    └── Equity Mutual Funds: 500
```

## Roadmap

- Add support for more tree operations.
- Add a macro to create trees from a DSL.
- Add support for weighted nodes.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
