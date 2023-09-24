use std::net::TcpStream;
use std::io::Write;

pub fn connect(address: String) -> Result<TcpStream, std::io::Error> {
    let stream = TcpStream::connect(address)?;
    Ok(stream)
}

pub fn send_message(stream: &mut TcpStream, message: String) -> Result<(), std::io::Error> {
    match stream.write_all(message.as_bytes()) {
        Ok(_) => {
            println!("{}", message);
            Ok(())
        }
        Err(e) => {
            return Err(e);
        }
    }
}
