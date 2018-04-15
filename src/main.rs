extern crate rand;

mod graph;
//mod spotify;

use std::time::{SystemTime};
use rand::{Rng, thread_rng};
use graph::UserDataGraph;

fn test_graph(n: i32){
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
  	
  	for x in 1..100 {
  		let start = SystemTime::now();
  		udg.add_weight(&rng.gen_range(0, n).to_string(), &rng.gen_range(0, n).to_string(), 1);
  		println!("{}", start.elapsed().unwrap().subsec_nanos());
  	}
}

fn main() {
	test_graph(100);
	test_graph(1000);
	test_graph(10000);
}