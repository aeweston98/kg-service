use chrono::prelude::*;
use serde_json::{Value, Error};

use hyper::Client;
use hyper::net::HttpsConnector;
use hyper_native_tls::NativeTlsClient;
use serde_json::{Value, Error};


pub struct SpotifyClient {
	client: Client,
	base_url: String,
	api_key: String
}

impl SpotifyClient {

	pub fn new() -> SpotifyClient {
		let ssl = NativeTlsClient::new().unwrap();
		let connector = HttpsConnector::new(ssl);
		let client = Client::with_connector(connector);

		SpotifyClient{client: client, base_url: String::from(""), api_key: api_key}
	}

}