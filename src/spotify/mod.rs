use chrono::prelude::*;
use serde_json::{Value, Error};

use hyper::Client;
use hyper::net::HttpsConnector;
use hyper_native_tls::NativeTlsClient;
use serde_json::{Value, Error};


pub enum Request {
	put,
	get,
	post
}

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
		let api_key = String::from("");

		SpotifyClient{client: client, base_url: String::from("https://api.spotify.com/v1/users/"), api_key: api_key}
	}

	pub fn request(&self, uri: String, t: Request) -> {

		match t {
			Request::put => {
				self.client.put();
			}
			Request::get => {
				self.client.get();
			}
			Request::post => {
				self.client.post();
			}
		}
	}
}

pub struct SpotifyPlaylist {
	playlist_id: String,
	user_id: String,
	spotify_client: &SpotifyClient
}

impl SpotifyPlaylist {

	pub fn new(playlist_id: String, user_id: String, spotify_client: &SpotifyClient) -> SpotifyPlaylist {
		SpotifyPlaylist{playlist_id: playlist_id, user_id: user_id, spotify_client: spotify_client}
	}

	pub fn create_playlist() -> bool {
		//first we need to double check the playlist doesn't exist already
		self.spotify_client.request("", Request::post);

		//

		if true {
			return true;
		}
		else {
			return false;
		}
	}

	//change the songs in a playlist to a new set
	pub fn update_playlist() -> bool {
		//first check that the playlist we are looking for exists
		self.spotify_client.request("", Request::put);
		//then sent the put request to update it
	}

	pub fn check_for_playlist(&self, playlist_id: &String) -> bool {
		//get list of user playlists
		self.spotify_client.request("", Request::get);

		//check if the playlist we are looking for is in the list
	}
}
