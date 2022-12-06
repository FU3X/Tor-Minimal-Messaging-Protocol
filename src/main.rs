use libtor::{Tor, TorFlag, TorAddress, HiddenServiceVersion};
use std::{io, io::Write, process::Command, thread, time::Duration};
use std::net::{TcpListener, TcpStream};
enum InputKind {
    Server{hs_port: u16, tor_port: u16},
    Username{username: String},
    Misc{result1: u16},
    Message{message: String}
}

fn main(){

    println!("Welcome to TMMP(\"Tor Minimal Messaging Protocol\"), run /help");
    let mut _global_username = String::new();
    let mut _msg = String::new();
    loop{

        let x =  get_input();

        match x {

            InputKind::Misc{result1} => {

                if result1 == 0{
                    println!("[system]$ ERROR: invalid input! Try againn");
                }

                if result1 == 1 {
                    println!("[system]$ OK exiting");
                    break;
                }

                if result1 == 2 {
                    println!("(NOTE: must be run with sudo priveleges)");
                    println!("Help: \nRun /username to change your username\n");
                    println!("Run /exit to exit the program\n");
                    println!("Run /join to join a server\n");
                    println!("Run /leave to leave a server\n");
                    println!("Run \"/server (port_for_hidden_service)\" To run a server(you will be unable to join a server until you stop yours). Max port number is 65,535 use no commas.");
                    println!("\nRun /msg to send a message\n");
                }
            },

            InputKind::Username{username} => {
                _global_username = username;
                println!("{}",_global_username);
            },

            InputKind::Server { hs_port, tor_port } => {
                let port_limit: u16 = 65535;
                println!("Creating server...");
                if hs_port == tor_port {
                    println!("[system]$ ERROR: hidden servie and tor port are the same! Exitting...");
                    break;
                }

                if hs_port > port_limit {
                    println!("[system]$ ERROR: hidden service port is greater than 65,535! Exitting...");
                    break;
                }

                if tor_port > port_limit {
                    println!("[system]$ ERROR: tor service port is greater than 65,535! Exitting...");
                    break;
                }
                command("rm -rf /tmp/tor-rust");

                let thread2 = thread::spawn(move || {

                    Tor::new()
                            .flag(TorFlag::DataDirectory("/tmp/.tmmp-tor".into()))
                            .flag(TorFlag::SocksPort(tor_port))
                            .flag(TorFlag::HiddenServiceDir("/tmp/.tmmp-tor/hs-dir".into()))
                            .flag(TorFlag::HiddenServiceVersion(HiddenServiceVersion::V3))
                            .flag(TorFlag::HiddenServicePort(TorAddress::Port(hs_port),None.into()))
                            .start_background();
                });


                if thread2.is_finished() == true {

                        let thread1 = thread::spawn(move || {
                            println!("[system]$Binding socket to port {}...",hs_port);
                            let listener_port = "127.0.0.1:".to_string()+&hs_port.to_string();
                            let listener = TcpListener::bind(listener_port);
                        });

                }

                thread2.join().unwrap();




                println!("Test");



                println!("\nstopping server\n");

            },

            InputKind::Message { message } => {

                _msg = message.clone();

            },
        };    
    }

    return;
}

fn get_input() -> InputKind {
    print!("$ ");

    io::stdout()
        .flush()
        .unwrap();

    let mut input1 = String::new();

    io::stdin()
        .read_line(&mut input1)
        .expect("Error getting inputt");

    println!();
    let len = input1.len();
    let slice1 = input1[4..len].to_string();
    let v: Vec<&str> = input1.split(' ').collect();


    if v[0].trim() == "/exit" {

        return InputKind::Misc{result1: 1};

    }

    if v[0].trim() == "/help" {

        return InputKind::Misc{result1: 2};

    }

    if v[0].trim() == "/username" {

        return InputKind::Username{username: v[1].trim().to_string()};

    }

    if v[0].trim() == "/server" {

        match v[1].parse::<u16>() {
            Ok(..) => println!(),

            Err(..) => return InputKind::Misc{result1: 0},
        };

        match v[2].trim().parse::<u16>() {
            Ok(..) => return InputKind::Server{hs_port: v[1]
            .parse::<u16>()
            .unwrap(),tor_port: v[2]
            .trim()
            .parse::<u16>()
            .unwrap()},

            Err(..) => println!(),
        };

        return InputKind::Misc{result1: 0};

    }

    if v[0].trim() == "/msg" {

        return InputKind::Message{message: slice1};
    }

    return InputKind::Misc{result1: 0};

}

fn command(stir: &str) {
    Command::new("sh")
            .arg("-c")
            .arg(stir.trim())
            .spawn()
            .expect("Failed to execute Command")
            .wait()
            .expect("Failed to execute Command");
    return;
}
