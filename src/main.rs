

mod server;
mod kg_client;

fn main() {
	let mut ggc = kg_client::GoogleGraphClient::new();
	
	ggc.make_request();
}