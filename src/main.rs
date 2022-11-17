use std::io;
use std::io::Write;


fn main() {
    println!("Welcome to TMMP(\"Tor Minimal Messaging Protocol\"), run /help");
    let mut username = String::new(); //note to self: leave this variable outside f the loop in
                                      //order to keep it from resetting every time
    


    loop{
        let x = get_input();
        
        
        if  x.0== 0 {
            println!("[system]$ ERROR: Invalid Input! Try again");
        }

        if x.0 == 1 {
            println!("Exitting, Goodbye!");
            break;
        }

        if x.0 == 2 {
            println!("Help: \n Run /username to change your username");
            println!("Run /exit to exit the program");
            println!("Run /join to join a server");
            println!("Run /leave to leave a server");
            println!("Run /server_run to run a server(you will be unable to join a server until you stop yours)");
        }

        if x.0 == 3{
            username = x.1;
        }

        println!("The username is {}",username);     
    }



    return;
}

fn get_input() -> (u32, String) {
    print!("$ ");
    io::stdout().flush().unwrap();
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1)
        .expect("Error getting inputt");
    
    let len = input1.len();
    let mut slice1 = String::from("Empty");
    let mut slice2 = String::from("Empty");

    if len > 9 {
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

    return (0,input1);

}
