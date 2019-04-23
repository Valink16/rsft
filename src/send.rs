use super::uinput;
use std::net;
use std::fs;
use std::io::{Write, Read};
use byteorder::{NetworkEndian, WriteBytesExt};
use std::io::{BufReader};
use std::io::{Seek, SeekFrom};

pub struct Sender {
    // Structure for easily managing the sender part
    pub addr: net::SocketAddr,
    pub stream_to_receiver: net::TcpStream,
    pub receiver_addr: net::SocketAddr,
    pub buf_file_reader: BufReader<fs::File>,
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
            buf_file_reader: BufReader::new(f),
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
        
        // Sending data to receiver
        loop {
            let mut data_buffer: [u8; 5000000] = [0; 5000000]; // The buffer will read 5MB at a time
            self.buf_file_reader.read_exact(&mut data_buffer)
                .expect("Failed to buffer data");
            println!("Cursor position: {}", self.buf_file_reader.seek(SeekFrom::Current(0)).unwrap());
            self.stream_to_receiver.write(&data_buffer)
                .expect("Failed to write to receiver");
            uinput::log(&format!("Successfully wrote {} bytes to the receiver's stream", data_buffer.len()));
        }
    }
}
