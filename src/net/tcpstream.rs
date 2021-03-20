pub struct TcpStream {}

impl TcpStream {
    pub fn connect() -> std::io::Result<Self> {
        Err(std::io::Error::new(std::io::ErrorKind::Other, "test"))
    }
}
