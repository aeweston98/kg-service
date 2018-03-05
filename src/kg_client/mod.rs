extern crate hyper;
extern crate hyper_native_tls;
extern crate serde_json;

use std::io::{self, Write, Read};
use std::fs::File;

use self::hyper::Client;
use self::hyper::net::HttpsConnector;
use self::hyper_native_tls::NativeTlsClient;
use self::serde_json::{Value, Error};


pub struct GoogleGraphClient {
	client: Client,
	base_url: String,
	api_key: String
}

impl GoogleGraphClient {

	pub fn new() -> GoogleGraphClient {
		let ssl = NativeTlsClient::new().unwrap();
		let connector = HttpsConnector::new(ssl);
		let client = Client::with_connector(connector);

		let mut filename = "src/kg_client/api_key.txt".to_string();
		let mut f = File::open(filename).expect("file not found");
		let mut api_key = String::new();
		f.read_to_string(&mut api_key).expect("something went wrong reading the file");

		return GoogleGraphClient{client: client, base_url: String::from("https://kgsearch.googleapis.com/v1/entities:search?"), api_key: api_key};
	}

	pub fn make_request(&self, request: &Request) -> Response {
		let mut target_uri = self.base_url.clone();
		self.construct_url(request, &mut target_uri);

		let mut s = String::new();
	    let mut res = self.client.get(&target_uri).send().unwrap().read_to_string(&mut s).unwrap();
		
    	println!("Result: {}", s);

    	let json_response: Value = serde_json::from_str(&s).unwrap();
    	
    	//println!("id: {}", id);
    	let response = self.parse_response(&json_response, 0);
    	return response;
	}
	
	fn construct_url(&self, request: &Request, s: &mut String){
		
		let temp = str::replace(&request.query, " ", "+");
		s.push_str("query=");
		s.push_str(&temp);

		s.push_str("&key=");
		s.push_str(&self.api_key);

		s.push_str("&languages=");
		s.push_str(&request.languages);

		s.push_str("&limit=");
		s.push_str(&request.limit.to_string());
	}

	fn parse_response(&self, json_response: &Value, i: usize) -> Response {
		let response = Response::new(&json_response, i);
		return response;
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
	pub fn new(copy_query: String, limit: i32 ) -> Request{
		let query = copy_query.clone();
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
	pub fn new(json_response: &Value, i: usize) -> Response {
		let id = json_response["itemListElement"][i]["result"]["@id"].to_string();
		let name = json_response["itemListElement"][i]["result"]["name"].to_string();
		let description = json_response["itemListElement"][i]["result"]["@id"].to_string();
		let result_score = json_response["itemListElement"][i]["resultScore"].as_f64().unwrap();
		
		let temp = json_response["itemListElement"][i]["result"]["@type"].as_array().unwrap();

		Response {id: id, name: name, types: types, description: description, result_score: result_score }
	}
}