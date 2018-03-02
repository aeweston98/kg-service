use std::vec::Vec;

pub mod client;

//changing the structure since each individual query will
//return a list of results, but they are only connected to
//the query node, not connected together therefore 
//introducing a graph structure in my own code
//would just be extra complexity with no benefit

pub struct Query {
	pub query: String,
	pub response_vector: Vec<client::Response>,

	graph_client: client::GoogleGraphClient
}

impl Query{
	query_graph() {

	}
}