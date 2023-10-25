#[derive(Debug)]
enum MyError {
    NASCII,
    NDIGIT,
    NHEX,
    NLETTER,
    NPRINT,
}

fn to_uppercase(c: char) -> Result<char, MyError> {
    if c >= 'a' && c <= 'z' {
        return Ok((c as u8 + 'A' as u8 - 'a' as u8) as char);
    } else {
        if c >= 'A' && c <= 'Z' {
            return Ok(c);
        } else {
            return Err(MyError::NLETTER);
        }
    }
}

fn to_lowercase(c: char) -> Result<char, MyError> {
    if c >= 'A' && c <= 'Z' {
        return Ok((c as u8 + 'a' as u8 - 'A' as u8) as char);
    } else {
        if c >= 'a' && c <= 'z' {
            return Ok(c);
        } else {
            return Err(MyError::NLETTER);
        }
    }
}

fn print_char(c: char) -> Result<(), MyError> {
    if c as u8 > 31 && c as u8 != 127 {
        println!("{c} is printable!");
        return Ok(());
    } else {
        return Err(MyError::NPRINT);
    }
}

fn char_to_number(c: char) -> Result<u8, MyError> {
    if (c as u8) < 0 as u8 || (c as u8) > 127 as u8 {
        return Err(MyError::NASCII);
    } else {
        if c < '0' || c > '9' {
            return Err(MyError::NDIGIT);
        } else {
            return Ok(c as u8 - '0' as u8);
        }
    }
}

fn char_to_number_hex(c: char) -> Result<u8, MyError> {
    if (c as u8) < 0 as u8 || (c as u8) > 127 as u8 {
        return Err(MyError::NASCII);
    } else {
        if c >= '0' && c <= '9' {
            return Ok(c as u8 - '0' as u8);
        } else {
            if c >= 'A' && c <= 'F' {
                return Ok(c as u8 - 'A' as u8 + 10);
            } else {
                return Err(MyError::NHEX);
            }
        }
    }
}

fn print_error(e: MyError) {
    println!("{:?}", e);
}

fn main() {
    let my_name = "Gavril Ștefan-Dorian";
    let my_hex_number = 'A';
    let mut i = 0;

    while let Ok(()) = print_char(my_name.chars().nth(i).unwrap()) {
        i += 1;
    }

    i = 0;

    while i < 20 {
        match to_uppercase(my_name.chars().nth(i).unwrap()) {
            Ok(x) => println!("{x}"),
            Err(e) => print_error(e),
        }
        i += 1;
    }

    match to_lowercase(my_hex_number) {
        Ok(x) => println!("{x}"),
        Err(e) => print_error(e),
    }

    match char_to_number_hex(my_hex_number) {
        Ok(x) => println!("{x}"),
        Err(e) => print_error(e),
    }

    match char_to_number(my_hex_number) {
        Ok(x) => println!("{x}"),
        Err(e) => print_error(e),
    }
}
