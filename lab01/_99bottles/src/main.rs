fn main() {
    let mut bottles = 99;
    while bottles != 0 {
        if bottles == 1 {
            println!(
                "{} bottle of beer on the wall,\n{} bottle of beer.",
                bottles, bottles
            );
        } else {
            println!(
                "{} bottles of beer on the wall,\n{} bottles of beer.",
                bottles, bottles
            );
        }
        bottles = bottles - 1;

        if bottles == 0 {
            println!("Take one down, pass it around,\nNo bottles of beer on the wall.\n");
        } else if bottles == 1 {
            println!(
                "Take one down, pass it around,\n{} bottle of beer on the wall.\n",
                bottles
            );
        } else {
            println!(
                "Take one down, pass it around,\n{} bottles of beer on the wall.\n",
                bottles
            );
        }
    }
}
