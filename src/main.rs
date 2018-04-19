extern crate rand;
extern crate serde_json;
extern crate chrono;
extern crate bson;
extern crate mongo_driver;

mod graph;
mod data_handler;
//mod spotify;
//mod test;

use std::time::{SystemTime};
use rand::{Rng, thread_rng};
use graph::UserDataGraph;
use data_handler::MongoAdapter;


fn test_graph(n: i32) -> UserDataGraph {
	let mut udg: UserDataGraph = UserDataGraph::new();

	for x in 1..n {
		udg.create_node(&x.to_string());
	}

	let mut rng = thread_rng();
	
  	for x in 1..n {
  		for y in x+1..n{
  			udg.create_edge(&x.to_string(), &y.to_string(), 1);
  			udg.add_weight(&x.to_string(), &y.to_string(), rng.gen_range(0, 100));
  		}
  	}
  	
  	for x in 1..100{
  		//let start = SystemTime::now();
  		udg.add_weight(&rng.gen_range(0, n).to_string(), &rng.gen_range(0, n).to_string(), 1);
  		//println!("{}", start.elapsed().unwrap().subsec_nanos());
  	}

  	udg
}

fn main() {
	/*
	let mut udg = test_graph(1000);
	let mut size: i32 = 25;

	println!("Built graph, attempting to create cluster");
	let tuple = udg.attempt_cluster(&String::from("1"), size);
 
	let mut cluster_score: i32 = tuple.0;
	let mut cluster_vec = tuple.1;

	println!("Cluster score is {} out of {}", cluster_score, size);
	println!("{:?}", cluster_vec);
	*/

	let temp = MongoAdapter::new();
}