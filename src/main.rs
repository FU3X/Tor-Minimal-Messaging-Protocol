use std::io;
use std::io::Write;
use libtor::{Tor, TorFlag, TorAddress, HiddenServiceVersion};
use std::process::Command;
use std::thread;
use std::time::Duration;

enum InputKind {
    Server{port1: u16, port2: u16},
    Username{username: String},
    Misc{result1: u16}
}
fn main(){
    
    println!("Welcome to TMMP(\"Tor Minimal Messaging Protocol\"), run /help");
    let mut global_username = String::new();
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
                    println!("Run /msg to send a message\n");
                }
            },

            InputKind::Username{username} => {
                global_username = username;
                println!("{}",global_username);
            },

            InputKind::Server { port1, port2 } => {
                
                println!("Creating server...");
            
                command("rm -rf /tmp/tor-rust");
                let thread2 = thread::spawn(|| {
                    thread::sleep(Duration::from_secs(5));
                    println!();
                    println!("Starting server...");
                
                });

                Tor::new()
                    .flag(TorFlag::DataDirectory("/tmp/.tmmp-tor".into()))
                    .flag(TorFlag::SocksPort(port2))
                    .flag(TorFlag::HiddenServiceDir("/tmp/.tmmp-tor/hs-dir".into()))
                    .flag(TorFlag::HiddenServiceVersion(HiddenServiceVersion::V3))
                    .flag(TorFlag::HiddenServicePort(TorAddress::Port(port1),None.into()))
                    .start_background();

            
                println!("Test");

                thread2.join().unwrap();
            
                println!("\nstopping server\n");

            }
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
            Ok(..) => return InputKind::Server{port1: v[1]
            .parse::<u16>()
            .unwrap(),port2: v[2]
            .trim()
            .parse::<u16>()
            .unwrap()},

            Err(..) => println!(),
        };
        

        return InputKind::Misc{result1: 0};

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
