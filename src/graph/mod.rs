

use std::collections::HashMap;

struct UserDataGraph{
	graph_vec: Vec<Node>,
	id_lookup: HashMap<String, i32>
}

impl UserDataGraph{
	pub fn insert_node(&self, new_node: Node){
		self.id_lookup.insert(new_node.id, self.id_lookup.len());
		self.graph_vec.push(new_node);
	}

	pub fn create_edge(&self, new_edge: Edge, node_1_id: String, node_2_id: String){
		let i_1: Option<&i32> = self.id_lookup.get(&node_1_id);
		let i_2: Option<&i32> = self.id_lookup.get(&node_2_id);

		let node_1: Option
	}
}

struct Edge{
	relationship_weight: i32,
	destination_node: i32
}

struct Node{
	cluster_label: i32,
	id: String,
	edges: Vec<Edge>
}

