use std::fs;
use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    path::Path,
    fs::{File, OpenOptions},
};

const PASSWORD_FILE: &str = "notpassword";

fn main() {
    println!("file path: {}", PASSWORD_FILE);

    let rpa_file = create_file_if_not_exist();
    let path = Path::new(PASSWORD_FILE);

    let mut s = String::new();
    let display = path.display();

    let mut file = match rpa_file {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(_) => {
            print!("{} contains:\n{}", display, s);
        }
    }

    // --snip--
    // println!("In file {}", file_path);

    // let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    // println!("With text:\n{contents}");

    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        // println!("Connection established!");


        handle_connection(stream);
    }
}

fn create_file_if_not_exist() -> Result<File, std::io::Error> {
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(PASSWORD_FILE);

    file
}


fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    println!("http_request: {:?}", http_request);

    let status_line = "HTTP/1.1 200 OK";
    let contents = fs::read_to_string("hello.html").unwrap();
    let length = contents.len();

    let response =
        format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}
