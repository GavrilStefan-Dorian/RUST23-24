use std::{fs, io};

fn main() -> Result<(), io::Error> {
    let mut s = fs::read_to_string("src/p3text.txt")?;
    let hardc_abbv = String::from("pentru pt\npentru ptr\ndomnul dl\ndomnul dna\n");
    let s1 = s.clone();

    for word in s1.split_whitespace() {
        for abbv_pair in hardc_abbv.lines() {
            let mut abbv = abbv_pair.split(" ");
            let pair0 = abbv.next().unwrap();
            let pair1 = abbv.next().unwrap();

            if word == pair1 {
                s = s.replace(word, pair0);
            }
        }
    }

    println!("{s}");

    Ok(())
}
