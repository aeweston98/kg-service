
use std::collections::HashMap;
use std::thread;

use spotify;


pub struct UserDataGraph{
	graph_vec: Vec<Node>,
	id_lookup: HashMap<String, i32>,
	edge_table: HashMap<String, i32>
}

impl UserDataGraph{

	pub fn new() -> UserDataGraph{
		let graph_vec = Vec::new();
		let id_lookup = HashMap::new();
		let edge_table = HashMap::new();
		UserDataGraph{graph_vec: graph_vec, id_lookup: id_lookup, edge_table: edge_table}
	}

	pub fn create_node(&mut self, new_node_id: &String) -> bool {
		if self.id_lookup.contains_key(new_node_id) {
			return false;
		}
		else{
			let new_node = Node::new(new_node_id.clone());

			let new_node_index = self.id_lookup.len() as i32;

			self.id_lookup.insert(new_node.get_id(), new_node_index);
			self.graph_vec.push(new_node);
			return true;
		}
	}

	pub fn create_edge(&mut self,  node_1_id: &String, node_2_id: &String, start_weight: i32) -> bool {
		let i_1: Option<&i32> = self.id_lookup.get(node_1_id);
		let i_2: Option<&i32> = self.id_lookup.get(node_2_id);

		if i_1 != None && i_2 != None {	//both of the nodes exist
			let i1 = *(i_1.unwrap());
			let i2 = *(i_2.unwrap());

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

			//once we know we are inserting a new edge, we can add it to each Node in the vec
			if let Some(node1) = self.graph_vec.get_mut(i1 as usize){
				node1.add_edge(i2, node_2_id.clone());
			}
			else{
				println!("Attempted to make an edge with a non-existant node");
				return false;
			}

			if let Some(node2) = self.graph_vec.get_mut(i2 as usize){
				node2.add_edge(i1, node_1_id.clone());
			}
			else{
				println!("Attempted to make an edge with a non-existant node");
				return false;
			}
		}
		else{
			println!("Attempted to make an edge with a non-existant node");
			return false;
		}
		return true;
	}

	pub fn add_weight(&mut self, node_1_id: &String, node_2_id: &String, inc: i32) -> bool{
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
	fn edge_table_hash(&self, node_1_id: &String, node_2_id: &String) -> String {
		let mut order = false;
		{
			order = self.str_cmp(node_1_id, node_2_id);
		}

		let mut result = String::new();
		if order {
			result = node_1_id.clone();
			result.push_str(node_2_id);
		}
		else{
			result = node_2_id.clone();
			result.push_str(node_1_id);
		}

		return result;
	}

	//need to have some function to implement < operator on string type
	//it doesn't actually matter what algorithm is used here as long as it is
	//consistent for an input and its reverse 
	//ie. str_cmp("a", "b") = true and str_cmp("b","a") = false

	fn str_cmp(&self, str1: &String, str2: &String) -> bool {
		//if str1 < str2 return true
		//else return false

		let l1 = (*str1).len();
		let l2 = (*str2).len();
		
		if l1 < l2 {
			return true;
		}
		if l1 > l2 {
			return false;
		}

		let mut c1 = (*str1).chars().next().unwrap();
		let mut c2 = (*str2).chars().next().unwrap();
		let mut i1 = c1 as u8;
		let mut i2 = c2 as u8;
		//they are the same length
		//iterate over the string and return upon first decision
		for i in 1..l1 {
			if i1 < i2 {
				return true;
			}
			if i1 > i2 {
				return false;
			}
			if i == l1-1 {
				break;
			}
			c1 = (*str1).chars().next().unwrap();
			c2 = (*str2).chars().next().unwrap();
			let mut i1 = c1 as u8;
			let mut i2 = c2 as u8;
		}

		return true;
	}

	pub fn node_exists(&self, node_id: &String) -> bool {
		let result = self.id_lookup.contains_key(node_id);
		return result;
	}

	pub fn edge_exists(&self, node_1_id: &String, node_2_id: &String) -> bool {
		let key = self.edge_table_hash(node_1_id, node_2_id);
		let result = self.edge_table.contains_key(&key);
		return result;
	}
/*
	pub fn generate_cluster(&self, num_nodes: i32) -> Vec<String> {
		//generate 4 threads and call attempt_cluster on each
		let t1 = thread::spawn(move || { 
		});

		//join the 4 threads


		//pick the resulting cluster with the highest score and return it


	}
*/

	fn attempt_cluster(&self, start_node: &Node, thread_num: usize, num_nodes: i32) -> (i32, Vec<String>) {
		//initialize the cluster score and set for cluster nodes
		let cluster_score: i32 = 0;
		let cluster_nodes: Vec<String> = Vec::new();
		let num_visited: i32 = 0;

		//add the start node to the node set and mark it as visited
		cluster_nodes.push((*start_node).get_id());
		(*start_node).set_visited(thread_num);
		let next_node: Node = graph_vec[*start_node.get_max_edge()];

		while num_visited < num_nodes {
			if next_node.visited[thread_num] {
				//pick another edge at random
			}

		}

		(cluster_score, cluster_nodes)
	}
}

struct Node{
	id: String,
	edges: Vec<(i32, String)>,
	visited: [bool; 4],
	max_edge: i32
}

impl Node{
	pub fn new(id: String) -> Node {
		let edges: Vec<(i32, String)> = Vec::new();
		let visited = [false, false, false, false];
		let max_edge = -1;
		Node{id: id, edges: edges, visited: visited, max_edge: max_edge}
	}

	pub fn get_id(&self) -> String {
		return self.id.clone();
	}

	pub fn get_max_edge(&self) -> i32 {
		return self.max_edge;
	}

	//this function assumes that the given edge does not already exist
	//this relies on some checks done in graph::add_edge on the edge hashmap since
	//it is much more efficient to check if an edge is an entry in the hash map
	//then to search the vector for it
	pub fn add_edge(&mut self, new_index: i32, new_id: String){
		self.edges.push((new_index, new_id));
	}

	pub fn clear_visited(&mut self) {
		self.visited = [false, false, false, false];
	}

	pub fn set_visited(&mut self, i: usize) {
		self.visited[i] = true;
	}
}

