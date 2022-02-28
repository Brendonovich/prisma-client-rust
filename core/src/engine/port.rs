use std::net::TcpListener;

pub fn get_port() -> String {
    let listener = TcpListener::bind("localhost:0").unwrap();

    listener.local_addr().unwrap().port().to_string()
}
