fn next_prime(x: u16) -> Option<u16> {
    let check = x as u32 + 1;

    if check > std::u16::MAX as u32 {
        return None;
    }

    let u: u16 = x + 1;

    if u <= 1 || u % 2 == 0 && u != 2 {
        return next_prime(u);
    }

    let mut d: u16 = 3;

    while d as u32 * d as u32 <= u as u32 {
        if u % d == 0 {
            return next_prime(u);
        } else {
            d += 2;
        }
    }

    Some(u)
}

fn main() {
    let mut n: u16 = 0;
    while let Some(i) = next_prime(n) {
        println!("Next prime from {} is {}", n, i);
        n += 1;
    }
}
