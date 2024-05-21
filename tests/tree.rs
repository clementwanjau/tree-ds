use tree_ds::prelude::*;

#[test]
fn test_tree() -> Result<()>{
    let mut tree: Tree<String, String> = Tree::new();
    let managing_director_node = tree.add_node(Node::new("Managing Director".to_string(), Some("Harry Doe".to_string())), None)?;
    let ceo_node = tree.add_node(Node::new("CEO".to_string(), Some("Francois Mercer".to_string())), Some(&managing_director_node))?;
    let cto_node = tree.add_node(Node::new("CTO".to_string(), Some("John Doe".to_string())), Some(&ceo_node))?;
    let cfo_node = tree.add_node(Node::new("CFO".to_string(), Some("Jane Doe".to_string())), Some(&ceo_node))?;
    let cmo_node = tree.add_node(Node::new("CMO".to_string(), Some("Alice Doe".to_string())), Some(&ceo_node))?;
    let coo_node = tree.add_node(Node::new("COO".to_string(), Some("Bob Doe".to_string())), Some(&ceo_node))?;
    let clo_node = tree.add_node(Node::new("CLO".to_string(), Some("Eve Doe".to_string())), Some(&ceo_node))?;
    let operations_manager_node = tree.add_node(Node::new("Operations Manager".to_string(), Some("Charlie Doe".to_string())), Some(&coo_node))?;
    let finance_manager_node = tree.add_node(Node::new("Finance Manager".to_string(), Some("David Doe".to_string())), Some(&cfo_node))?;
    let marketing_manager_node = tree.add_node(Node::new("Marketing Manager".to_string(), Some("Grace Doe".to_string())), Some(&cmo_node))?;
    let legal_manager_node = tree.add_node(Node::new("Legal Manager".to_string(), Some("Hannah Doe".to_string())), Some(&clo_node))?;
    let tech_manager_node = tree.add_node(Node::new("Tech Manager".to_string(), Some("Ivy Doe".to_string())), Some(&cto_node))?;
    
    // The legal team under the legal manager
    let mut legal_team = Tree::new();
    let legal_team_lead_node = legal_team.add_node(Node::new("Legal Team Lead".to_string(), Some("Katie Doe".to_string())), None)?;
    legal_team.add_node(Node::new("Legal Team Member 1".to_string(), Some("Liam Doe".to_string())), Some(&legal_team_lead_node))?;
    legal_team.add_node(Node::new("Legal Team Member 2".to_string(), Some("Mia Doe".to_string())), Some(&legal_team_lead_node))?;
    legal_team.add_node(Node::new("Legal Team Member 3".to_string(), Some("Noah Doe".to_string())), Some(&legal_team_lead_node))?;
    
    tree.add_subtree(&legal_manager_node, legal_team);
    
    // The tech team under the tech manager
    let mut tech_team = Tree::new();
    let tech_team_lead_node = tech_team.add_node(Node::new("Tech Team Lead".to_string(), Some("Olivia Doe".to_string())), None)?;
    tech_team.add_node(Node::new("Tech Team Member 1".to_string(), Some("Peter Doe".to_string())), Some(&tech_team_lead_node))?;
    tech_team.add_node(Node::new("Tech Team Member 2".to_string(), Some("Quinn Doe".to_string())), Some(&tech_team_lead_node))?;
    tech_team.add_node(Node::new("Tech Team Member 3".to_string(), Some("Ryan Doe".to_string())), Some(&tech_team_lead_node))?;
    tech_team.add_node(Node::new("Tech Team Member 4".to_string(), Some("Sarah Doe".to_string())), Some(&tech_team_lead_node))?;
    
    tree.add_subtree(&tech_manager_node, tech_team);
    
    // The finance team under the finance manager
    let mut finance_team = Tree::new();
    let finance_team_lead_node = finance_team.add_node(Node::new("Finance Team Lead".to_string(), Some("Tom Doe".to_string())), None)?;
    finance_team.add_node(Node::new("Finance Team Member 1".to_string(), Some("Ursula Doe".to_string())), Some(&finance_team_lead_node))?;
    finance_team.add_node(Node::new("Finance Team Member 2".to_string(), Some("Victor Doe".to_string())), Some(&finance_team_lead_node))?;
    finance_team.add_node(Node::new("Finance Team Member 3".to_string(), Some("Wendy Doe".to_string())), Some(&finance_team_lead_node))?;
    
    tree.add_subtree(&finance_manager_node, finance_team);
    
    // The marketing team under the marketing manager
    let mut marketing_team = Tree::new();
    let marketing_team_lead_node = marketing_team.add_node(Node::new("Marketing Team Lead".to_string(), Some("Xander Doe".to_string())), None)?;
    marketing_team.add_node(Node::new("Marketing Team Member 1".to_string(), Some("Yara Doe".to_string())), Some(&marketing_team_lead_node))?;
    marketing_team.add_node(Node::new("Marketing Team Member 2".to_string(), Some("Zane Doe".to_string())), Some(&marketing_team_lead_node))?;
    
    tree.add_subtree(&marketing_manager_node, marketing_team);
    
    // The operations team under the operations manager
    let mut operations_team = Tree::new();
    let operations_team_lead_node = operations_team.add_node(Node::new("Operations Team Lead".to_string(), Some("Abby Doe".to_string())), None)?;
    operations_team.add_node(Node::new("Operations Team Member 1".to_string(), Some("Ben Doe".to_string())), Some(&operations_team_lead_node))?;
    operations_team.add_node(Node::new("Operations Team Member 2".to_string(), Some("Cara Doe".to_string())), Some(&operations_team_lead_node))?;
    operations_team.add_node(Node::new("Operations Team Member 3".to_string(), Some("Dylan Doe".to_string())), Some(&operations_team_lead_node))?;
    operations_team.add_node(Node::new("Operations Team Member 4".to_string(), Some("Ella Doe".to_string())), Some(&operations_team_lead_node))?;
    
    tree.add_subtree(&operations_manager_node, operations_team);
    println!("{:?}", tree);
    Ok(())
}
