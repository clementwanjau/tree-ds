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


## License
This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
