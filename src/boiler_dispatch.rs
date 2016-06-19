//! Experimental boiler library for dispatching messages to handlers

use boiler::{Message, EMsg};

pub trait MessageHandler {
    fn invoke(&mut self, message: Message);
}

impl<F: FnMut(Message)> MessageHandler for F {
    fn invoke(&mut self, message: Message) {
        self(message);
    }
}

pub struct MessageDispatcher<'a> {
    handlers: Vec<Option<Box<MessageHandler + 'a>>>,
    fallback: Option<Box<FnMut(Message)>>
}

impl<'a> MessageDispatcher<'a> {
    pub fn new() -> Self {
        // Fill the vector with Nones, we can't use vec! for this
        let mut handlers = Vec::with_capacity(10000);
        for _ in 0..8000 {
            handlers.push(None);
        }

        MessageDispatcher {
            handlers: handlers,
            fallback: None
        }
    }

    pub fn register<H: MessageHandler + 'a>(&mut self, msg: EMsg, handler: H) {
        self.handlers[msg as usize] = Some(Box::new(handler));
    }

    pub fn register_fallback(&mut self, handler: Box<FnMut(Message)>) {
        self.fallback = Some(handler);
    }

    pub fn handle(&mut self, message: Message) {
        if let &mut Some(ref mut handler) = &mut self.handlers[message.header.emsg() as usize] {
            handler.invoke(message);
            return;
        }

        // We were unable to find anything, call the fallback
        self.invoke_fallback(message);
    }

    fn invoke_fallback(&mut self, message: Message) {
        if let &mut Some(ref mut handler) = &mut self.fallback {
            handler(message);
        }
    }
}
