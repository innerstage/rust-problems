/*static X:i8 = 42;

fn main() {
    for i in 0..=5 {
        println!("{}", X-i);
    }
}*/

static X:i8 = 42;

fn main() {
    let mut i= 0;
    while i <= 5 {
        println!("{}", X-i);
        i += 1;
    }
}