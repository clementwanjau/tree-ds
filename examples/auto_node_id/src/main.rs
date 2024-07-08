use tree_ds::prelude::*;

fn main() {
    let mut tree = Tree::<AutomatedId, &str>::new(Some("Sample Tree"));
    let ceo_node = tree
        .add_node(Node::new_with_auto_id(Some("CEO")), None)
        .unwrap();
    let _coo_node = tree
        .add_node(Node::new_with_auto_id(Some("COO")), Some(&ceo_node))
        .unwrap();
    let _cto_node = tree
        .add_node(Node::new_with_auto_id(Some("CTO")), Some(&ceo_node))
        .unwrap();
    let _cfo_node = tree
        .add_node(Node::new_with_auto_id(Some("CFO")), Some(&ceo_node))
        .unwrap();
    let _cio_node = tree
        .add_node(Node::new_with_auto_id(Some("CIO")), Some(&ceo_node))
        .unwrap();
    println!("{}", tree);
}
