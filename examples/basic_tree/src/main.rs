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
