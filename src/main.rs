use std::io;
use std::io::Write;
use libtor::{Tor, TorFlag, TorAddress, HiddenServiceVersion};
use std::process::Command;

fn main(){
    
    println!("Welcome to TMMP(\"Tor Minimal Messaging Protocol\"), run /help");
    let mut username = String::new(); 
    let mut port: u16;
     
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
            println!("Run /server (port) to run a server(you will be unable to join a server until you stop yours)\n");
            println!("Run /msg to send a message\n");
            
        }

        if x.0 == 3{

            username = x.1.clone();

        }

        if x.0 == 4{

            println!("Creating server...");
            port = x.2;
            server(port);
            println!("\nstopping server\n");

        }
        
        println!("The username is {}\n",username);     
    }

    return;
}

fn get_input() -> (u32, String, u16) {
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
    let mut slice1 = String::from("Empty");
    let mut slice2 = String::from("Empty");
    let mut slice3 = String::from("Empty");
    let mut _slice4 = String::from("Empty");
    
    let mut port : &str = "empty";
    if len > 9 {

        if len > 35 {

            println!("$ ERROR: Too Many Characters!\n");
            return(0,input1,0);
        }

        slice1 = input1[..9].to_string();
        slice2 = input1[9..len].to_string();
        slice3 = input1[..7].to_string();
        _slice4 = input1[7..len].to_string();
        port = _slice4.trim();

        
    }

    if input1.trim() == "/exit" {

        return (1,input1,0);

    }

    if input1.trim() == "/help" {

        return (2,input1,0);

    }

    if slice1.trim() == "/username" {

        return (3,slice2,0);

    }

    if slice3.trim() == "/server" {
        println!("{}",slice3.trim());
        match port.parse::<u16>() {
            Ok(..) => return (4, input1, port.parse::<u16>().unwrap()),
            Err(..) => println!("[system] ERROR! Invalid port!\n"),
        };

        return (0,slice2,0);

    }

    return (0,input1,0);

}

fn server(port: u16) {
  
    println!("\n[system]$ starting server on port {}",port);
    command("rm -rf /tmp/tor-rust");
    match Tor::new()
            .flag(TorFlag::DataDirectory("/tmp/tor-rust".into()))
            .flag(TorFlag::SocksPort(19050))
            .flag(TorFlag::HiddenServiceDir("/tmp/tor-rust/hs-dir".into()))
            .flag(TorFlag::HiddenServiceVersion(HiddenServiceVersion::V3))
            .flag(TorFlag::HiddenServicePort(TorAddress::Port(port),None.into()))
            .start(){
                        Ok(..) => println!("Did it"),
                        Err(..) => println!("couldnt do it"),

            };
    println!("[system]$ Finished initializing tor daemon\nTor sock running on port 19050\nTor Hidden Service running on port {}",port);


    return;
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
