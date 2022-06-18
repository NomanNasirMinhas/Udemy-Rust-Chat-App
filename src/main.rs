mod server;
mod client;
use std::io;

fn main() {
    println!("Welcome to Chat Application");
    println!("Please choose type of the application ");
    println!("1-Server");
    println!("2-Client");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Unable to read the input");

    let typeApp:i8 = input.trim().parse().unwrap();

    if typeApp == 1{
        println!("Starting Chat Server......");
        server::start_server();
    }
    else if typeApp ==2{
        println!("Starting Chat Client......");
        client::start_client();
    }
    else{
        println!("Please Provide a valid choice");
    }
}   
