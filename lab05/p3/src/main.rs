use anyhow::Result;
use serde_derive::Deserialize;
use std::fs;

#[derive(Debug, Deserialize)]
struct Student {
    name: String,
    phone: String,
    age: u8,
}
fn main() -> Result<()> {
    let fd = fs::read_to_string("src/input.txt")?;

    let mut smax = Student {
        name: String::new(),
        phone: String::new(),
        age: 0,
    };
    let mut smin = Student {
        name: String::new(),
        phone: String::new(),
        age: 100,
    };

    for mut l in fd.lines() {
        l = l.trim();
        let s: Student = serde_json::from_str(&l)?;

        if s.age > smax.age {
            smax.name = s.name.clone();
            smax.phone = s.phone.clone();
            smax.age = s.age;
        }
        if s.age < smin.age {
            smin.name = s.name.clone();
            smin.phone = s.phone.clone();
            smin.age = s.age;
        }
    }
    println!("Oldest student is: {}", smax.name);
    println!("Youngest student is: {}", smin.name);
    Ok(())
}
