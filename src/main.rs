

mod server;
mod kg_client;

fn main() {

	let query = String::from("Al Green");
	let request = kg_client::Request::new(query, 1);
	let mut ggc = kg_client::GoogleGraphClient::new();

	let response: kg_client::Response = ggc.make_request(&request);
}