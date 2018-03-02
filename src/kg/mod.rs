use std::vec::Vec;

pub struct Graph {
	pub description: String,
	pub node_vector: Vec<node>,
	pub seed_node: node
}

pub struct Node {
	pub name: String,
	pub type: String,
	pub node_id: u64,
	pub edges: Vec<edge>
}

pub struct Edge {
	pub relationship: String,
	pub pair_node_id: u64
}


