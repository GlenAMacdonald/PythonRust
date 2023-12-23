use std::collections::HashMap;

use anyhow::Result;
use id_tree::{self, NodeId};

// pub struct AMap {
//     map: HashMap<String, String>
// }

pub struct Tree {
    tree: id_tree::Tree<String>
}

pub struct Node {
    node: id_tree::Node<String>
}

pub struct NodeIdMap {
    map:HashMap<String,id_tree::NodeId>
}

impl Tree {
    pub fn new() -> Self {
        let mut tree: id_tree::Tree<String> = id_tree::TreeBuilder::new()
            // .with_node_capacity(50)
            .build();

        // let root_id: id_tree::NodeId = tree.insert(id_tree::Node::new("hi".to_string()), id_tree::InsertBehavior::AsRoot).unwrap();
        // let child_id: id_tree::NodeId = tree.insert(id_tree::Node::new("Child of Hi".to_string()),id_tree::InsertBehavior::UnderNode(&root_id)).unwrap();
        Self { tree : tree }
    }

    pub fn insert_root(mut self, node: Node) -> Result<id_tree::NodeId, id_tree::NodeIdError> {
        self.tree.insert(node.node, id_tree::InsertBehavior::AsRoot)
    }

    pub fn insert(mut self, node: Node, parent: &id_tree::NodeId) -> Result<id_tree::NodeId, id_tree::NodeIdError> {
        self.tree.insert(node.node, id_tree::InsertBehavior::UnderNode(parent))
    }
}

impl Node {
    pub fn new(data: String) -> Self {
        let node: id_tree::Node<String> = id_tree::Node::new(data);
        Self { node: node }
    }
}


// impl AMap {
//     pub fn new() -> Self {
//         let mut map = HashMap::new();
//         map.insert("1".to_string(), "value".to_string());
//         Self { map: map }
//     }

//     pub fn get(&self, key: String) -> Result<&String> {
//         match self.map.get(&key) {
//             Some(v) => Ok(v),
//             None => Err(anyhow::anyhow!("{}", key)) 
//         }
//     }

//     pub fn insert(&mut self, key: String, value: String) -> (){
//         self.map.insert(key, value);
//     }
// }


// impl Node {


// }

/// Formats the sum of two numbers as string.
pub fn sum_as_string(a: usize, b: usize) -> Result<String, String> {
    let c = a + b;
    if c > 10 {
        return Err("Dummy Error, greater than 10".to_string())
    }
    Ok(c.to_string())
}