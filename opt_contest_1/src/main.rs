use std::{io,fs};

fn rot13(s:&[u8]) -> Result<(),&'static str> {
    let mut result = Vec::with_capacity(s.len());

    for &c in s {
        if c > 127 {
            return Err("Non-ASCII encountered!");
        }
        if c > 96 && c < 123 {
            if c + 13 > 122 {
                result.push(c-13);
            }
            else {
                result.push(c+13);
            }
        } 
        else {
            if c > 64 && c < 91 {
                if c + 13 > 90 {
                    result.push(c-13);
                }
                else {
                    result.push(c+13);
                }
            } 
              else {
                 result.push(c);
                   }
        }   
    }
   Ok(())
}
fn main()->Result<(),io::Error> {
    let s = fs::read("src/input.txt")?;
    match rot13(&s){
        Ok(())=> (),
        Err(e)=>{fs::write("src/output.txt",e)?;
            }
    }
    Ok(())
}