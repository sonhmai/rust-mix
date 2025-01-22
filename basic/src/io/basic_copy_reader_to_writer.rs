use std::io::{self, Read, Write, ErrorKind};

const DEFAULT_BUF_SIZE: usize = 8 * 1024; // 8 KB

/// Copy all bytes from reader to writer.
/// Can be used to copy data from File to TcpStream, Stdin to Vec<u8>, etc.
/// This is the impl of std::io::copy().
pub fn copy<R: ?Sized, W: ?Sized>(
    reader: &mut R,
    writer: &mut W
) -> io::Result<u64> where R: Read, W: Write {
    let mut buf = [0; DEFAULT_BUF_SIZE];
    let mut written = 0; // returned how many bytes written
    loop {
        let len = match reader.read(&mut buf) {
            Ok(0) => return Ok(written),
            Ok(len) => len,
            Err(ref e) if e.kind() == ErrorKind::Interrupted => continue,
            Err(e) => return Err(e)
        };
        writer.write_all(&buf[..len])?;
        written += len as u64;
    }
    // no method for closing reader and writer as they typically implement Drop
}