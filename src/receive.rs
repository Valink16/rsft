use std::net;
use std::io::{Write, Read, Cursor};
use byteorder::{ReadBytesExt, NetworkEndian};
use std::fs::File;

use super::uinput;

pub struct Receiver {
    // Structure for easily managing the receiver part
    pub stream_to_sender: net::TcpStream,
    pub receiver_addr: net::SocketAddr
}

impl Receiver {
    pub fn new() -> Receiver {
        // Sets up a receiver
        let connect_addr = uinput::get_address(uinput::OperatingMode::Recv);

        let stream = net::TcpStream::connect(connect_addr)
            .expect("Could not connect to sender");
        uinput::log(&format!("Connected to : {}", connect_addr));

        Receiver {
            stream_to_sender: stream,
            receiver_addr: connect_addr
        }
    }

    pub fn read_size(&mut self) -> u64 {
        // Receives file size from sender and returns it
        let mut file_size_data: Vec<u8> = vec![0; 8];
        self.stream_to_sender.read(&mut file_size_data)
            .expect("Could not read from sender");

        let mut file_size_reader = Cursor::new(file_size_data);
        let file_size = file_size_reader.read_u64::<NetworkEndian>()
            .expect("Could not decode file size from sender");

        return file_size;
    }

    pub fn confirm(&mut self) {
        // Sends a single byte which equals to either 1 or 0 to receiver
        if uinput::choose_between("Continue ?", "y", "n") {
            self.stream_to_sender.write(&[1])
                .expect("Failed to confirm");
        } else {
            self.stream_to_sender.write(&[0])
                .expect("Failed to confirm");
        }
    }
}

pub fn save_data(path: &str, data: Vec<u8>) {
        let mut save_file = File::create(path)
            .expect("Failed to create file to save data");
        
        uinput::log(&format!("Saved {} bytes !", save_file.write(&data).unwrap()));
    }
