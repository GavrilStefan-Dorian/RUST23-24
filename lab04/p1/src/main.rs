use std::{fs, io};

fn main() -> Result<(), io::Error> {
    let s = fs::read_to_string("src/text.txt")?;
    let mut chars_max = 0;
    let mut bytes_max = 0;
    let mut s1 = String::new();
    let mut s2 = String::new();

    for lines in s.lines() {
        if bytes_max < lines.len() {
            bytes_max = lines.len();
            s1 = lines.to_string();
        }
        for c in lines.char_indices() {
            if chars_max < c.0 {
                chars_max = c.0;
                s2 = lines.to_string()
            }
        }
    }

    println!("{s1}\n{s2}");
    Ok(())
}
