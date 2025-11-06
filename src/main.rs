use std::error::Error;
use std::io::{Read, Write, Result as IoResult};

fn hello_by_read_write(source: &mut Vec<u8>) -> IoResult<Vec<u8>> {
    let mut glop: Vec<u8> = "hello world".as_bytes().to_vec();
    glop.push(0x80);
    source.write_all(&glop)?;
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
