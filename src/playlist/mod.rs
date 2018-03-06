extern crate serde_json;

use self::serde_json::{Value, Error};

trait PlaylistMaker {

	fn new(playlist_size: i32) -> Self;

	fn save_to_db(&self); 

	fn create_playlist(&self,  songs: &Vec<Song>);
}

struct Song{
	title: String,
	artist: String
}

impl Song{
	pub fn new(title: String, artist: String) -> Song {
		Song { title: title, artist: artist}
	}

	pub fn to_json(&self) -> String {
		
		let json_value: Value = null;
		json_value["title"] = serde_json::from_str(&self.title).unwrap();
		json_value["artist"] = serde_json::from_str(&self.artist).unwrap();

		return json_value.to_string();
	}

	pub fn get_title(&self) -> String {
		return self.title.clone();
	}
}

struct SimplePM {
	playlist_size: i32
}

impl PlaylistMaker for SimplePM {
	fn new(playlist_size: i32) -> SimplePM {
		SimplePM { playlist_size: playlist_size}
	}

	fn create_playlist(&self, songs: &Vec<Song>, client: kg_client::GoogleKnowledgeGraphClient){
		let n = songs.len();
		let q: i32 = self.playlist_size / (n as i32);

		for i in 0..n {
			//make some actual query
			let mut s = String::from("songs similar to");
			s.push_str(&songs[i].get_title());

			let request = kg_client::Request::new(s, q);
			client.make_request(request);
		}
	}

	fn save_to_db(&self) {

	}
}