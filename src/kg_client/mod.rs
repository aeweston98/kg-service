extern crate hyper;
extern crate hyper_native_tls;

use std::io::{self, Write, Read};
use std::fs::File;
use self::hyper::Client;
use self::hyper::net::HttpsConnector;
use self::hyper_native_tls::NativeTlsClient;

pub struct GoogleGraphClient {
	base_url: String,
	client: Client
}

impl GoogleGraphClient {

	pub fn new() -> GoogleGraphClient {
		let ssl = NativeTlsClient::new().unwrap();
		let connector = HttpsConnector::new(ssl);
		let client = Client::with_connector(connector);

		return GoogleGraphClient{client: client, base_url: String::from("https://kgsearch.googleapis.com/v1/entities:search?")};
	}

	pub fn make_request(&self) {

		let mut s = String::new();

		let mut api_key = self.get_api_key();

		let mut target_uri = &self.base_url;

	    let mut res = self.client.get(target_uri).send()
	                    .unwrap()
	                    .read_to_string(&mut s)
	                    .unwrap();
		
    	println!("Result: {}", s);
	}
	
	fn get_api_key(&self) -> String{
		let mut filename = "src/kg_client/api_key.txt".to_string();
		let mut f = File::open(filename).expect("file not found");

		let mut contents = String::new();
		f.read_to_string(&mut contents).expect("something went wrong reading the file");

		return contents;
	}
	
	fn construct_url(&self, request: &Request){

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
		Request {query: query, ids: None, languages: String::from("en"), types: None, indent: false, prefix: false, limit: limit}
	}
}


pub struct Response {
	id: String,
	name: String,
	types: Vec<String>,
	description: String,
	result_score: f64
}

impl Response {

}