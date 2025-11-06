use std::error::Error;
use std::io::{self, Read, Write};

fn hello_by_read_write(source: &mut Vec<u8>) -> io::Result<Vec<u8>> {
    #[allow(unused_mut)]
    let mut glop: Vec<u8> = "hello world".as_bytes().to_vec();
    #[cfg(feature = "bad-utf8")]
    glop.push(0x80);
    source.write_all(&glop)?;
    #[cfg(feature = "gratuitous-flush")]
    source.flush()?;
    let mut sink = Vec::new();
    source.as_slice().read_to_end(&mut sink)?;
    Ok(sink)
}


fn do_the_thing() -> Result<(), Box<dyn Error>> {
    let mut source = Vec::new();
    let sink = hello_by_read_write(&mut source)?;
    assert_eq!(source, sink);
    println!("{}", str::from_utf8(&sink)?);
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    do_the_thing()
}
