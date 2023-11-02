use std::{io,fs};

fn rot13() -> Result<(),&'static str> {
    let s = fs::read_to_string("src/input.txt").unwrap();
    let mut result = String::with_capacity(s.len());
    let mut c1;
    for c in s.as_bytes() {
        c1=*c;
        if c1 > 127 {
            return Err("Non-ASCII encountered!");
        }
        match c1 {
            97..=122 => result.push((((c1 - 97 + 13) % 26) + 97 ) as char),
            65..=90 => result.push((((c1 - 65 + 13) % 26) + 65 ) as char),
            _ => result.push(c1 as char),
        }
    }
   Ok(())
}
fn main()->Result<(),io::Error> {
    match rot13(){
        Ok(())=> Ok(()),
        Err(e)=>{fs::write("src/output.txt",e)?;
                       Ok(())}
    }
}