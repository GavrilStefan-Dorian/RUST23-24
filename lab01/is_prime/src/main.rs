fn is_prime(x: u32) -> bool {
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

fn main() {
    let mut x = 0u32;
    while x <= 100 {
        if is_prime(x) {
            println!("{}", x);
        }
        x = x + 1;
    }
}
