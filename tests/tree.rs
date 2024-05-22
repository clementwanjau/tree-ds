use tree_ds::prelude::*;

#[test]
fn test_tree() -> Result<()>{
    // Regression tests for the tree data structure.
    // We test all the supported operations on the tree.
    // 1. Create a tree.
    // 2. Add nodes to the tree.
    // 3. Add subtrees to the tree.
    // 4. Get a subtree from the tree.
    // 5. Remove a node from the tree.
    // 6. Remove a subtree from the tree.
    // 7. Traverse the tree.
    // 8. Serialize the tree.
    // 9. Deserialize the tree.
    
    // region:- Create a tree.
    let mut tree = Tree::new();
    // endregion
    
    // region:- Add nodes to the tree.
    let managing_director_node = tree.add_node(Node::new("Managing Director", Some("Harry Doe")), None)?;
    let ceo_node = tree.add_node(Node::new("CEO", Some("Francois Mercer")), Some(&managing_director_node))?;
    let cto_node = tree.add_node(Node::new("CTO", Some("John Doe")), Some(&ceo_node))?;
    let cfo_node = tree.add_node(Node::new("CFO", Some("Jane Doe")), Some(&ceo_node))?;
    let cmo_node = tree.add_node(Node::new("CMO", Some("Alice Doe")), Some(&ceo_node))?;
    let coo_node = tree.add_node(Node::new("COO", Some("Bob Doe")), Some(&ceo_node))?;
    let clo_node = tree.add_node(Node::new("CLO", Some("Eve Doe")), Some(&ceo_node))?;
    let operations_manager_node = tree.add_node(Node::new("Operations Manager", Some("Charlie Doe")), Some(&coo_node))?;
    let finance_manager_node = tree.add_node(Node::new("Finance Manager", Some("David Doe")), Some(&cfo_node))?;
    let marketing_manager_node = tree.add_node(Node::new("Marketing Manager", Some("Grace Doe")), Some(&cmo_node))?;
    let legal_manager_node = tree.add_node(Node::new("Legal Manager", Some("Hannah Doe")), Some(&clo_node))?;
    let tech_manager_node = tree.add_node(Node::new("Tech Manager", Some("Ivy Doe")), Some(&cto_node))?;
    // The legal team under the legal manager
    let mut legal_team = Tree::new();
    let legal_team_lead_node = legal_team.add_node(Node::new("Legal Team Lead", Some("Katie Doe")), None)?;
    legal_team.add_node(Node::new("Legal Team Member 1", Some("Liam Doe")), Some(&legal_team_lead_node))?;
    legal_team.add_node(Node::new("Legal Team Member 2", Some("Mia Doe")), Some(&legal_team_lead_node))?;
    legal_team.add_node(Node::new("Legal Team Member 3", Some("Noah Doe")), Some(&legal_team_lead_node))?;
    // The tech team under the tech manager
    let mut tech_team = Tree::new();
    let tech_team_lead_node = tech_team.add_node(Node::new("Tech Team Lead", Some("Olivia Doe")), None)?;
    tech_team.add_node(Node::new("Tech Team Member 1", Some("Peter Doe")), Some(&tech_team_lead_node))?;
    let tech_team_member_2 = tech_team.add_node(Node::new("Tech Team Member 2", Some("Quinn Doe")), Some(&tech_team_lead_node))?;
    tech_team.add_node(Node::new("Tech Team Member 3", Some("Ryan Doe")), Some(&tech_team_lead_node))?;
    tech_team.add_node(Node::new("Tech Team Member 4", Some("Sarah Doe")), Some(&tech_team_lead_node))?;
    // The finance team under the finance manager
    let mut finance_team = Tree::new();
    let finance_team_lead_node = finance_team.add_node(Node::new("Finance Team Lead", Some("Tom Doe")), None)?;
    finance_team.add_node(Node::new("Finance Team Member 1", Some("Ursula Doe")), Some(&finance_team_lead_node))?;
    finance_team.add_node(Node::new("Finance Team Member 2", Some("Victor Doe")), Some(&finance_team_lead_node))?;
    finance_team.add_node(Node::new("Finance Team Member 3", Some("Wendy Doe")), Some(&finance_team_lead_node))?;
    // The marketing team under the marketing manager
    let mut marketing_team = Tree::new();
    let marketing_team_lead_node = marketing_team.add_node(Node::new("Marketing Team Lead", Some("Xander Doe")), None)?;
    marketing_team.add_node(Node::new("Marketing Team Member 1", Some("Yara Doe")), Some(&marketing_team_lead_node))?;
    marketing_team.add_node(Node::new("Marketing Team Member 2", Some("Zane Doe")), Some(&marketing_team_lead_node))?;
    // The operations team under the operations manager
    let mut operations_team = Tree::new();
    let operations_team_lead_node = operations_team.add_node(Node::new("Operations Team Lead", Some("Abby Doe")), None)?;
    operations_team.add_node(Node::new("Operations Team Member 1", Some("Ben Doe")), Some(&operations_team_lead_node))?;
    operations_team.add_node(Node::new("Operations Team Member 2", Some("Cara Doe")), Some(&operations_team_lead_node))?;
    operations_team.add_node(Node::new("Operations Team Member 3", Some("Dylan Doe")), Some(&operations_team_lead_node))?;
    operations_team.add_node(Node::new("Operations Team Member 4", Some("Ella Doe")), Some(&operations_team_lead_node))?;
    // endregion
    
    // region:- Add subtrees to the tree.
    tree.add_subtree(&legal_manager_node, legal_team);
    tree.add_subtree(&tech_manager_node, tech_team);
    tree.add_subtree(&finance_manager_node, finance_team);
    tree.add_subtree(&marketing_manager_node, marketing_team);
    tree.add_subtree(&operations_manager_node, operations_team);
    // endregion
    
    // region:- Get a subtree from the tree.
    // The direct CEO's team including the CEO.
    assert_eq!(tree.get_subtree(&ceo_node, Some(1)).get_nodes().len(), 6);
    // The CTO's team
    assert_eq!(tree.get_subtree(&cto_node, None).get_nodes().len(), 7);
    // The finance team
    assert_eq!(tree.get_subtree(&finance_team_lead_node, None).get_nodes().len(), 4);
    // endregion
    
    // region:- Remove a node and subtree from the tree.
    tree.remove_node(&tech_team_member_2, NodeRemovalStrategy::RetainChildren)?;
    assert_eq!(tree.get_subtree(&tech_manager_node, None).get_nodes().len(), 5);
    
    tree.remove_node(&tech_manager_node, NodeRemovalStrategy::RemoveNodeAndChildren)?;
    assert_eq!(tree.get_subtree(&cto_node, None).get_nodes().len(), 1);
    
    tree.remove_node(&operations_team_lead_node, NodeRemovalStrategy::RetainChildren)?;
    assert_eq!(tree.get_subtree(&operations_manager_node, None).get_nodes().len(), 5);
    // endregion
    
    // region:- Serialize and deserialize the tree.
    let expected_str = serde_json::to_string(&tree).unwrap();
    let serialized_tree = r##"{"nodes":[{"node_id":"Managing Director","value":"Harry Doe","children":["CEO"],"parent":null},{"node_id":"CEO","value":"Francois Mercer","children":["CTO","CFO","CMO","COO","CLO"],"parent":"Managing Director"},{"node_id":"CTO","value":"John Doe","children":[],"parent":"CEO"},{"node_id":"CFO","value":"Jane Doe","children":["Finance Manager"],"parent":"CEO"},{"node_id":"CMO","value":"Alice Doe","children":["Marketing Manager"],"parent":"CEO"},{"node_id":"COO","value":"Bob Doe","children":["Operations Manager"],"parent":"CEO"},{"node_id":"CLO","value":"Eve Doe","children":["Legal Manager"],"parent":"CEO"},{"node_id":"Operations Manager","value":"Charlie Doe","children":["Operations Team Member 1","Operations Team Member 2","Operations Team Member 3","Operations Team Member 4"],"parent":"COO"},{"node_id":"Finance Manager","value":"David Doe","children":["Finance Team Lead"],"parent":"CFO"},{"node_id":"Marketing Manager","value":"Grace Doe","children":["Marketing Team Lead"],"parent":"CMO"},{"node_id":"Legal Manager","value":"Hannah Doe","children":["Legal Team Lead"],"parent":"CLO"},{"node_id":"Legal Team Lead","value":"Katie Doe","children":["Legal Team Member 1","Legal Team Member 2","Legal Team Member 3"],"parent":"Legal Manager"},{"node_id":"Legal Team Member 1","value":"Liam Doe","children":[],"parent":"Legal Team Lead"},{"node_id":"Legal Team Member 2","value":"Mia Doe","children":[],"parent":"Legal Team Lead"},{"node_id":"Legal Team Member 3","value":"Noah Doe","children":[],"parent":"Legal Team Lead"},{"node_id":"Finance Team Lead","value":"Tom Doe","children":["Finance Team Member 1","Finance Team Member 2","Finance Team Member 3"],"parent":"Finance Manager"},{"node_id":"Finance Team Member 1","value":"Ursula Doe","children":[],"parent":"Finance Team Lead"},{"node_id":"Finance Team Member 2","value":"Victor Doe","children":[],"parent":"Finance Team Lead"},{"node_id":"Finance Team Member 3","value":"Wendy Doe","children":[],"parent":"Finance Team Lead"},{"node_id":"Marketing Team Lead","value":"Xander Doe","children":["Marketing Team Member 1","Marketing Team Member 2"],"parent":"Marketing Manager"},{"node_id":"Marketing Team Member 1","value":"Yara Doe","children":[],"parent":"Marketing Team Lead"},{"node_id":"Marketing Team Member 2","value":"Zane Doe","children":[],"parent":"Marketing Team Lead"},{"node_id":"Operations Team Member 1","value":"Ben Doe","children":[],"parent":"Operations Manager"},{"node_id":"Operations Team Member 2","value":"Cara Doe","children":[],"parent":"Operations Manager"},{"node_id":"Operations Team Member 3","value":"Dylan Doe","children":[],"parent":"Operations Manager"},{"node_id":"Operations Team Member 4","value":"Ella Doe","children":[],"parent":"Operations Manager"}]}"##;
    assert_eq!(expected_str, serialized_tree);
    let deserialized_tree = serde_json::from_str::<Tree<_, _>>(serialized_tree).unwrap();
    assert_eq!(tree, deserialized_tree);
    // endregion
    
    Ok(())
}
