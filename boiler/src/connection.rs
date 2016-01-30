use std::thread::{self, JoinHandle};
use std::io::{Read, Cursor};
use byteorder::{ReadBytesExt, LittleEndian};
use mio::{Handler, EventLoop, Token, EventSet, PollOpt};
use mio::tcp::TcpStream;
use steam_data::MessageHeader;
use std::sync::mpsc::{self, Sender, Receiver};

/// A message to be sent to or received from a steam server.
#[derive(Debug)]
pub struct Message {
    pub header: MessageHeader,
    pub body: Vec<u8>
}

struct SteamConnectionRuntime {
    stream: TcpStream,
    msg_sender: Sender<Message>
}

impl Handler for SteamConnectionRuntime {
    type Timeout = ();
    type Message = ();

    fn ready(&mut self, _event_loop: &mut EventLoop<SteamConnectionRuntime>, _token: Token, events: EventSet) {
        trace!("Handling ready() event...");

        if events.is_readable() {
            trace!("Received connection readable event");

            // Read in the header for this packet
            let mut header = vec![0u8; 8];
            self.stream.read(&mut header).unwrap();
            let mut header_c = Cursor::new(header);
            let length = header_c.read_u32::<LittleEndian>().unwrap();

            // Sanity check
            let magic = header_c.read_u32::<LittleEndian>().unwrap();
            if magic != 0x31305456 {
                panic!("Invalid magic, you are not connecting to a steam server");
            }

            // Read in the actual data
            let mut data = vec![0u8; length as usize];
            self.stream.read(&mut data).unwrap();

            // TODO: decrypt the data here if encryption is on

            // Handle the packet of data we've prepared
            self.handle_packet(&data);
        }
        else {
            debug!("Received unhandled event type");
        }
    }
}

impl SteamConnectionRuntime {
    fn handle_packet(&mut self, data: &Vec<u8>) {
        let mut data_c = Cursor::new(data);

        // Parse in the header
        let header = MessageHeader::parse(&mut data_c);

        // Get the remaining data
        let mut body = Vec::new();
        data_c.read_to_end(&mut body).unwrap();

        // Move the message into the channel so people can handle it
        let msg = Message {
            header: header,
            body: body
        };
        self.msg_sender.send(msg).unwrap();
    }
}

/// A token to interact with a steam connection. Used to receive and send messages from and to a
/// steam server.
pub struct SteamConnection {
    runtime: JoinHandle<()>,
    msg_receiver: Receiver<Message>
}

impl SteamConnection {
    /// Connects to a steam server. Aquires the server it should connect to automatically.
    pub fn connect() -> Self {
        // Set up the channels to send messages through
        let (sender, receiver) = mpsc::channel();

        // Start the runtime
        let handle = thread::Builder::new()
            .name("boiler-runtime".into())
            .spawn(move || {
                Self::client_runtime(sender);
            })
            .unwrap();

        // Return the token that keeps track of the runtime
        SteamConnection {
            runtime: handle,
            msg_receiver: receiver
        }
    }

    /// Returns a reference to the message receiver associated with this connection. Can be used to
    /// receive messages.
    pub fn messages(&mut self) -> &mut Receiver<Message> {
        &mut self.msg_receiver
    }

    /// Blocks until this connection has been closed.
    pub fn wait_close(self) {
        self.runtime.join().unwrap();
    }

    fn client_runtime(msg_sender: Sender<Message>) {
        // Get a server to connect to
        // TODO: Take one address from a list of many
        // TODO: Actually fetch a server using the API
        let server_addr = "72.165.61.185:27017".parse().unwrap();
        info!("Connecting to {}", server_addr);

        // Create an event loop
        let mut event_loop = EventLoop::new().unwrap();

        // Start the connection to the server
        let stream = TcpStream::connect(&server_addr).unwrap();
        let token = Token(0);
        event_loop.register(
                &stream, token, EventSet::readable(),
                PollOpt::edge()
            ).unwrap();

        // Run the event loop
        let mut runtime = SteamConnectionRuntime {
            stream: stream,
            msg_sender: msg_sender
        };
        event_loop.run(&mut runtime).unwrap();
    }
}
