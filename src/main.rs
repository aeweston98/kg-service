

mod server;
mod playlist;
mod kg_client;
mod graph;

fn main() {

	let query = String::from("Purple Haze");
	let request = kg_client::Request::new(query, 5);
	let ggc = kg_client::GoogleGraphClient::new();

	let response: Vec<kg_client::Response> = ggc.make_request(&request);

}