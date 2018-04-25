
use std::sync::Arc;
use chrono::prelude::*;
use serde_json::{Value, Error};
use bson::Bson;

use mongo_driver::client::{Client, ClientPool, Uri};
use mongo_driver::flags;

pub struct MongoAdapter{
	pool: Arc<ClientPool>,
}

impl MongoAdapter{
	pub fn new() -> MongoAdapter{
		
	    let uri = Uri::new("mongodb://localhost:27017/").unwrap();
		let pool = Arc::new(ClientPool::new(uri.clone(), None));

		MongoAdapter{pool: pool}
	}

	pub fn test_client(&self) -> i32 {
		let client = self.pool.pop();
		client.get_server_status(None).unwrap();

		let mut collection = client.get_collection("ezyplayer", "user_graph");

		collection.insert(&doc!{"key" => 1}, None).is_ok();
		return 1;
	}
}

