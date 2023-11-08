use anyhow::Result;
use std::fs;

struct Student<'a> {
    name: &'a str,
    phone: &'a str,
    age: usize,
}
fn main() -> Result<()> {
    let fd = fs::read_to_string("src/input.txt")?;
    let mut smax = Student {
        name: "",
        phone: "",
        age: 0,
    };
    let mut smin = Student {
        name: "",
        phone: "",
        age: 100,
    };
    let mut c;
    let mut s = Student {
        name: "",
        phone: "",
        age: 0,
    };
    for l in fd.lines() {
        c = 0;
        for e in l.split(",") {
            if c == 0 {
                s.name = e;
            }
            if c == 1 {
                s.phone = e;
            }
            if c == 2 {
                let mut nr = 0;
                for i in e.chars() {
                    nr = nr * 10 + i as usize - '0' as usize;
                }
                s.age = nr;
            }
            c += 1;
        }
        if s.age > smax.age {
            smax.name = s.name;
            smax.phone = s.phone;
            smax.age = s.age;
        }
        if s.age < smin.age {
            smin.name = s.name;
            smin.phone = s.phone;
            smin.age = s.age;
        }
    }
    println!("Oldest student is: {}", smax.age);
    println!("Youngest student is: {}", smin.age);
    Ok(())
}
