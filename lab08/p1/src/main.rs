use core::cmp::Ordering;
use std::{collections::HashMap, fs, io};

#[derive(Debug)]
struct WordData {
    word: String,
    count: usize,
}

impl Ord for WordData {
    fn cmp(&self, other: &Self) -> Ordering {
        return self.count.cmp(&other.count);
    }
}

impl PartialOrd for WordData {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for WordData {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl Eq for WordData {}
fn main() -> Result<(), io::Error> {
    let mut maxlen = 0;
    let input = fs::read_to_string("src/input.txt")?;

    let mut map = HashMap::new();

    for word in input.split(|x: char| x.is_ascii_whitespace() || x.is_ascii_punctuation()) {
        if word.trim().len() != 0 {
            map.entry(word.to_ascii_lowercase())
                .and_modify(|x| *x += 1)
                .or_insert(1);
            if word.len() > maxlen {
                maxlen = word.len();
            }
        }
    }

    let mut vec = Vec::new();

    for word in map.into_iter() {
        vec.push(WordData {
            word: word.0,
            count: word.1,
        });
    }

    vec.sort_unstable();
    vec.reverse();

    for i in vec {
        print!("{}", i.word);
        let mut j = maxlen - i.word.len();
        while j != 0 {
            print!(" ");
            j -= 1;
        }
        println!(" => {}", i.count);
    }

    Ok(())
}
