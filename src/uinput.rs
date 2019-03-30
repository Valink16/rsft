use std::{io, net, fs, path};

pub enum OperatingMode {
    Send,
    Recv
}

pub fn choose_between(message: &str, o1: &str, o2: &str) -> bool { 
    // Prints the message, reads an input and returns true or false depending on the user's choice
    let op1 = o1.to_lowercase();
    let op2 = o2.to_lowercase();
    loop { // reask for input 
        log(&format!("{} ({}/{})", message, o1, o2));
        let mut input = String::new();
        io::stdin().read_line(&mut input)
            .expect("Failed to read user input");
        let processed_input = input.trim().to_lowercase();
        if processed_input == op1 {
            break true; // user choose o1
        } else if processed_input == op2 {
            break false; // user choose o2
        }
    }
}

pub fn get_address(mode: OperatingMode) -> net::SocketAddr {
    // Gets an address to bind to depending on OperatingMode
    if let OperatingMode::Send = mode { // You only need to know the port to open when sending
        return net::SocketAddr::new (
            net::IpAddr::V4(net::Ipv4Addr::new(127, 0, 0, 1)), // we'll bind our listener to loopback
            //get_port("Enter the port you want to open (preferred: 49152-65535): ")
            50000
        );
    } else { // You need both the sender's IP and port when receiving
        return net::SocketAddr::new (
            get_ip("Enter sender's IP address(IPv4 or IPv6): "),
            //get_port("Enter sender's port: ")
            50000
        );
    }
}

fn get_ip(msg: &str) -> net::IpAddr {
    println!("{}", msg);
    net::IpAddr::V4(loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input)
           .expect("Failed to read user input");
        
        match input.trim().parse::<net::Ipv4Addr>() {
            Ok(ip) => break ip,
            Err(e) => {
                println!("Please enter a valid IPv4 address, Error: {}", e);
                continue;
            }
        }
    })
}

fn get_port(msg: &str) -> u16 {
    println!("{}", msg);
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input)
            .expect("Failed to read user input");
        match input.trim().parse::<u16>() {
            Ok(p) => break p,
            Err(e) => {
                println!("Please enter a valid port, Error: {}", e);
                continue;
            }
        }
    }
}

pub fn read_file(msg: &str) -> fs::File {
    log(msg);
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input)
            .expect("Failed to read user input"); 
        let file_name = input.trim();
        let file_path = path::Path::new(&file_name);
        match fs::File::open(file_path) {
            Ok(f) => break f,
            Err(e) => println!("{} can't be opened, please retry, {}", file_path.display(), e)
        }
    }
}

pub fn log(msg: &str) {
    // Simply beautifies log messages
    println!("[*] {}", msg);
}