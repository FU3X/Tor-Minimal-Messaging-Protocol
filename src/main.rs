use std::io;
use std::io::Write;
use libtor::{Tor, TorFlag, TorAddress, HiddenServiceVersion};

fn main(){

    println!("Welcome to TMMP(\"Tor Minimal Messaging Protocol\"), run /help");
    let mut username = String::new(); 

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
            println!("Run /serveron (port) to run a server(you will be unable to join a server until you stop yours)\n");
            println!("Run /msg to send a message\n");

        }

        if x.0 == 3{

            username = x.1.clone();

        }

        if x.0 == 4{

            println!("Creating server...");
            server();
            println!("stopping server");

        }

        println!("The username is {}\n",username);     
    }

    return;
}

fn get_input() -> (u32, String) {
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

    if len > 9 {

        if len > 35 {

            println!("$ ERROR: Too Many Characters!\n");
            return(0,input1);
        }

        slice1 = input1[..9].to_string();
        slice2 = input1[9..len].to_string();

    }

    if input1.trim() == "/exit" {

        return (1,input1);

    }

    if input1.trim() == "/help" {

        return (2,input1);

    }

    if slice1.trim() == "/username" {

        return (3,slice2);

    }

    if input1.trim() == "/server" {

        return (4,slice2);

    }

    return (0,input1);

}

fn server() -> u32 {

    let mut server_info;  
    loop {

        server_info = get_port();

        if server_info.0 == 1 {

            break;

        }
        if server_info.0 == 2 {

            return 1;

        }

        println!("[system]$ ERROR: Not a valid port! Try again\n");
    }

    println!("[system]$ starting server on port {}",server_info.1);

    return 0;
}

fn get_port() -> (u32, u32) {
    let mut port0 = String::new();

    print!("[system]$ What port do you wish to run the server on: ");

    io::stdout()
       .flush()
       .unwrap(); 

    io::stdin()
        .read_line(&mut port0)
        .expect("Couldnt Get Input");

    let port = port0.trim();

    if port == "/exit" {

        return (2,0);

    }

    match port.parse::<u32>() {

        Ok(..) => return(1,port.parse::<u32>().unwrap()),

        Err(..) => println!(),

    };

    return (0,1);

}
