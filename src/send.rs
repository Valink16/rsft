use super::uinput;
use std::net;
use std::fs;
use std::io::{Write, Read};
use byteorder::{NetworkEndian, WriteBytesExt};

pub struct Sender {
    // Structure for easily managing the sender part
    pub addr: net::SocketAddr,
    pub stream_to_receiver: net::TcpStream,
    pub receiver_addr: net::SocketAddr,
    pub file: fs::File,
    pub file_metadata: fs::Metadata
}

impl Sender {
    pub fn new() -> Sender {
        // Sets up a sender
        let bind_addr = uinput::get_address(uinput::OperatingMode::Send);

        let listener = net::TcpListener::bind(bind_addr)
            .expect("Could not bind listener");
        uinput::log(&format!("Listening : {}", bind_addr));

        let (stream, recv_addr) = listener.accept()
            .expect("Could not accept client's connection");
        uinput::log(&format!("Accepted client : {}", recv_addr));

        let f = uinput::read_file("Enter the name of the file to send: ");
        let fmd = f.metadata()
            .expect("Could not get file's metadata");

        Sender {
            addr: bind_addr,
            stream_to_receiver: stream,
            receiver_addr: recv_addr,
            file: f,
            file_metadata: fmd
        }
    }
    
    pub fn send_confirmation(&mut self) {
        // Sends size as a buffer with the file size encoded(using byteorder) in it to the receiver
        // Then checks for confirmation from receiver and aborts if receiver is not ready
        uinput::log("Waiting for confirmation from receiver");
        let mut file_size_writer: Vec<u8> = vec![];
        file_size_writer.write_u64::<NetworkEndian>(self.file_metadata.len())
            .expect("Unable to encode file size");

        self.stream_to_receiver.write(&file_size_writer)
            .expect("Failed to send the size of the file to receiver's stream");

        let mut confirmation_reader: [u8; 1] = [255];
        self.stream_to_receiver.read_exact(&mut confirmation_reader)
            .expect("Failed to read confirmation from receiver");

        if confirmation_reader[0] == 1 {
            uinput::log("Receiver is ready to receive !");
        } else if confirmation_reader[0] == 0 {
            panic!("Transfer cancelled by receiver, aborting");
        }
    }

    pub fn send_data(&mut self) {
        // Sends data to receiver 
        // /!\ Only use after confirmation of receiver (receiver will send a buffer containing a 1: [1])
        let mut data: Vec<u8> = vec![0; self.file_metadata.len() as usize];
        uinput::log(&format!("Successfully read {} bytes from the file", 
            self.file.read(&mut data)
                .expect("Failed to read from file")));
        
        // Sending data to receive
        
        self.stream_to_receiver.write_all(&data)
            .expect("Failed to write to receiver's stream");
        uinput::log(&format!("Successfully wrote {} bytes to the receiver's stream", data.len()));
    }
}
