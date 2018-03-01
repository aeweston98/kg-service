use std::vec::Vec;

pub struct kg {
	pub description: String,
	pub size: i32,
	pub nodes: Vec<node>
}

pub struct node {
	pub name: String,
	pub type: String,
	pub node_id: u64,
	pub edges: Vec<edge>
}

pub struct edge {
	pub relationship: String,
	pub pair_node_id: u64
}
