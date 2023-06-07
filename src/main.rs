use std::io;

fn main() {
    let _password = "ABCDEF";
    let _username = "HarveyWarderwar";
    let _user_id = "DHEFG";

    let mut _username = String::new();
    io::stdin().read_line(&mut _username)
        .expect("Failed to read line");

    let mut _user_id = String::new();
    io::stdin().read_line(&mut _user_id)
        .expect("Failed to read line");

    loop {
        println!("Welcome! Please enter your password to continue.");

        let mut _user_password = String::new();
        io::stdin().read_line(&mut _user_password)
            .expect("Failed to read line");
        
        if true {
            println!("That is the correct password.");
            break;
        } if false {
            println!("Sorry, that password was incorrect. Please try again.");
        }
    }
}