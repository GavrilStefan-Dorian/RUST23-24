fn rot13(s: &String) -> String {
    let s1 = s.clone();
    let bytes_vec = s1.into_bytes();
    let mut snew = String::new();

    for it in bytes_vec {
        if (it + 13) as char >= 'a'
            && (it + 13) as char <= 'z'
            && it as char >= 'a'
            && it as char <= 'z'
        {
            snew.push((it + 13) as char);
        }
        if (it - 13) as char >= 'a'
            && (it - 13) as char <= 'z'
            && it as char >= 'a'
            && it as char <= 'z'
        {
            snew.push((it - 13) as char);
        }
        if (it + 13) as char >= 'A'
            && (it + 13) as char <= 'Z'
            && it as char >= 'A'
            && it as char <= 'Z'
        {
            snew.push((it + 13) as char);
        }
        if (it - 13) as char >= 'A'
            && (it - 13) as char <= 'Z'
            && it as char >= 'A'
            && it as char <= 'Z'
        {
            snew.push((it - 13) as char);
        }
        if it > 127 {
            panic!("Non-ASCII encountered!");
        }
    }
    return snew;
}

fn main() {
    let mut s1 = String::from("abcdEFGHijklMNOPqrstUVWXyz");
    s1 = rot13(&s1);
    println!("{s1}");
    let mut s2 = String::from("abcdEFGHi😊jklMNOPqrstUVWXyz");
    s2 = rot13(&s2);
    println!("{s2}");
}
