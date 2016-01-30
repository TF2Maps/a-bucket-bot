extern crate boiler;
#[macro_use] extern crate log;
extern crate log4rs;

fn main() {
    // Set up logging
    log4rs::init_file("./config/Log4rs.toml", Default::default()).unwrap();

    // Start the client
    let mut client = boiler::SteamConnection::connect();

    // Loop over events that get sent to us
    loop {
        let message = client.messages().recv().unwrap();

        debug!("Message: {:?}", message);
        client.disconnect();
        break;
    }

    client.wait_close();
}
