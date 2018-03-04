extern crate serialize;
extern crate futures;
extern crate hyper;
extern crate tokio_core;

use std::io::{self, Write};
use std::vec::Vec;

use serialize::json;
use futures::{Future, Stream};
use hyper::Client;
use tokio_core::reactor::Core;

pub struct GoogleGraphClient {
	core: Core,
	client: Client,
	pub base_url: String
}

impl GoogleGraphClient {

	pub init_client() -> bool {
		core = Core::new()?;
		client = Client::new(&core.handle());
	}

	pub make_request(r: Request) -> Vec<client::Response> {
		let work = client.get(uri);
	}

	get_api_key(){
		
	}

	construct_url(r: Request) -> String{

	}
}

pub struct Request {
	query: String,
	ids: Option<String>,
	languages: String,
	types: Option<String>,
	indent: bool,
	prefix: bool,
	limit: i32
}

impl Request{
	pub fn new(query: String, limit: i32 ) -> Request{
		Request {query: query, languages: String::from("en"), indent: false, prefix: false, limit: limit}
	}
}


pub struct Response {
	@id: String,
	name: String,
	@type: Vec<String>,
	description: String,
	image: String, 
	detailedDescription: string,
	url: String,
	resultScore: i32
}

impl Response {

}