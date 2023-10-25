fn checked_sum(x: u32, y: u32) -> u32 {
    let sum = x as u64 + y as u64;
    if sum > std::u32::MAX as u64 {
        panic!("Sum of x = {x} and y = {y} does not fit in a u32");
    }
    x + y
}

fn checked_mul(x: u32, y: u32) -> u32 {
    let mul = x as u64 * y as u64;
    if mul > std::u32::MAX as u64 {
        panic!("Mul of x = {x} and y = {y} does not fit in a u32");
    }
    x * y
}

fn main() {
    let x: u32 = std::u32::MAX;

    checked_sum(x, 0);
    checked_mul(x, 1);
    checked_sum(x, 1);
    checked_mul(x, 2);
}
