use std::any::Any;

use prost::Message;
use zmq::SocketType;

use crate::generated::prover;

pub mod generated;

pub fn create_proof(input: &str) -> prover::TlsProof {
    let mut proof = prover::TlsProof::default();
    proof.id = "test".to_string();
    proof.data = input.to_string();
    prover::TlsProof::from(proof)
}

pub fn publish_binaries(socket: &zmq::Socket, binary: Vec<u8>) {
    socket.send(&binary, 0).unwrap();
}

pub fn create_socket(path: &str, socket_type: SocketType) -> zmq::Socket {
    let context = zmq::Context::new();
    let socket = context.socket(socket_type).unwrap();
    let protocol = "ipc://";
    let endpoint = format!("{}{}", protocol, path);
    socket.bind(&endpoint).unwrap();
    socket
}

pub fn publish_from_iterator<T: Iterator<Item = Box<dyn Any>>>(socket: &zmq::Socket, iterator: T) {
    for item in iterator {
        let binary = item.downcast::<Vec<u8>>().unwrap();
        publish_binaries(socket, binary.to_vec());
    }
}

pub struct SubscriptionServer {
    socket: zmq::Socket
}

impl SubscriptionServer {
    pub fn new(socket: zmq::Socket) -> Self {
        Self { socket }
    }

    pub fn send_message<T: prost::Message>(&mut self, name: &str, data: T) -> zmq::Result<()> {
        let message = data.encode_to_vec();
        let messages = vec![name.as_bytes(), &message];
        self.socket.send_multipart(messages, 0)
    }

    pub fn subscribe_to_proofs(&mut self, data: prover::TlsProof) -> zmq::Result<()> {
        self.send_message("SubscribeToProofs", data)
    }
}