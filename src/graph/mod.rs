

use std::collections::HashMap;

struct UserDataGraph{
	graph_vec: Vec<Node>,
	id_lookup: HashMap<String, i32>,
	edge_table: HashMap<String, i32>
}

impl UserDataGraph{
	pub fn create_node(&mut self, new_node_id: String){
		let new_node = Node::new(new_node_id);

		let new_node_index = self.id_lookup.len() as i32;

		self.id_lookup.insert(new_node.get_id(), new_node_index);
		self.graph_vec.push(new_node);
	}

	pub fn create_edge(&mut self,  node_1_id: String, node_2_id: String, start_weight: i32) -> bool {
		let i_1: Option<&i32> = self.id_lookup.get(&node_1_id);
		let i_2: Option<&i32> = self.id_lookup.get(&node_2_id);

		if i_1 != None && i_2 != None {	//both of the nodes exist
			let key: String = self.edge_table_hash(node_1_id, node_2_id);

			//scope the test to check if the edge already exists since it uses a borrow and insert needs a mutable reference
			{ 
				let weight = self.edge_table.get(&key);

				if weight != None{
	    			println!("Attempted to create an edge that already exists");
					return false;
				}
			}
			
			self.edge_table.insert(key, start_weight);
			return true;
		}
		else{
			println!("Attempted to make an edge with a non-existant node");
			return false;
		}
	}

	pub fn add_weight(&mut self, node_1_id: String, node_2_id: String, inc: i32) -> bool{
		let key: String = self.edge_table_hash(node_1_id, node_2_id);

		if let Some(weight) = self.edge_table.get_mut(&key) {
    		*weight += inc;
    		return true;
		}
		else{
			println!("Attempted to add weight to non-existant edge");
			return false;
		}
	}

	//later I think I will want to update all the hashing from extremely long string id's
	//to some smarter hash function which returns i32's to hash on
	//this is a performance/space concern, not necessary right away
	fn edge_table_hash(&self, node_1_id: String, node_2_id: String) -> String {
		let mut result: String = node_1_id.clone();
		result.push_str(&node_2_id);

		return result;
	}

	//need to have some function to implement < operator on string type
	fn str_cmp(&self, str1: String, str2: String){

	}
}

struct Edge{
	relationship_weight: i32,
	destination_node: i32
}

struct Node{
	cluster_label: Option<i32>,
	id: String,
	edges: Vec<Edge>
}

impl Node{
	pub fn new(id: String) -> Node {
		let edges: Vec<Edge> = Vec::new();
		Node{cluster_label: None, id: id, edges: edges}
	}

	pub fn get_id(&self) -> String {
		return self.id.clone();
	}
}

