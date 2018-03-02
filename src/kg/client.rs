extern crate serialize;
extern crate futures;
extern crate hyper;
extern crate tokio_core;

use serialize::json;
use std::io::{self, Write};
use futures::{Future, Stream};
use hyper::Client;
use tokio_core::reactor::Core;

struct GoogleGraphClient {
	core: Core,
	client: Client,
	target_url: String
}

impl GoogleGraphClient {

	pub init_client() -> {
		core = Core::new()?;
		client = Client::new(&core.handle());
	}

	pub make_request() -> {
		let work = client.get(uri);
	}
}


struct Request {
	pub query: String,
	pub ids: String,
	pub languages: String,
	pub types: String,
	pub indent: bool,
	pub prefix: bool,
	pub limit: i32
}

struct Response {
	pub @id: String,
	pub name: String, 
	pub @type: Vec<String>,
	pub description: String,
	pub image: String, 
	pub detailedDescription: string,
	pub url: String,
	pub resultScore: i32
}