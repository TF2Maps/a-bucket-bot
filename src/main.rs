extern crate boiler;
#[macro_use] extern crate log;
extern crate log4rs;

fn main() {
    // Set up logging
    log4rs::init_file("./config/Log4rs.toml", Default::default()).unwrap();

    // Start the client
    let mut client = boiler::SteamConnection::connect();

    // Loop waiting for events
    for message in client.messages().iter() {
        debug!("Message: {:?}", message);
    }

    client.wait_close();
}
