#[derive(Debug)]
enum MyError {
    SumOverflowu32,
    MulOverflowu32,
}

fn checked_sum(x: u32, y: u32) -> Result<u32, MyError> {
    let sum = x as u64 + y as u64;
    if sum > std::u32::MAX as u64 {
        return Err(MyError::SumOverflowu32);
    }
    Ok(x + y)
}

fn checked_mul(x: u32, y: u32) -> Result<u32, MyError> {
    let mul = x as u64 * y as u64;
    if mul > std::u32::MAX as u64 {
        return Err(MyError::MulOverflowu32);
    }
    Ok(x * y)
}

fn calc(x: u32, y: u32) -> Result<bool, MyError> {
    let m = checked_mul(x, y)?;
    let s = checked_sum(x, y)?;

    Ok(s != m)
}

fn calc2(x: u32, y: u32) -> Result<bool, MyError> {
    let s = checked_sum(x, y)?;
    let m = checked_mul(x, y)?;

    Ok(s != m)
}

fn main() {
    let x: u32 = std::u32::MAX;

    match calc(x, 2) {
        Ok(x) => println!("Value: {}", x),
        Err(e) => eprintln!("{:?}", e),
    }

    match calc2(x, 2) {
        Ok(x) => println!("Value: {}", x),
        Err(e) => eprintln!("{:?}", e),
    }

    match calc2(x, 0) {
        Ok(x) => println!("Value: {}", x),
        Err(e) => eprintln!("{:?}", e),
    }

    match calc(x, 0) {
        Ok(x) => println!("Value: {}", x),
        Err(e) => eprintln!("{:?}", e),
    }
}
