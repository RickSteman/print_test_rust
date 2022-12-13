use std::{
        env::args,
        io::{Read, Write},
};

use std::io;

fn main() {
        println!("\n To interact with the UART the following commands are supported: \n\
        s -- Store (This stores the note supplied) \n\
        r -- Retrieve (This retrieves the note associated with the supplied ID) \n\
        d -- Delete (This deletes the note associated with the supplied ID) \n\
        l -- ListID (This lists all IDs in use) \n \n\
        Please input one of the mentioned commands:");

        let mut user_input = String::new();
        io::stdin().read_line(&mut user_input);

        match user_input.as_str() {
                "s\n" => {println!("Please supply a note: \n");

                        io::stdin().read_line(&mut user_input);
                        println!("note supplied is: {}", user_input)},
                "r\n" => {println!("Please supply an ID: \n");
                        io::stdin().read_line(&mut user_input);
                        println!("ID supplied is: {}", user_input)},
                "d\n" => {println!("Please supply an ID: \n");
                        io::stdin().read_line(&mut user_input);
                        println!("ID supplied is: {}", user_input)},
                "l\n" => println!("ServerProtocol::ListIds"),
                _ => println!("invalid command"),
        };
}
