use std::io;
use anyhow::Result;
use std::cell::RefCell;

struct Cache{
values: Vec<(RefCell<usize>,RefCell<bool>)>
}

impl Cache {
    fn new() -> Cache{
        let c = Cache { values: Vec::new()};
        c
    }

    fn push(&mut self,v: RefCell<usize>,p: RefCell<bool>){
        self.values.push((v,p));
    }
}

fn is_prime(x: usize) -> bool {
    let mut prime: bool = true;
    let mut d = 3;
    if x <= 1 || x != 2 && x % 2 == 0 {
        return false;
    }
    while d * d <= x {
        if x % d == 0 {
            prime = false;
            break;
        } else {
            d = d + 2;
        }
    }
    prime
}

fn main()->Result<()> {
    let mut number ;
    let mut cache = Cache::new();

    'outer:for line in io::stdin().lines(){
    number = 0;
    for digit in line.unwrap().chars(){
        if '0' <= digit && digit <= '9'{
            number = number*10 + digit as usize - '0' as usize;
        }
    }

    for el in &cache.values{
        if *el.0.borrow() == number{
            if *el.1.borrow() == true{
                println!("{} from cache is prime",number);
            }
            else {
                println!("{} from cache is not prime",number);
            }
        continue 'outer;
        }
    }

    if is_prime(number){
        println!("{} is prime",number);
        if cache.values.len() == 10{
            cache.values.remove(0);
        }
        cache.push(RefCell::new(number),RefCell::new(true));
    }
    else {
        println!("{} is not prime",number);
        if cache.values.len() == 10{
            cache.values.remove(0);
        }
        cache.push(RefCell::new(number),RefCell::new(false));
    }
    }
    Ok(())
}
