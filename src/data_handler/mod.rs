
use std::sync::Arc;
use chrono::prelude::*;
use serde_json::{Value, Error};
use bson::Bson;

use mongo_driver::client::{Client, ClientPool, Uri};

pub struct MongoAdapter{
	pool: Arc<ClientPool>,
}


impl MongoAdapter{
	pub fn new() -> MongoAdapter{
		
	    let uri = Uri::new("mongodb://localhost:27017/").unwrap();
		let pool = Arc::new(ClientPool::new(uri.clone(), None));
		//let client = pool.pop();
		//client.get_server_status(None).unwrap();

		MongoAdapter{pool: pool}
	}
}

