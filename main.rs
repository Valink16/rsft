mod uinput;
mod send;
mod receive;

use std::io::{Read, stdin};
use byteorder::{NetworkEndian, ReadBytesExt};
use std::time::Duration;
use std::option::Option;

fn main() {
    println!("Welcome to RSFT - Rust Simple File Transfer");

    match uinput::choose_between("What do you want to do ? send or receive ?", "send", "receive") {
        true => {
            let mut sender = send::Sender::new();

            sender.send_confirmation();
            sender.send_data();
        }
        false => {
            let mut receiver = receive::Receiver::new();

            let file_size = receiver.read_size();
            let mut data: Vec<u8> = vec![];

            uinput::log(&format!("File is {} bytes big", file_size));
            receiver.confirm();

            uinput::log("Starting to receive...");
            uinput::log(&format!("Successfully read {} bytes from sender's stream", 
                receiver.stream_to_sender.read_to_end(&mut data)
                    .expect("Failed to receive data")));
            
            uinput::log("Save as: ");
            let mut file_name_reader = String::new();
            stdin().read_line(&mut file_name_reader)
                .expect("Failed to read file name");
            receive::save_data(file_name_reader.trim(), data);
        }
    }
}
