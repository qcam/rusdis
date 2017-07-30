use std::io::{Read, Write};
use std::net::TcpStream;

pub struct RedisClient {
    stream: TcpStream,
}

impl RedisClient {
    pub fn connect(connection_url: String) -> RedisClient {
        let stream = TcpStream::connect(connection_url).unwrap();
        return RedisClient { stream: stream };
    }

    pub fn command(&mut self, command: String) -> Result<String, String> {
        self.stream.write(command.as_bytes()).unwrap();
        self.stream.write(&[10]).unwrap(); // new line
        self.stream.flush().unwrap();

        return RedisClient::handle_response(&mut self.stream);
    }

    fn handle_response(stream: &mut TcpStream) -> Result<String, String> {
        let mut output_buffer = [0u8; 1];
        RedisClient::read_one_byte_response(stream, &mut output_buffer);

        let mut msg_vec = Vec::new();
        RedisClient::read_full_response(stream, &mut msg_vec);
        let msg = String::from_utf8(msg_vec).unwrap();

        if &output_buffer == b"+" {
            return Ok(msg);
        } else {
            return Err(msg);
        }
    }

    fn read_one_byte_response(stream: &mut TcpStream, buf: &mut [u8; 1]) {
        stream.read(buf).unwrap();
    }

    fn read_full_response(stream: &mut TcpStream, vector: &mut Vec<u8>) {
        let mut output_buffer = [0u8; 1];

        while &output_buffer != b"\n" {
            RedisClient::read_one_byte_response(stream, &mut output_buffer);
            // stream.read(&mut output_buffer).unwrap();
            vector.push(output_buffer[0]);
        }
    }
}

fn main() {
    let string = String::from("127.0.0.1:6379");
    let mut client = RedisClient::connect(string);

    let command = String::from("SET a 200");
    let buffer = client.command(command);
    println!("{:?}", buffer);

    let command = String::from("ET a 200");
    let buffer = client.command(command);
    println!("{:?}", buffer);
}
