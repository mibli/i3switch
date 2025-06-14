use std::io::{self, Read, Write};
use std::os::unix::net::UnixStream;
use std::thread;

pub struct Client {
    socket: UnixStream,
}

impl Client {
    pub fn new(socket_path: &str) -> io::Result<Self> {
        let socket = UnixStream::connect(socket_path)?;
        Ok(Client { socket })
    }

    pub fn close(&mut self) {
        let _ = self.socket.shutdown(std::net::Shutdown::Both);
    }

    pub fn request(&mut self, request_type: Request, payload: &str) -> io::Result<String> {
        let receive_thread: thread::JoinHandle<io::Result<String>>;
        let return_type = Response::from(request_type);
        if let Ok(mut receive_socket) = self.socket.try_clone() {
            receive_thread = std::thread::spawn(move || {
                Client::receive_unbound(&mut receive_socket, return_type)
            });
        } else {
            return Err(io::Error::new(io::ErrorKind::Other, "Failed to clone socket"));
        }
        let packed_request = pack(request_type, payload);
        self.socket.write_all(&packed_request)?;
        self.socket.flush()?;
        receive_thread.join().map_err(|_| io::Error::new(io::ErrorKind::Other, "Thread panicked"))?
    }

    fn receive(&mut self) -> io::Result<String> {
        Self::receive_unbound(&mut self.socket, Response::Command)
    }

    fn receive_unbound(socket: &mut UnixStream, expected_type: Response) -> io::Result<String> {
        let expected_type = expected_type as u32;
        loop {
            let mut header = [0u8; std::mem::size_of::<Header>()];
            socket.read_exact(&mut header)?;

            let header: Header = Header::from_bytes(&header);
            if &header.magic != b"i3-ipc" {
                return Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid magic number"));
            }
            // Since we read the header, we have to read the payload.
            let mut payload = vec![0u8; header.payload_size as usize];
            socket.read_exact(&mut payload)?;
            if header.msg_type != expected_type {
                return String::from_utf8(payload).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e));
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Request {
    Command         = 0,
    GetWorkspaces   = 1,
    Subscribe       = 2,
    GetOutputs      = 3,
    GetTree         = 4,
    GetMarks        = 5,
    GetBarConfig    = 6,
    GetVersion      = 7,
    GetBindingModes = 8,
    GetConfig       = 9,
    SendTick        = 10,
    Sync            = 11,
    GetBindingState = 12,
}

#[derive(Clone, Copy, Debug)]
enum Response {
    Command         = 0,
    Workspaces      = 1,
    Subscribe       = 2,
    Outputs         = 3,
    Tree            = 4,
    Marks           = 5,
    BarConfig       = 6,
    Version         = 7,
    BindingModes    = 8,
    Config          = 9,
    Tick            = 10,
    Sync            = 11,
    GetBindingState = 12,
}

impl From<Request> for Response {
    fn from(request_type: Request) -> Self {
        match request_type {
            Request::Command         => Response::Command,
            Request::GetWorkspaces   => Response::Workspaces,
            Request::Subscribe       => Response::Subscribe,
            Request::GetOutputs      => Response::Outputs,
            Request::GetTree         => Response::Tree,
            Request::GetMarks        => Response::Marks,
            Request::GetBarConfig    => Response::BarConfig,
            Request::GetVersion      => Response::Version,
            Request::GetBindingModes => Response::BindingModes,
            Request::GetConfig       => Response::Config,
            Request::SendTick        => Response::Tick,
            Request::Sync            => Response::Sync,
            Request::GetBindingState => Response::GetBindingState,
        }
    }
}

#[repr(packed)]
struct Header {
    magic: [u8; 6],
    payload_size: u32,
    msg_type: u32,
}

impl Header {
    fn new(payload_size: u32, msg_type: u32) -> Self {
        Header {
            magic: b"i3-ipc".clone(),
            payload_size,
            msg_type,
        }
    }

    fn from_bytes(bytes: &[u8]) -> Self {
        let magic = bytes[0..6].try_into().unwrap();
        let payload_size = u32::from_le_bytes(bytes[6..10].try_into().unwrap());
        let msg_type = u32::from_le_bytes(bytes[10..14].try_into().unwrap());
        Header {
            magic,
            payload_size,
            msg_type,
        }
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut buffer = Vec::new();
        buffer.extend_from_slice(&self.magic);
        buffer.extend_from_slice(&self.payload_size.to_le_bytes());
        buffer.extend_from_slice(&self.msg_type.to_le_bytes());
        buffer
    }
}

fn pack(request_type: Request, payload: &str) -> Vec<u8> {
    let mut buffer = Vec::new();
    let header = Header::new(payload.len() as u32, request_type as u32);
    buffer.extend_from_slice(&header.to_bytes());
    buffer.extend_from_slice(payload.as_bytes());
    buffer
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_header() {
        let header = Header::new(10, 1);
        let bytes = header.to_bytes();
        let parsed_header = Header::from_bytes(&bytes);
        let parsed_bytes = parsed_header.to_bytes();
        assert_eq!(std::mem::size_of::<Header>(), bytes.len());
        assert_eq!(std::mem::size_of_val(&header), std::mem::size_of_val(&parsed_header));
        assert_eq!(bytes, parsed_bytes);
    }

    #[test]
    fn test_pack() {
        let payload = "test";
        let packed = pack(Request::Command, payload);
        assert_eq!(packed.len(), std::mem::size_of::<Header>() + payload.len());
        assert_eq!(&packed[0..6], b"i3-ipc");
        assert_eq!(packed[6..10], (payload.len() as u32).to_le_bytes());
        assert_eq!(packed[10..14], (Request::Command as u32).to_le_bytes());
        assert_eq!(&packed[14..], payload.as_bytes());
    }
}
