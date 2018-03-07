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
	name: String,
	artists: Vec<String>,
	duration: f64,
	id: String,
	played_at: DateTime<Utc> 
}

impl Song{
	pub fn new(name: String, artists: Vec<String>, duration: f64, id: String, played_at: DateTime<Utc>) -> Song {
		Song { name: name, artists: artists, duration: duration, id: id, played_at: played_at}
	}

	pub fn to_json(&self) {
		
	}

	pub fn get_title(&self) -> String {
		return self.name.clone();
	}
}

struct SimplePM {
	playlist_size: i32
}

impl PlaylistMaker for SimplePM {
	fn new(playlist_size: i32) -> SimplePM {
		SimplePM { playlist_size: playlist_size}
	}

	fn create_playlist(&self, songs: &Vec<Song>){

		
	}

	fn save_to_db(&self) {

	}
}

