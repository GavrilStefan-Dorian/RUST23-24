fn cmmdc(mut x: u32, mut y: u32) -> u32 {
    if y == 0 {
        return x;
    }
    let mut r = x % y;
    while r != 0 {
        x = y;
        y = r;
        r = x % y;
    }
    y
}

fn co_prime(x: u32, y: u32) -> bool {
    if cmmdc(x, y) == 1 {
        return true;
    }
    false
}

fn main() {
    let mut x = 0;
    let mut y;

    while x <= 100 {
        y = 0;
        while y <= 100 {
            if co_prime(x, y) {
                println!("{} and {} are coprime!", x, y);
            }
            y = y + 1;
        }
        x = x + 1;
    }
}
