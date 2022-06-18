use std::net::TcpStream;
use std::thread;
use std::io::{Read, Write};

pub fn start_client(){
    let mut client = TcpStream::connect("127.0.0.1:6000").unwrap();
    println!("Connected to Server......");
    let mut write_client = client.try_clone().unwrap();    

    let read_thread = thread::spawn(move ||{
        loop{
            let mut buff = vec![0; 1024];

            match client.read(&mut buff){
                Ok(msg)=>{
                    let msg = buff.into_iter().take_while(|&x| x!=0).collect::<Vec<_>>();
                    let msg = String::from_utf8(msg).unwrap();
                    println!("Server = {}", msg);
                }
                Err(e)=>{
                    println!("Got Error = {:?}", e);
                    break;
                }
            }
        }
    });

    let write_thread = thread::spawn(move ||{
        loop{
            let mut buff = String::new();
            std::io::stdin().read_line(&mut buff).unwrap();
            let msg = buff.trim().to_string();
            write_client.write(msg.as_bytes()).unwrap();
            write_client.flush().unwrap();
        }
    });

    read_thread.join().unwrap();
    write_thread.join().unwrap();
}