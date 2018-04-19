
use std::sync::Arc;
use chrono::prelude::*;
use serde_json::{Value, Error};
use bson::Bson;

use mongo_driver::client::{Client, ClientPool, Uri};

pub struct MongoAdapter{
}


impl MongoAdapter{
	pub fn new() {
		
	    let uri = Uri::new("mongodb://localhost:27017/").unwrap();
		let pool = Arc::new(ClientPool::new(uri.clone(), None));
		let client = pool.pop();
		client.get_server_status(None).unwrap();

		//DataHandler{client: client}
	}

}

