use std::io;
use std::io::Write;
use libtor::{Tor, TorFlag, TorAddress, HiddenServiceVersion};
use std::process::Command;
use std::iter;

fn main(){
    
    println!("Welcome to TMMP(\"Tor Minimal Messaging Protocol\"), run /help");
    let mut username = String::new(); 
    let mut port1: u16;
    let mut port2: u16;
     
    loop{

        let x = get_input();
                
        if  x.0== 0 {
            println!("[system]$ ERROR: Invalid Input! Try again\n");
        }

        if x.0 == 1 {
            println!("Exitting, Goodbye!\n");
            break;
        }

        if x.0 == 2 {

            println!("Help: \nRun /username to change your username\n");
            println!("Run /exit to exit the program\n");
            println!("Run /join to join a server\n");
            println!("Run /leave to leave a server\n");
            println!("Run \"/server (port_for_hidden_service)\" To run a server(you will be unable to join a server 
            until you stop yours). Max port number is 65,535 use no commas.\n");
            println!("Run /msg to send a message\n");
            
        }

        if x.0 == 3{

            username = x.1.clone();

        }

        if x.0 == 4{

            println!("Creating server...");
            port1 = x.2;
            port2 = x.3;
            println!("port1: {} port2: {}",port1,port2);

            println!("\nstopping server\n");

        }
        
        println!("The username is {}\n",username);     
    }

    return;
}

fn get_input() -> (u32, String, u16, u16) {
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

        return (1,v[0].to_string(),0,0);

    }

    if v[0].trim() == "/help" {

        return (2,v[0].to_string(),0,0);

    }

    if v[0].trim() == "/username" {

        return (3,v[1].to_string(),0,0);

    }

    if v[0].trim() == "/server" {
        println!("{}",v[0].trim());
        match v[1].parse::<u16>() {
            Ok(..) => return(4,v[0].to_string(),v[1]
            .parse::<u16>()
            .unwrap(),v[2]
            .parse::<u16>()
            .unwrap()),

            Err(..) => {println!("Not ok")},
        };
        

        return (0,v[0].to_string(),0,0);

    }

    return (0,v[0].to_string(),0,0);

}

/* fn server(port: u16) {
  
    println!("\n[system]$ starting server on port {}",port);
    command("rm -rf /tmp/tor-rust");
    match Tor::new()
            .flag(TorFlag::DataDirectory("/tmp/tor-rust".into()))
            .flag(TorFlag::SocksPort(19050))
            .flag(TorFlag::HiddenServiceDir("/tmp/tor-rust/hs-dir".into()))
            .flag(TorFlag::HiddenServiceVersion(HiddenServiceVersion::V3))
            .flag(TorFlag::HiddenServicePort(TorAddress::Port(port),None.into()))
            .start() {
                Ok(..) => println!("Finished"),
                Err(..) => println!("Couldnt do it"),
            };

    println!("[system]$ Finished initializing tor daemon\nTor sock running on port 19050\nTor Hidden Service running on port {}",port);


    return;
} */

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
