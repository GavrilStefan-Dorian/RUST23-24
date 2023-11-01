use std::{fs, io};

fn main() -> Result<(), io::Error> {
    let s = fs::read_to_string("src/hosts.txt")?;
    let mut count;
    let mut s1 = String::new();
    let mut s2 = String::new();

    for lines in s.lines() {
        if lines.starts_with("#") == false {
            count = 2;
            for col in lines.split_whitespace() {
                if count == 2 {
                    s2 = col.to_string();
                }
                if count == 1 {
                    s1 = col.to_string();
                }
                count -= 1;
            }
            if count <= 0 {
                println!("{s1} => {s2}");
            }
        }
    }

    Ok(())
}
