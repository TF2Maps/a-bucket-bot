use std::thread::{self, JoinHandle};
use std::io::{Read, Write, Cursor};
use std::sync::mpsc::{self, Sender, Receiver};
use byteorder::{ReadBytesExt, WriteBytesExt, LittleEndian};
use mio::{Handler, EventLoop, Token, EventSet, PollOpt};
use mio::tcp::TcpStream;
use crypto;
use steam_data::Message;

enum SteamConnectionEvent {
    Shutdown,
    SendMessage(Message),
    SetKey(Vec<u8>)
}

struct SteamConnectionRuntime {
    stream: TcpStream,
    token: Token,
    event_set: EventSet,
    msg_sender: Sender<Message>,
    queued_messages: Vec<Message>,
    encryption_key: Option<Vec<u8>>
}

impl Handler for SteamConnectionRuntime {
    type Timeout = ();
    type Message = SteamConnectionEvent;

    fn ready(&mut self, event_loop: &mut EventLoop<SteamConnectionRuntime>, _token: Token, events: EventSet) {
        trace!("Handling ready() event with EventSet {:?}...", events);

        if events.is_readable() {
            self.readable();
        }

        if events.is_writable() {
            self.writeable(event_loop);
        }
    }

    fn notify(&mut self, event_loop: &mut EventLoop<SteamConnectionRuntime>, event: SteamConnectionEvent) {
        trace!("Handling notify() event...");

        match event {
            SteamConnectionEvent::Shutdown => {
                debug!("Shutting down event loop...");
                event_loop.shutdown();
            },
            SteamConnectionEvent::SendMessage(msg) => {
                debug!("Received message for sending");
                self.queued_messages.push(msg);
                self.event_set.insert(EventSet::writable());
                event_loop
                    .reregister(&self.stream, self.token, self.event_set, PollOpt::edge())
                    .unwrap();
            },
            SteamConnectionEvent::SetKey(key) => {
                debug!("Switching to encrypted mode");
                self.encryption_key = Some(key);
            }
        }
    }
}

impl SteamConnectionRuntime {
    fn readable(&mut self) {
        // TODO: Detect if we were dropped by the server, in which case we'll read 0 bytes
        trace!("Received connection readable event");

        // Read in the packet header
        let mut header = vec![0u8; 8];
        self.stream.read(&mut header).unwrap();
        let mut header_c = Cursor::new(header);
        let length = header_c.read_u32::<LittleEndian>().unwrap();

        // Sanity check
        let magic = header_c.read_u32::<LittleEndian>().unwrap();
        if magic != 0x31305456 {
            debug!("Data for following panic: {} {}", length, magic);
            panic!("Invalid magic, you are not connecting to a steam server");
        }

        // Read in the actual data
        let mut data = vec![0u8; length as usize];
        self.stream.read(&mut data).unwrap();

        // Decrypt the data if encryption is on
        if let &Some(ref key) = &self.encryption_key {
            data = crypto::symmetric_decrypt(&data, key);
        }

        // Turn the data into a message and send it over
        let msg = Message::parse(&mut Cursor::new(&data));
        self.msg_sender.send(msg).unwrap();
    }

    fn writeable(&mut self, event_loop: &mut EventLoop<SteamConnectionRuntime>) {
        // Go over all queued messages
        for msg in &self.queued_messages {
            trace!("Sending queued message...");

            // TODO: Perhaps send this all at once? See if it improves performance later.
            // TODO: Pre-estimate the packet size needed.
            let packet: Vec<u8> = Vec::new();
            let mut packet_c = Cursor::new(packet);

            // Turn the message into data
            let mut msg_data = msg.into_bytes();

            // Encrypt the data if encryption is on
            if let &Some(ref key) = &self.encryption_key {
                msg_data = crypto::symmetric_encrypt(&msg_data, key);
            }

            // Write the packet header and the message to the data
            packet_c.write_u32::<LittleEndian>(msg_data.len() as u32).unwrap(); // Msg length
            packet_c.write_u32::<LittleEndian>(0x31305456).unwrap(); // Magic
            packet_c.write(&msg_data).unwrap();

            // Send the message finally
            self.stream.write(&packet_c.into_inner()).unwrap();
        }

        // Now that all of those are sent, clear the list and stop asking for writeable
        self.queued_messages.clear();
        self.event_set.remove(EventSet::writable());
        event_loop
            .reregister(&self.stream, self.token, self.event_set, PollOpt::edge())
            .unwrap();
    }
}

/// A token to interact with a steam connection. Used to receive and send messages from and to a
/// steam server.
pub struct SteamConnection {
    runtime: JoinHandle<()>,
    incoming_receiver: Receiver<Message>,
    event_sender: ::mio::Sender<SteamConnectionEvent>
}

impl SteamConnection {
    /// Connects to a steam server. Aquires the server it should connect to automatically.
    pub fn connect() -> Self {
        // Set up the channels to send messages through
        let (incoming_sender, incoming_receiver) = mpsc::channel();

        // Set up the event loop and get the event sender from it
        let event_loop = EventLoop::new().unwrap();
        let event_sender = event_loop.channel();

        // Start the runtime
        let handle = thread::Builder::new()
            .name("boiler-runtime".into())
            .spawn(move || {
                Self::client_runtime(event_loop, incoming_sender);
            })
            .unwrap();

        // Return the token that keeps track of the runtime
        SteamConnection {
            runtime: handle,
            incoming_receiver: incoming_receiver,
            event_sender: event_sender,
        }
    }

    /// Disconnects this connection.
    pub fn disconnect(&mut self) {
        debug!("Sending disconnect event to runtime...");
        self.event_sender.send(SteamConnectionEvent::Shutdown).unwrap();
    }

    /// Blocks until a single message has been received, then returns it.
    pub fn recv(&mut self) -> Message {
        self.incoming_receiver.recv().unwrap()
    }

    /// Queues up a single message for sending.
    pub fn send(&mut self, message: Message) {
        self.event_sender.send(SteamConnectionEvent::SendMessage(message)).unwrap();
    }

    /// Notifies the runtime to use an ecryption key from now on for all messages.
    pub fn set_encryption_key(&mut self, key: Vec<u8>) {
        self.event_sender.send(SteamConnectionEvent::SetKey(key)).unwrap();
    }

    /// Blocks until this connection has been closed.
    pub fn wait_close(self) {
        self.runtime.join().unwrap();
    }

    fn client_runtime(mut event_loop: EventLoop<SteamConnectionRuntime>, msg_sender: Sender<Message>) {
        // Get a server to connect to
        // TODO: Take one address from a list of many
        // TODO: Actually fetch a server using the API
        let server_addr = "72.165.61.185:27017".parse().unwrap();
        info!("Connecting to {}", server_addr);

        // Start the connection to the server
        let stream = TcpStream::connect(&server_addr).unwrap();
        let event_set = EventSet::readable();
        let token = Token(0);
        event_loop.register(&stream, token, event_set, PollOpt::edge()).unwrap();

        // Run the event loop
        let mut runtime = SteamConnectionRuntime {
            stream: stream,
            token: token,
            event_set: event_set,
            msg_sender: msg_sender,
            queued_messages: Vec::new(),
            encryption_key: None
        };
        event_loop.run(&mut runtime).unwrap();
    }
}
