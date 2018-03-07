extern crate serde_json;
extern crate chrono;

use self::chrono::prelude::*;
use self::serde_json::{Value, Error};

trait PlaylistMaker {

	fn new(playlist_size: i32) -> Self;

	fn save_to_db(&self); 

	fn create_playlist(&self,  songs: &Vec<Song>);
}

struct Song{
	Name: String,
	artist: Vec<String>,
	duration: f64,
	id: String,
	played_at: DateTime<Utc> 
}

impl Song{
	pub fn new(title: String, artist: String) -> Song {
		Song { title: title, artist: artist}
	}

	pub fn to_json(&self) -> String {
		
		let json_value: Value;

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

		}
	}

	fn save_to_db(&self) {

	}
}