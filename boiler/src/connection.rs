use std::thread::{self, JoinHandle};
use std::io::{Read, Write, Cursor};
use std::sync::mpsc::{self, Sender, Receiver};
use byteorder::{ReadBytesExt, WriteBytesExt, LittleEndian};
use flate2::read::GzDecoder;
use mio::{Handler, EventLoop, Token, EventSet, PollOpt};
use mio::tcp::TcpStream;
use boiler_generated::ProtoMessage;
use boiler_generated::steammessages_base::{CMsgMulti, CMsgProtoBufHeader};
use boiler_generated::steammessages_clientserver::CMsgClientHeartBeat;
use crypto;
use steam_data::{EMsg, Message, MessageHeader, MsgHdrProtoBuf};

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
                trace!("Received message for sending");
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

        // Turn the data into a message
        let msg = Message::parse(&mut Cursor::new(&data));

        // If it's not a multi-message, we can send it and finish
        if msg.header.emsg() != EMsg::Multi {
            trace!("Received single message from server");
            self.msg_sender.send(msg).unwrap();
            return;
        }

        // If it is a multi message however, we need to split it apart
        trace!("Received multi message from server");

        // First, parse in the protobuf message
        let mut multi = CMsgMulti::new();
        multi.merge_from_bytes(&msg.body).unwrap();

        let mut payload = Vec::from(multi.get_message_body());

        // See if we need to unzip the payload and if yes, do so
        let size_unzipped = multi.get_size_unzipped() as usize;
    	if size_unzipped != 0 {
            trace!("Unzipping multi payload...");
            let mut decoder = GzDecoder::new(Cursor::new(payload)).unwrap();
            let mut unzipped_payload = vec![0u8; size_unzipped];
            decoder.read(&mut unzipped_payload).unwrap();
            payload = unzipped_payload;
    	}

        // Parse and send all the individual messages
        let mut remaining_bytes = payload.len();
        let mut cursor = Cursor::new(&payload);
        while remaining_bytes > 0 {
            // Get the size of the sub message for sanity checking later on
            let submsg_size = cursor.read_u32::<LittleEndian>().unwrap() as usize;

            // Get the sub-message's data TODO: Make this zero-copy
            let mut data = vec![0u8; submsg_size];
            cursor.read_exact(&mut data).unwrap();

            // Parse and send the actual message
            let message = Message::parse(&mut Cursor::new(&data));
            self.msg_sender.send(message).unwrap();

            // Reduce the remaining so we actually stop the loop when all is read
            remaining_bytes -= 4 + submsg_size;
        }
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

    /// Starts the heartbeat runtime with an interval of the given amount of seconds.
    pub fn start_heartbeat(&mut self, interval: i32, session_id: i32) {
        // Start the heartbeat
        let sender = self.event_sender.clone();
        let _handle = thread::Builder::new()
            .name("boiler-heartbeat".into())
            .spawn(move || {
                Self::heartbeat_runtime(interval, session_id, sender);
            })
            .unwrap();
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

    fn heartbeat_runtime(interval: i32, session_id: i32, event_sender: ::mio::Sender<SteamConnectionEvent>) {
        debug!("Starting at interval of {}", interval);

        loop {
            // Wait for the given amount of seconds
            ::std::thread::sleep(::std::time::Duration::new(interval as u64, 0));

            // Fill the heartbeat payload
            let body = CMsgClientHeartBeat::new();

            // Build up the heartbeat message
            let mut hdr_proto = CMsgProtoBufHeader::new();
            hdr_proto.set_client_sessionid(session_id);
            hdr_proto.set_steamid(76561197960265728);
            let header = MsgHdrProtoBuf {
                msg: EMsg::ClientHeartBeat,
                proto: hdr_proto,
            };
            let message = Message {
                header: MessageHeader::MsgHdrProtoBuf(header),
                body: body.write_to_bytes().unwrap()
            };

            // Send that message
            event_sender.send(SteamConnectionEvent::SendMessage(message)).unwrap();
            trace!("Dispatched heartbeat to runtime");
        }
    }
}
